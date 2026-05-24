use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    if let Err(error) = run() {
        eprintln!("mdp: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);

    let markdown = match args.next() {
        Some(path) => {
            if args.next().is_some() {
                return Err("expected at most one input file".to_string());
            }

            fs::read_to_string(&path).map_err(|error| format!("failed to read {path}: {error}"))?
        }
        None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .map_err(|error| format!("failed to read stdin: {error}"))?;
            input
        }
    };

    println!("{}", mdp::parse(&markdown));
    Ok(())
}
