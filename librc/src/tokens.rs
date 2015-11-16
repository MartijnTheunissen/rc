use NumType;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InfixOp {
    Add,
    Sub,
    Div,
    Mul,
    Pow,
}

impl InfixOp {
    pub fn from_char(c: char) -> Option<InfixOp> {
        use self::InfixOp::*;
        Some(match c {
            '+' => Add,
            '-' => Sub,
            '/' => Div,
            '*' => Mul,
            '^' => Pow,
            _ => return None,
        })
    }
}

impl Into<char> for InfixOp {
    fn into(self) -> char {
        use self::InfixOp::*;
        match self {
            Add => '+',
            Sub => '-',
            Div => '/',
            Mul => '*',
            Pow => '^',
        }
    }
}

impl InfixOp {
    pub fn precedence(&self) -> u8 {
        use self::InfixOp::*;
        match *self {
            Add | Sub => 1,
            Div | Mul | Pow => 2,
        }
    }
}

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", Into::<char>::into(*self))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Infix(InfixOp),
    LParen,
    RParen,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Num(NumType),
    Var(String),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(Operator),
    Operand(Operand),
    Assign,
}
