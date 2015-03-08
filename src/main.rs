#![feature(io, old_io)]

type NumType = f64;

mod calc;
mod tokens;
mod tokenizer;

use std::io::{self, BufReadExt};

#[cfg(not(unix))]
fn show_output(_expr: &str, string: &str) {
    println!("{}", string);
}

#[cfg(unix)]
fn show_output(expr: &str, string: &str) {
    if std::old_io::stdio::stdout_raw().isatty() {
        println!("{}", string);
    } else {
        extern crate libnotify;
        let notify = libnotify::Context::new("rc").unwrap();
        let n = notify.new_notification(&format!("{} = {}", expr, string),
                                        None, None).unwrap();
        n.show().unwrap();
    }
}

fn main() {
    let reader = io::stdin();
    let mut calc = calc::Calc::new();

    let input =
        std::env::args().skip(1).fold(String::new(), |a, b| a + " " + &b);

    if !input.is_empty() {
        let exprs = input.split(';');
        for expr in exprs {
            let result = calc.eval(&expr);
            let string = match result {
                Ok(num) => format!("{}", num),
                Err(e) => format!("{}", e)
            };
            show_output(expr, &string);
        }
        return;
    }

    for line_result in reader.lock().lines() {
        let text = line_result.unwrap();
        let text = text.trim();
        if !text.is_empty() {
            let expressions = text.split(';');
            for expr in expressions {
                calc.eval_print(expr);
            }
        }
    }
}
