extern crate atty;
extern crate librc;
extern crate linefeed;

use linefeed::Reader;

fn show_output(expr: &str, string: &str) {
    if atty::is() {
        println!("{}", string);
    } else {
        extern crate notify_rust;
        notify_rust::Notification::new()
            .summary(&format!("{} = {}", expr, string))
            .show()
            .unwrap();
    }
}

fn main() {
    let mut calc = librc::calc::Calc::new();

    let input = std::env::args()
        .skip(1)
        .fold(String::new(), |a, b| a + " " + &b);

    if !input.is_empty() {
        let exprs = input.split(';');
        for expr in exprs {
            let result = calc.eval(expr);
            let string = match result {
                Ok(num) => format!("{}", num),
                Err(e) => format!("{}", e),
            };
            show_output(expr, &string);
        }
        return;
    }

    let mut reader = Reader::new("rc").unwrap();
    reader.set_prompt("> ");

    loop {
        match reader.read_line() {
            Ok(Some(line)) => {
                let text = line.trim();
                if !text.is_empty() {
                    let expressions = text.split(';');
                    for expr in expressions {
                        calc.eval_print(expr);
                    }
                    reader.add_history(text.into());
                }
            }
            Ok(None) => break,
            Err(e) => match e.kind() {
                std::io::ErrorKind::Interrupted => break,
                _ => panic!("I/O error: {}", e),
            },
        }
    }
}
