#![feature(io)]

type NumType = f64;

mod calc;
mod tokens;
mod tokenizer;

use std::io::{self, BufReadExt};

fn main() {
    let reader = io::stdin();
    let mut calc = calc::Calc::new();

    let input =
        std::env::args().skip(1).fold(String::new(), |a, b| a + " " + &b);

    if !input.is_empty() {
        calc.eval_print(&input);
        return;
    }

    for line_result in reader.lock().lines() {
        let text = line_result.unwrap();
        calc.eval_print(text.trim());
    }
}
