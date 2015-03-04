use NumType;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mul,
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
