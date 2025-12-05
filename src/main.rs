//! VibeSQL CLI - A SQL parser and semantic analyzer.
//!
//! This binary provides a command-line interface for parsing and analyzing SQL.

use std::io::{self, BufRead, Write};
use vibesql::{Error, Parser};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        // Parse SQL from command line argument
        let sql = args[1..].join(" ");
        parse_and_print(&sql);
    } else {
        // Interactive REPL mode
        run_repl();
    }
}

fn run_repl() {
    println!("VibeSQL Parser v{}", env!("CARGO_PKG_VERSION"));
    println!("Enter SQL statements (Ctrl+D to exit):\n");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("sql> ");
        stdout.flush().unwrap();

        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let sql = input.trim();
                if sql.is_empty() {
                    continue;
                }
                if sql.eq_ignore_ascii_case("quit") || sql.eq_ignore_ascii_case("exit") {
                    break;
                }
                parse_and_print(sql);
                println!();
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

fn parse_and_print(sql: &str) {
    let mut parser = Parser::new(sql);
    match parser.parse() {
        Ok(statements) => {
            println!("Parsed {} statement(s):", statements.len());
            for (i, stmt) in statements.iter().enumerate() {
                println!("  [{}] {:?}", i + 1, stmt);
            }
        }
        Err(e) => {
            print_error(sql, &e);
        }
    }
}

fn print_error(sql: &str, error: &Error) {
    eprintln!("Error: {}", error);
    if let Some(span) = error.span() {
        let line_start = sql[..span.start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let line_end = sql[span.start..]
            .find('\n')
            .map(|i| span.start + i)
            .unwrap_or(sql.len());
        let line = &sql[line_start..line_end];
        let col = span.start - line_start;

        eprintln!("  |");
        eprintln!("  | {}", line);
        eprintln!("  | {}^", " ".repeat(col));
    }
}
