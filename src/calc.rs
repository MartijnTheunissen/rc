use tokenizer;
use NumType;
use std::collections::HashMap;
use tokens::Token;

pub struct Calc {
    vars: HashMap<String, NumType>
}

#[derive(Debug, PartialEq)]
enum Error {
    Unknown
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
                match self.eval_tokens(&tokens) {
                    Ok(result) => println!("= {}", result),
                    Err(e) => println!("Malformed expression: {:?}", e)
                }
            },
            Err(e) => println!("Syntax error: {:?}", e)
        }
    }

    fn eval_tokens(&mut self, tokens: &[Token]) -> Result<NumType, Error> {
        Ok(4.0)
    }
}

#[cfg(test)]
mod test {
    use super::Calc;
    use tokens::*;
    use tokens::Token::*;
    use tokens::InfixOperatorType::*;

    #[test]
    fn test_simple_addition() {
        let mut calc = Calc::new();
        assert_eq!(calc.eval_tokens(&[Num(2.), InfixOperator(Add), Num(2.)]),
                   Ok(4.0));
    }
}
