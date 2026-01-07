#[allow(unused_imports)]
use std::io::{self, Write};

// enum Commands {
//     Unknown,
// }

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {

        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let Some(command) = input.split_whitespace().next() else {
            continue;
        };

        match command.trim() {
            _ => {
                println!("{}: command not found", command.trim());
            }
        }
    }

}
