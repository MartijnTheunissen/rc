use NumType;

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mul,
    LParen,
    RParen
}

#[derive(Debug)]
pub enum Operand {
    Num(NumType),
    Var(String)
}

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    Operand(Operand),
    Assign
}
