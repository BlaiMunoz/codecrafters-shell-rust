#[allow(unused_imports)]
use std::str::FromStr;
use std::io::{self, Write};

enum Commands {
    Echo,
    Type,
    Exit,
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Commands::Echo),
            "type" => Ok(Commands::Type),
            "exit" => Ok(Commands::Exit),
            _ => Err(()),
        }
    }
}

impl Commands {
    fn echo_cmd(message: String) {
        println!("{}", message);
    }

    fn type_cmd(command: &[&str]) {
        for cmd in command {
            match Commands::from_str(cmd) {
                Ok(_) => println!("{} is a shell builtin", cmd),
                Err(_) => println!("{}: not found", cmd),
            }
        }
    }

    fn exit_cmd() {
        std::process::exit(0)
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
                Commands::Echo => Commands::echo_cmd(parameters.join(" ")),
                Commands::Type => Commands::type_cmd(parameters),
                Commands::Exit => Commands::exit_cmd(),
            }
        }else{
            println!("{}: command not found ", command_raw);
        }
    }
}