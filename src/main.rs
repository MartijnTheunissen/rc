#![feature(old_io)]

mod eval;

use std::old_io::stdio;

fn main() {
    let mut reader = stdio::stdin();

    for line_result in reader.lock().lines() {
        let text = line_result.unwrap();
        println!("= {}", eval::evaluate(text.trim()));
    }
}
