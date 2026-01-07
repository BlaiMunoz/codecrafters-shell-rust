#[allow(unused_imports)]
use std::io::{self, Write};

enum Commands {
    Unknown,
}

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    let mut command = String::new();
    print!("$ ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut command).unwrap();
    
    match command.trim() {
        _ => {
            println!("{}: command not found", command.trim());
        }
    }
}
