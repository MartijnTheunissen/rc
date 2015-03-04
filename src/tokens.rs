use NumType;

#[derive(Debug)]
pub enum InfixOp {
    Add,
    Sub,
    Div,
    Mul
}

#[derive(Debug)]
pub enum Token {
    Infix(InfixOp),
    LParen,
    RParen,
    Assign,
    Num(NumType),
    Ident(String)
}
