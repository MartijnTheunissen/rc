use tokenizer;
use NumType;
use std::collections::HashMap;
use tokens::{Token, Operand, Operator};

pub struct Calc {
    vars: HashMap<String, NumType>
}

#[derive(Debug, PartialEq)]
enum Error {
    UndefinedVariable(String),
    SyntaxError(tokenizer::Error),
    UnexpectedToken(Token),
    Other(String)
}

impl Calc {
    pub fn new() -> Calc {
        Calc {
            vars: HashMap::new()
        }
    }

    pub fn eval_print(&mut self, input: &str) {
        match self.eval(input) {
            Ok(result) => println!("= {}", result),
            Err(e) => println!("{:?}", e)
        }
    }

    pub fn eval(&mut self, input: &str) -> Result<NumType, Error> {
        match tokenizer::tokenize(input.chars()) {
            Ok(tokens) => self.eval_tokens(tokens.into_iter()),
            Err(e) => Err(Error::SyntaxError(e))
        }
    }

    fn eval_tokens<T>(&mut self, tokens: T) -> Result<NumType, Error>
       where T: Iterator<Item = Token> {
        let mut operands: Vec<Operand> = Vec::new();
        let mut operators: Vec<Operator> = Vec::new();
        let assign_to: Option<String> = None;

        for token in tokens {
            print!("{:?} | ", token);
            match token {
                Token::Operand(o) => {
                    operands.push(o);
                }
                t => return Err(Error::UnexpectedToken(t))
            }
            println!("{:?} | {:?}", operands, operators);
        }

        // The last remaining operand in the stack is the answer
        let result = match operands.pop() {
            Some(tok) => match tok {
                Operand::Num(n) => Ok(n),
                Operand::Var(i) => self.lookup_var(i)
            },
            None => Err(Error::Other("No result? (stack empty)".to_string()))
        };

        if let Ok(num) = result {
            if let Some(ident) = assign_to {
                self.set_var(ident, num);
            }
        }

        result
    }

    fn set_var(&mut self, ident: String, num: NumType) {
        self.vars.insert(ident, num);
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

    macro_rules! test {
        ($name:ident, $input:expr, $expected:expr) => (
            #[test]
            fn $name() {
                let mut calc = Calc::new();
                assert_eq!(calc.eval($input), Ok($expected));
            }
        )
    }

    test!(test_self_yield, "2", 2.);
    test!(test_self_yield_parens, "(2)", 2.);

    #[test]
    fn test_assignment() {
        let mut calc = Calc::new();
        // over_9000 = 9000 + 1
        assert_eq!(calc.eval("over_9000 = 9000 + 1"), Ok(9001.));
        assert_eq!(calc.eval("over_9000"), Ok(9001.));
    }

    test!(test_simple_arith_2, "2 + 2", 4.);
    test!(test_simple_arith_n, "9 - 6 - 1", 2.);
    test!(test_precedence, "9 + 4 * 2", 17.);
    test!(test_simple_parens, "3 * (2 + 4)", 18.);
    test!(test_nested_parens, "3 + 4 + 2 * 7 * (3 + (2 + 4) * 2)", 217.);
}
