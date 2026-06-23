use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::process::{Command};

pub fn read_input(prompt: &str) -> String {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut cmd = String::new();

    print!("{} ", prompt);
    stdout.flush().expect("Failed to flush the output");

    loop {
        stdin.read_line(&mut cmd).expect("Failed to read input");

        let cmd = cmd.trim();
        
        if cmd.len() == 0 {
            continue;
        } else {
            return cmd.to_string();
        }
    }
}

pub fn process_input(input: String) -> Vec<String> {
    if input.len() == 0 {
        return Vec::new();
    } 
    input.split_whitespace().map(|s| s.to_string()).collect()
}

pub fn handle_cd(input: &[String]) {
    let new_dir = match input.get(1) {
        Some(dir) => dir,
        None => &".".to_string()
    };

    let full_path = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(full_path) {
        eprintln!("{}", e)
    }
}

pub fn process_cmd(input: &[String]) {
    let mut cmd = Command::new(&input[0])
        .args(&input[1..])
        .spawn();

    match &mut cmd {
            Ok(child) => {
                if let Err(e) = child.wait() {
                    eprintln!("Command ran, but failed to exit cleanly: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
            }
        };
        
    
}