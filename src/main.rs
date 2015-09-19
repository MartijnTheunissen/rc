use std::ffi::CString;

extern crate readline;
extern crate libc;

type NumType = f64;

mod calc;
mod tokens;
mod tokenizer;

#[cfg(not(unix))]
fn show_output(_expr: &str, string: &str) {
    println!("{}", string);
}

#[cfg(unix)]
fn show_output(expr: &str, string: &str) {
    if unsafe { libc::isatty(libc::STDOUT_FILENO) } == 1 {
        println!("{}", string);
    } else {
        extern crate libnotify;
        let notify = libnotify::Context::new("rc").unwrap();
        let n = notify.new_notification(&format!("{} = {}", expr, string), None, None)
                      .unwrap();
        n.show().unwrap();
    }
}

fn main() {
    let mut calc = calc::Calc::new();

    let input = std::env::args().skip(1).fold(String::new(), |a, b| a + " " + &b);

    if !input.is_empty() {
        let exprs = input.split(';');
        for expr in exprs {
            let result = calc.eval(&expr);
            let string = match result {
                Ok(num) => format!("{}", num),
                Err(e) => format!("{}", e),
            };
            show_output(expr, &string);
        }
        return;
    }

    loop {
        match readline::readline_bare(&CString::new("> ").unwrap()) {
            Ok(line_bytes) => {
                let line = String::from_utf8_lossy(line_bytes.to_bytes());
                let text = line.trim();
                if !text.is_empty() {
                    let expressions = text.split(';');
                    for expr in expressions {
                        calc.eval_print(expr);
                    }
                    readline::add_history(&line_bytes);
                }
            }
            Err(_) => {
                // Just assume it's EOF, and break. What a pain.
                break;
            }
        }
    }
}
