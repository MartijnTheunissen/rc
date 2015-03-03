use NumType;

#[derive(Debug)]
pub enum InfixOperatorType {
    Add,
    Sub,
    Div,
    Mul
}

#[derive(Debug)]
pub enum Token {
    InfixOperator(InfixOperatorType),
    LParen,
    RParen,
    Assign,
    Num(NumType),
    Ident(String)
}
