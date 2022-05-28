use std::env;
use std::fs;
use std::io::Write;
use std::io;

mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_prompt();
    }
}

#[allow(dead_code)]
fn run_file(path: &str) {
    let _ = fs::read_to_string(path).expect("Error while reading the file");
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line.len() == 0 {
            println!("Bye!");
            break;
        }

				let mut scan = scanner::scanner::Scanner::new(&line);
				scan.scan_tokens();

				println!("{:?}", scan.tokens);
        println!("{}", line)
    }
}

#[allow(dead_code)]
fn run(_: &str) -> bool {
  return false;
}

#[allow(dead_code)]
fn error(line: i32, msg: &str) {
    report(line, "", msg);
}

fn report(line: i32, loc: &str, msg: &str) {
    println!("[line {}] Error {}: {}", line, loc, msg);
}
