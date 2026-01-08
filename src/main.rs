#[allow(unused_imports)]
use std::str::FromStr;
use std::io::{self, Write};

enum Commands {
    Echo,
    Exit,
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Commands::Echo),
            "exit" => Ok(Commands::Exit),
            _ => Err(()),
        }
    }
}

fn main() {
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        
        let resultado: Vec<&str> = input.split_whitespace().collect();
        if resultado.len() == 0 {
            continue;
        };
        
        let command_raw = &resultado[0];
        let parameters = &resultado[1..];

        // Convertimos el &str al enum Commands
        if let Ok(command) = Commands::from_str(command_raw) {
            match command {
                Commands::Echo => println!("{}", parameters.join(" ")),
                Commands::Exit => std::process::exit(0),
            }
        }else{
            println!("{}: command not found ", command_raw);
        }
    }
}