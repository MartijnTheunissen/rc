use NumType;

#[derive(Debug, PartialEq, Copy)]
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

#[derive(Debug, PartialEq, Copy)]
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
