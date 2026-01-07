#[allow(unused_imports)]
use std::str::FromStr;
use std::io::{self, Write};

enum Commands {
    Exit,
    Unknown,
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Commands, ()>{
        match s {
            "exit" => Ok(Commands::Exit),
            _ => Ok(Commands::Unknown),
        }
    }
}

fn main() {
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let Some(raw_input) = input.split_whitespace().next() else {
            continue;
        };

        // Convertimos el &str al enum Commands
        if let Ok(command) = Commands::from_str(raw_input) {
            match command {
                Commands::Exit => std::process::exit(0),
                Commands::Unknown => println!("{}: command not found", raw_input),
            }
        }
    }
}