use NumType;

#[derive(Debug, PartialEq)]
pub enum InfixOp {
    Add,
    Sub,
    Div,
    Mul
}

#[derive(Debug, PartialEq)]
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
