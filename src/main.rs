#![feature(io, old_io)]

type NumType = f64;

mod calc;
mod tokens;
mod tokenizer;

use std::io::{self, BufReadExt};

#[cfg(not(unix))]
fn show_output(string: &str) {
    println!("{}", string);
}

#[cfg(unix)]
fn show_output(string: &str) {
    if std::old_io::stdio::stdout_raw().isatty() {
        println!("{}", string);
    } else {
        extern crate libnotify;
        let notify = libnotify::Context::new("rc").unwrap();
        let n = notify.new_notification("=", string).unwrap();
        n.show().unwrap();
    }
}

fn main() {
    let reader = io::stdin();
    let mut calc = calc::Calc::new();

    let input =
        std::env::args().skip(1).fold(String::new(), |a, b| a + " " + &b);

    if !input.is_empty() {
        let result = calc.eval(&input);
        let string = match result {
            Ok(num) => format!("{}", num),
            Err(e) => format!("{}", e)
        };
        show_output(&string);
        return;
    }

    for line_result in reader.lock().lines() {
        let text = line_result.unwrap();
        let text = text.trim();
        if !text.is_empty() {
            calc.eval_print(text);
        }
    }
}
