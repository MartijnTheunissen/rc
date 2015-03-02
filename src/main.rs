#![feature(old_io)]

type NumType = f64;

mod eval;
mod tokenizer;

use std::old_io::stdio;

fn main() {
    let mut reader = stdio::stdin();

    for line_result in reader.lock().lines() {
        let text = line_result.unwrap();
        println!("= {}", eval::evaluate(text.trim()));
    }
}
