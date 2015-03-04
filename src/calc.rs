use tokenizer;
use NumType;
use std::collections::HashMap;
use tokens::Token;

pub struct Calc {
    vars: HashMap<String, NumType>
}

#[derive(Debug, PartialEq)]
enum Error {
    Unknown,
    UndefinedVariable(String),
    Other(String)
}

impl Calc {
    pub fn new() -> Calc {
        Calc {
            vars: HashMap::new()
        }
    }

    pub fn eval_print(&mut self, input: &str) {
        match tokenizer::tokenize(input.chars()) {
            Ok(tokens) => {
                println!("{:?}", tokens);
                match self.eval_tokens(tokens.into_iter()) {
                    Ok(result) => println!("= {}", result),
                    Err(e) => println!("Malformed expression: {:?}", e)
                }
            },
            Err(e) => println!("Syntax error: {:?}", e)
        }
    }

    fn eval_tokens<T>(&mut self, tokens: T) -> Result<NumType, Error>
       where T: Iterator<Item = Token> {
        let mut operands: Vec<Token> = Vec::new();
        let mut operators: Vec<Token> = Vec::new();

        for token in tokens {
            print!("{:?} | ", token);
            match token {
                Token::Num(n) => {
                    operands.push(token);
                }
                _ => {}
            }
            println!("{:?} | {:?}", operands, operators);
        }

        // The last remaining number in the stack is the answer
        match operands.pop() {
            Some(tok) => match tok {
                Token::Num(n) => Ok(n),
                Token::Ident(i) => self.lookup_var(i),
                _ => Err(Error::Other(format!("Result is {:?}", tok)))
            },
            None => Err(Error::Other("No result? (stack empty)".to_string()))
        }
    }

    fn lookup_var(&self, ident: String) -> Result<NumType, Error> {
        match self.vars.get(&ident) {
            Some(v) => Ok(*v),
            None => Err(Error::UndefinedVariable(ident))
        }

    }
}

#[cfg(test)]
mod test {
    use super::Calc;
    use tokens::*;
    use tokens::Token::*;
    use tokens::InfixOperatorType::*;
    use tokens::Token::InfixOperator as I;

    #[test]
    fn test_self_yield() {
        let mut calc = Calc::new();
        // 2 = 2
        let t = vec![Num(2.)];
        assert_eq!(calc.eval_tokens(t.into_iter()), Ok(2.));
    }

    #[test]
    fn test_self_yield_parens() {
        let mut calc = Calc::new();
        // 2 = 2
        let t = vec![LParen, Num(2.), RParen];
        assert_eq!(calc.eval_tokens(t.into_iter()), Ok(2.));
    }

    #[test]
    fn test_assignment() {
        let mut calc = Calc::new();
        // over_9000 = 9000 + 1
        let t = vec![Ident("over_9000".to_string()), Assign, Num(9000.),
                     I(Add), Num(1.)];
        assert_eq!(calc.eval_tokens(t.into_iter()), Ok(9001.));
        let t = vec![Ident("over_9000".to_string())];
        assert_eq!(calc.eval_tokens(t.into_iter()), Ok(9001.));
    }

    #[test]
    fn test_simple_arith_2() {
        let mut calc = Calc::new();
        // 2 + 2 = 4
        let t = vec![Num(2.), I(Add), Num(2.)];
        assert_eq!(calc.eval_tokens(t.into_iter()), Ok(4.));
    }
    #[test]
    fn test_simple_arith_n() {
        let mut calc = Calc::new();
        // 9 - 6 - 1 = 2
        let t = vec![Num(9.), I(Sub), Num(6.), I(Sub), Num(1.)];
        assert_eq!(calc.eval_tokens(t.into_iter()), Ok(2.));
    }
    #[test]
    fn test_precedence() {
        let mut calc = Calc::new();
        // 9 + 4 * 2 = 17
        let t = vec![Num(9.), I(Add), Num(4.), I(Mul), Num(2.)];
        assert_eq!(calc.eval_tokens(t.into_iter()), Ok(17.));
    }
    #[test]
    fn test_simple_parens() {
        let mut calc = Calc::new();
        // 3 * (2 + 4) = 18
        let t = vec![Num(3.), I(Mul), LParen, Num(2.), I(Add), Num(4.),
                     RParen];
        assert_eq!(calc.eval_tokens(t.into_iter()), Ok(18.));
    }
    #[test]
    fn test_nested_parens() {
        let mut calc = Calc::new();
        // 3 + 4 + 2 * 7 * (3 + (2 + 4) * 2) = 217
        let t = vec![Num(3.), I(Add), Num(4.), I(Add), Num(2.), I(Mul),
                     Num(7.), I(Mul), LParen, Num(3.), I(Add), LParen, Num(2.),
                     I(Add), Num(4.), RParen, I(Mul), Num(2.), RParen];
        assert_eq!(calc.eval_tokens(t.into_iter()), Ok(217.));
    }
}
