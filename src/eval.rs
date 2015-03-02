use tokenizer;
use NumType;

pub fn evaluate(input: &str) -> NumType {
    let tokens = tokenizer::tokenize(input.chars());
    println!("{:?}", tokens);
    0.0
}