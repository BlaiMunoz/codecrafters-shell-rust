use std::env;
use std::io::{self, Write};
use std::process::Command;
use std::str::FromStr;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

enum Commands {
    Echo,
    Type,
    Pwd,
    Cd,
    Exit,
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Commands::Echo),
            "type" => Ok(Commands::Type),
            "pwd" => Ok(Commands::Pwd),
            "cd" => Ok(Commands::Cd),
            "exit" => Ok(Commands::Exit),
            _ => Err(()),
        }
    }
}

fn is_executable(path: &std::path::Path) -> bool {
    #[cfg(unix)]
    {
        std::fs::metadata(path)
            .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
}

fn find_in_path(cmd: &str) -> Option<std::path::PathBuf> {
    let path_var = env::var_os("PATH")?;
    for path in env::split_paths(&path_var) {
        let full_path = path.join(cmd);
        if is_executable(&full_path) { // Usamos la nueva comprobacion
            return Some(full_path);
        }
    }
    None
}

impl Commands {
    fn echo_cmd(parameters: &[&str]) {
        println!("{}", parameters.join(" "));
    }

    fn type_cmd(parameters: &[&str]) {
        for cmd in parameters {
            if Commands::from_str(cmd).is_ok() {
                println!("{} is a shell builtin", cmd);
            } else if let Some(path) = find_in_path(cmd) {
                println!("{} is {}", cmd, path.display());
            } else {
                println!("{}: not found", cmd);
            }
        }
    }

    fn pwd_cmd() {
        let path = env::current_dir();
        println!("{}", path.unwrap().display());
    }

    fn cd_cmd(parameters: &[&str]) {
        if parameters.is_empty() { return; }
    
        let raw_path = parameters[0];
    
        let root = if raw_path.starts_with("~") {
            let Some(home) = env::home_dir() else { return };
            
            match raw_path.strip_prefix("~/") {
                Some(rest) => home.join(rest),
                None => home,
            }
        } else {
            PathBuf::from(raw_path)
        };
    
        if env::set_current_dir(&root).is_err() {
            eprintln!("cd: {}: No such file or directory", raw_path);
        }
    }


    fn exit_cmd() {
        std::process::exit(0);
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() { continue; }

        let cmd_name = parts[0];
        let args = &parts[1..];

        // 1. Intentar ejecutar comando interno (builtin)
        if let Ok(builtin) = Commands::from_str(cmd_name) {
            match builtin {
                Commands::Echo => Commands::echo_cmd(args),
                Commands::Type => Commands::type_cmd(args),
                Commands::Pwd => Commands::pwd_cmd(),
                Commands::Cd => Commands::cd_cmd(args),
                Commands::Exit => Commands::exit_cmd(),
            }
        } 
        // 2. Intentar ejecutar comando externo (ej. ls, cat, git)
        else if let Some(_) = find_in_path(cmd_name) {
            let status = Command::new(cmd_name)
                .args(args)
                .status();

            if let Err(e) = status {
                eprintln!("Error ejecutando {}: {}", cmd_name, e);
            }
        } 
        // 3. Comando no encontrado
        else {
            println!("{}: command not found", cmd_name);
        }
    }
}