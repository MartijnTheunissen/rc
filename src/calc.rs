use tokenizer;
use NumType;
use std::collections::HashMap;

pub struct Calc {
    vars: HashMap<String, NumType>
}

impl Calc {
    pub fn new() -> Calc {
        Calc {
            vars: HashMap::new()
        }
    }
    
    pub fn evaluate(&mut self, input: &str) -> NumType {
        let tokens = tokenizer::tokenize(input.chars());
        println!("{:?}", tokens);
        0.0
    }
}
