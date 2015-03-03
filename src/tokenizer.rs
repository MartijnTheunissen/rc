use NumType;

#[derive(Debug)]
enum InfixOperatorType {
    Add,
    Sub,
    Div,
    Mul
}

#[derive(Debug)]
enum Token {
    InfixOperator(InfixOperatorType),
    LParen,
    RParen,
    Assign,
    Num(NumType),
    Ident(String)
}

use std::str::FromStr;

#[derive(Debug)]
enum Error {
    UnexpectedChar(char),
    NumParseError(<NumType as FromStr>::Err)
}

use std::iter::Peekable;

fn get_num<T>(mut chars: &mut Peekable<T>) -> Result<NumType, Error>
        where T: Iterator<Item = char> {
    let mut string = String::new();
    while let Some(&c) = chars.peek() {
        match c {
            '0' ... '9' => {
                string.push(c);
                chars.next();
            }
            ' ' | ')' => {
                break;
            }
            c => return Err(Error::UnexpectedChar(c))
        }
    }
    match string.parse() {
        Ok(num) => Ok(num),
        Err(e) => Err(Error::NumParseError(e))
    }
}

fn get_ident<T>(mut chars: &mut Peekable<T>) -> Result<String, Error>
        where T: Iterator<Item = char> {
    let mut string = String::new();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | ')' => {
                break;
            }
            c if c.is_alphanumeric() || c == '_' => {
                string.push(c);
                chars.next();
            }
            c => return Err(Error::UnexpectedChar(c))
        }
    }
    Ok(string)
}

pub fn tokenize<T>(chars: T) -> Result<Vec<Token>, Error>
        where T: Iterator<Item = char> {
    let mut tokens = Vec::new();
    let mut chars = chars.peekable();

    while let Some(&c) = chars.peek() {
        use self::Token::*;
        use self::InfixOperatorType::*;

        match c {
            '+' => {
                tokens.push(InfixOperator(Add));
                chars.next();
            }
            // Either an infix - or a number with - prefix
            '-' => {
                chars.next();
                match chars.peek() {
                    Some(&c) => match c {
                        // Number with - prefix
                        '0' ... '9' => {
                            let n = try!(get_num(&mut chars));
                            tokens.push(Num(-n));
                        }
                        ' ' => {
                            tokens.push(InfixOperator(Sub));
                        }
                        c => return Err(Error::UnexpectedChar(c))
                    },
                    None => {
                        tokens.push(InfixOperator(Sub));
                    }
                }
            },
            '/' => {
                tokens.push(InfixOperator(Div));
                chars.next();
            }
            '*' => {
                 tokens.push(InfixOperator(Mul));
                 chars.next();
            }
            '(' => {
                tokens.push(LParen);
                chars.next();
            }
            ')' => {
                tokens.push(RParen);
                chars.next();
            }
            '=' => {
                tokens.push(Assign);
                chars.next();
            }
            '0' ... '9' => {
                tokens.push(Num(try!(get_num(&mut chars))));
            }
            ' ' => { chars.next(); }
            c if c == '_' || c.is_alphabetic() => {
                tokens.push(Ident(try!(get_ident(&mut chars))));
            }
            c => return Err(Error::UnexpectedChar(c))
        }
    }

    Ok(tokens)
}
