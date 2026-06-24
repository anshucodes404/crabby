use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio, Child};
use self::iter::Peekable;
use std::iter;

pub fn read_input(prompt: &str) -> String {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut cmd = String::new();

    print!("{} ", prompt);
    stdout.flush().expect("Failed to flush the output");

    loop {
        // PERF: clear before each read so empty-input retries don't keep appending
        cmd.clear();
        stdin.read_line(&mut cmd).expect("Failed to read input");

        let cmd = cmd.trim();

        if cmd.len() == 0 {
            continue;
        } else {
            return cmd.to_string();
        }
    }
}

pub fn handle_cd(directory: Option<&str>) {
    let new_dir = match directory {
        Some(dir) => dir,
        None => ".",
    };

    let full_path = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(full_path) {
        eprintln!("{}", e)
    }
}

pub fn process_cmd<'a, I, J>(cmd: &str, args: I, prev_cmd: Option<&str>, commands: &mut Peekable<J>)
where
    I: Iterator<Item = &'a str>,
    J: Iterator<Item = &'a str>,
{

    let stdin = prev_cmd.map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));

    

    let mut cmd = Command::new(cmd).args(args).stdin(stdin).spawn();

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
