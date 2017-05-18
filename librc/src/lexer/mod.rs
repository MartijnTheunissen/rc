#[cfg(test)]
mod test;

use Num;

#[derive(Debug, PartialEq, Clone, Copy)]
enum LexState {
    /// Not inside any construct
    Outside,
    /// Inside a numeric literal
    NumLiteral,
    /// Inside an identifier
    Identifier,
    /// Inside an operator
    Operator,
    /// Reached end of stream
    End,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    span: (usize, usize),
    kind: ErrorKind,
    state: LexState,
}

#[derive(Debug, PartialEq)]
enum ErrorKind {
    UnexpectedCharacter(char),
    NumParseError(::std::num::ParseFloatError),
    NumLitExtraPeriod,
    UnknownOperator,
}

struct Lexer<'a> {
    /// Char indices for the text we are lexing.
    char_indices: ::std::iter::Peekable<::std::str::CharIndices<'a>>,
    /// The lex state.
    state: LexState,
    /// The position of the beginning of the current token we are parsing, if any.
    token_begin: usize,
    /// The tokens acquired from the text
    tokens: Vec<Token>,
    /// The text we are lexing
    text: &'a str,
    /// Whether we already encountered a '.' in a numeric literal
    numlit_encountered_period: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            char_indices: text.char_indices().peekable(),
            state: LexState::Outside,
            token_begin: 0,
            tokens: Vec::new(),
            text: text,
            numlit_encountered_period: false,
        }
    }
    pub fn lex(mut self) -> Result<Vec<Token>, Error> {
        loop {
            match self.state {
                LexState::End => return Ok(self.tokens),
                LexState::Identifier => self.handle_identifier()?,
                LexState::NumLiteral => self.handle_numliteral()?,
                LexState::Operator => self.handle_operator()?,
                LexState::Outside => self.handle_outside()?,
            }
        }
    }
    /// Handle when the current state ends.
    ///
    /// It usually means we finished extracting a token, so we should add it to our list.
    fn handle_state_end(&mut self, end_pos: usize) -> Result<(), Error> {
        match self.state {
            // This shouldn't happen, since we shouldn't reach end again
            // after reaching the end.
            LexState::End => {}
            LexState::Identifier => {
                self.tokens
                    .push(
                        Token {
                            kind: TokenKind::Identifier,
                            span: (self.token_begin, end_pos),
                        }
                    );
            }
            LexState::NumLiteral => {
                let text = &self.text[self.token_begin..end_pos];
                eprintln!("Parsing {:?} for num", text);
                let value = match text.parse() {
                    Ok(value) => value,
                    Err(e) => {
                        return Err(
                            Error {
                                span: (self.token_begin, end_pos),
                                kind: ErrorKind::NumParseError(e),
                                state: self.state,
                            }
                        )
                    }
                };
                self.tokens
                    .push(
                        Token {
                            kind: TokenKind::NumLiteral(value),
                            span: (self.token_begin, end_pos),
                        }
                    )
            }
            LexState::Operator => {
                let kind = match &self.text[self.token_begin..end_pos] {
                    "+" => TokenKind::Plus,
                    "-" => TokenKind::Minus,
                    "*" => TokenKind::Asterisk,
                    "/" => TokenKind::ForwardSlash,
                    "(" => TokenKind::LParen,
                    ")" => TokenKind::Rparen,
                    "^" => TokenKind::Caret,
                    "," => TokenKind::Comma,
                    "=" => TokenKind::EqualsSign,
                    _ => {
                        return Err(
                            Error {
                                span: (self.token_begin, end_pos),
                                kind: ErrorKind::UnknownOperator,
                                state: self.state,
                            }
                        )
                    }
                };
                self.tokens
                    .push(
                        Token {
                            kind: kind,
                            span: (self.token_begin, end_pos),
                        }
                    )
            }
            _ => unimplemented!(),
        }
        Ok(())
    }
    fn handle_identifier(&mut self) -> Result<(), Error> {
        match self.char_indices.peek() {
            Some(&(i, ch)) => {
                if !is_ident_continue(ch) {
                    self.handle_state_end(i)?;
                    self.state = LexState::Outside;
                } else {
                    self.consume_char();
                }
            }
            None => {
                let len = self.text.len();
                self.handle_state_end(len)?;
                self.state = LexState::End;
            }
        }
        Ok(())
    }
    fn handle_numliteral(&mut self) -> Result<(), Error> {
        match self.char_indices.peek() {
            Some(&(i, ch)) => {
                if ch >= '0' && ch <= '9' {
                    self.consume_char();
                } else if ch == '.' {
                    if self.numlit_encountered_period {
                        return Err(
                            Error {
                                span: (i, i),
                                kind: ErrorKind::NumLitExtraPeriod,
                                state: self.state,
                            }
                        );
                    }
                    self.consume_char();
                } else {
                    self.handle_state_end(i)?;
                    self.state = LexState::Outside;
                }
            }
            None => {
                let len = self.text.len();
                self.handle_state_end(len)?;
                self.state = LexState::End;
            }
        }
        Ok(())
    }
    fn handle_operator(&mut self) -> Result<(), Error> {
        match self.char_indices.peek() {
            Some(&(i, ch)) => {
                if ch == '>' {
                    self.consume_char();
                } else {
                    self.handle_state_end(i)?;
                    self.state = LexState::Outside;
                }
            }
            None => {
                let len = self.text.len();
                self.handle_state_end(len)?;
                self.state = LexState::End;
            }
        }
        Ok(())
    }
    fn handle_outside(&mut self) -> Result<(), Error> {
        let (new_state, begin) = self.determine_next_lex_state()?;
        self.state = new_state;
        if self.state != LexState::Outside {
            self.token_begin = begin;
        }
        Ok(())
    }
    fn consume_char(&mut self) {
        self.char_indices.next();
    }
    fn determine_next_lex_state(&mut self) -> Result<(LexState, usize), Error> {
        let (i, ch) = match self.char_indices.next() {
            Some(ind) => ind,
            None => return Ok((LexState::End, 0)),
        };
        if is_ident_start(ch) {
            Ok((LexState::Identifier, i))
        } else if is_operator_start(ch) {
            Ok((LexState::Operator, i))
        } else if ch >= '0' && ch <= '9' {
            Ok((LexState::NumLiteral, i))
        } else if ch.is_whitespace() {
            Ok((LexState::Outside, i))
        } else {
            Err(
                Error {
                    kind: ErrorKind::UnexpectedCharacter(ch),
                    span: (self.token_begin, i),
                    state: self.state,
                }
            )
        }
    }
}

/// Convert textual input into tokens.
pub fn lex(text: &str) -> Result<Vec<Token>, Error> {
    Lexer::new(text).lex()
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

fn is_operator_start(c: char) -> bool {
    match c {
        '+' | '-' | '/' | '*' | '^' | '(' | ')' | ',' | '=' => true,
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
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    Caret,
    LParen,
    Rparen,
    Comma,
    EqualsSign,
}
