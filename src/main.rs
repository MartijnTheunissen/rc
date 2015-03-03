#![feature(old_io)]

type NumType = f64;

mod calc;
mod tokenizer;

use std::old_io::stdio;

fn main() {
    let mut reader = stdio::stdin();
    let mut calc = calc::Calc::new();

    let input =
        std::env::args().skip(1).fold(String::new(), |a, b| a + " " + &b);

    if !input.is_empty() {
        println!("= {}", calc.evaluate(&input));
        return;
    }

    for line_result in reader.lock().lines() {
        let text = line_result.unwrap();
        println!("= {}", calc.evaluate(text.trim()));
    }
}
