use NumType;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InfixOp {
    Add,
    Sub,
    Div,
    Mul
}

impl InfixOp {
    pub fn precedence(&self) -> u8 {
        use self::InfixOp::*;
        match *self {
            Add | Sub => 1,
            Div | Mul => 2
        }
    }
}

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::InfixOp::*;
        let c = match *self {
            Add => '+',
            Sub => '-',
            Div => '/',
            Mul => '*'
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Infix(InfixOp),
    LParen,
    RParen
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Num(NumType),
    Var(String)
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(Operator),
    Operand(Operand),
    Assign
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            _ => unimplemented!()
        }
    }
}
