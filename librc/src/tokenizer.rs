use NumType;
use std::fmt;
use std::str::FromStr;
use tokens::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedChar(char),
    NumParseError(<NumType as FromStr>::Err),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::Error::*;

        match *self {
            UnexpectedChar(c) => write!(f, "Unexpected character: '{}'", c),
            NumParseError(ref err) => write!(f, "Invalid number: {}", err),
        }
    }
}

use std::iter::Peekable;

fn is_separator(c: char) -> bool {
    match c {
        ' ' | ')' | '=' => true,
        c if ::tokens::InfixOp::from_char(c).is_some() => true,
        _ => false,
    }
}

fn get_num<T>(mut chars: &mut Peekable<T>) -> Result<NumType, Error>
where
    T: Iterator<Item = char>,
{
    let mut string = String::new();
    while let Some(&c) = chars.peek() {
        if is_separator(c) {
            break;
        } else {
            string.push(c);
            chars.next();
        }
    }
    match string.parse() {
        Ok(num) => Ok(num),
        Err(e) => Err(Error::NumParseError(e)),
    }
}

fn get_ident<T>(mut chars: &mut Peekable<T>) -> Result<String, Error>
where
    T: Iterator<Item = char>,
{
    let mut string = String::new();
    while let Some(&c) = chars.peek() {
        if is_separator(c) {
            break;
        } else if c.is_alphanumeric() || c == '_' {
            string.push(c);
            chars.next();
        } else {
            return Err(Error::UnexpectedChar(c));
        }
    }
    Ok(string)
}

pub fn tokenize<T>(chars: T) -> Result<Vec<Token>, Error>
where
    T: Iterator<Item = char>,
{
    let mut tokens = Vec::new();
    let mut chars = chars.peekable();

    while let Some(&c) = chars.peek() {
        use tokens::Token::*;
        use tokens::Operator::*;
        use tokens::Operand::*;
        use tokens::InfixOp::*;

        match c {
            // Either an infix - or a number with - prefix
            '-' => {
                chars.next();
                match chars.peek() {
                    Some(&c) => {
                        match c {
                            // Number with - prefix
                            '0'...'9' => {
                                let n = try!(get_num(&mut chars));
                                tokens.push(Operand(Num(-n)));
                            }
                            ' ' => {
                                tokens.push(Operator(Infix(Sub)));
                            }
                            c => return Err(Error::UnexpectedChar(c)),
                        }
                    }
                    None => {
                        tokens.push(Operator(Infix(Sub)));
                    }
                }
            }
            '(' => {
                tokens.push(Operator(LParen));
                chars.next();
            }
            ')' => {
                tokens.push(Operator(RParen));
                chars.next();
            }
            '=' => {
                tokens.push(Assign);
                chars.next();
            }
            '0'...'9' => {
                tokens.push(Operand(Num(try!(get_num(&mut chars)))));
            }
            ' ' => {
                chars.next();
            }
            c if c == '_' || c.is_alphabetic() => {
                tokens.push(Operand(Var(try!(get_ident(&mut chars)))));
            }
            c => {
                if let Some(tok) = ::tokens::InfixOp::from_char(c) {
                    tokens.push(Operator(Infix(tok)));
                    chars.next();
                } else {
                    return Err(Error::UnexpectedChar(c));
                }
            }
        }
    }

    Ok(tokens)
}
