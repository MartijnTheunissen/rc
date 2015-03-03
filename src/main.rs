#![feature(old_io)]

type NumType = f64;

mod calc;
mod tokenizer;

use std::old_io::stdio;

fn main() {
    let mut reader = stdio::stdin();
    let mut calc = calc::Calc::new();

    for line_result in reader.lock().lines() {
        let text = line_result.unwrap();
        println!("= {}", calc.evaluate(text.trim()));
    }
}
