#![allow(dead_code)] // TODO: Remove once shit is used

#[cfg(test)]
mod test;

use Num;

enum LexState {
    /// Not inside any construct
    Outside,
    /// Inside a numeric literal
    NumLiteral,
    /// Inside an identifier
    Identifier,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    span: (usize, usize),
    kind: ErrorKind,
}

#[derive(Debug, PartialEq)]
enum ErrorKind {
    UnexpectedCharacter(char),
    NumParseError(::std::num::ParseFloatError),
}

/// Convert textual input into tokens.
pub fn lex(text: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();

    // Use a state machine for the lexing.
    use self::LexState::*;
    let mut state = Outside;
    let mut begin = 0;
    for (pos, c) in text.char_indices() {
        match state {
            Outside => {
                match c {
                    '0'...'9' => {
                        state = NumLiteral;
                        begin = pos;
                    }
                    _ if is_ident_start(c) => {
                        state = Identifier;
                        begin = pos;
                    }
                    _ => {
                        return Err(
                            Error {
                                span: (pos, pos),
                                kind: ErrorKind::UnexpectedCharacter(c),
                            }
                        )
                    }
                }
            }
            NumLiteral => {
                match c {
                    '0'...'9' | '.' => {}
                    _ if terminates_numliteral(c) => {
                        unimplemented!() // TODO: END OF TOKEN, implement, add test
                    }
                    _ => {
                        return Err(
                            Error {
                                span: (pos, pos),
                                kind: ErrorKind::UnexpectedCharacter(c),
                            }
                        )
                    }
                }
            }
            Identifier => {
                if !is_ident_continue(c) {
                    if terminates_ident(c) {
                        unimplemented!() // TODO: END OF TOKEN, implement, add test
                    } else {
                        return Err(
                            Error {
                                span: (pos, pos),
                                kind: ErrorKind::UnexpectedCharacter(c),
                            }
                        );
                    }
                }
            }
        }
    }
    // We reached the end, handle EOF. EOF can be a valid terminator for tokens.
    match state {
        Outside => {}
        NumLiteral => {
            let span = (begin, text.len());
            let num: f64 = match text[span.0..span.1].parse() {
                Ok(num) => num,
                Err(e) => {
                    return Err(
                        Error {
                            span: span,
                            kind: ErrorKind::NumParseError(e),
                        }
                    )
                }
            };
            tokens.push(
                Token {
                    span: span,
                    kind: TokenKind::NumLiteral(num),
                }
            );
        }
        Identifier => {
            tokens.push(
                Token {
                    span: (begin, text.len()),
                    kind: TokenKind::Identifier,
                }
            )
        }
    }
    Ok(tokens)
}

/// Whether this character is an identifier starting character
fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

/// Whether this character is an identifier continuing character
fn is_ident_continue(c: char) -> bool {
    is_ident_start(c) ||
        match c {
            '0'...'9' => true,
            _ => false,
        }
}

/// Whether this character terminates an identifier
fn terminates_ident(c: char) -> bool {
    // Prolly same as numliteral
    terminates_numliteral(c)
}

/// Whether this character terminates a numeric literal or not
fn terminates_numliteral(c: char) -> bool {
    c.is_whitespace() || is_operator(c)
}

/// Whether this character is an operator
fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '/' | '*' | '^' | '(' | ')' => true,
        _ => false,
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    span: (usize, usize),
    kind: TokenKind,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    NumLiteral(Num),
    Identifier,
}
