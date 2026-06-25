use self::iter::Peekable;
use std::env;
use std::io::{self, Write};
use std::iter;
use std::path::Path;
use std::process::{Child, Command, Stdio};

pub fn read_input(prompt: &str) -> String {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut cmd = String::new();

    print!("{} ", prompt);
    stdout.flush().expect("Failed to flush the output");

    loop {
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

pub fn process_cmd<'a, I, J>(
    cmd: &str,
    args: I,
    prev_cmd: Option<Child>,
    commands: &mut Peekable<J>,
) -> Option<Child>
where
    I: Iterator<Item = &'a str>,
    J: Iterator<Item = &'a str>,
{
    let stdin = match prev_cmd {
        Some(mut child) => Stdio::from(child.stdout.take().expect("no stdout on previous child")),
        None => Stdio::inherit(),
    };

    let stdout = if commands.peek().is_some() {
        Stdio::piped()
    } else {
        Stdio::inherit()
    };

    match Command::new(cmd)
        .args(args)
        .stdin(stdin)
        .stdout(stdout)
        .spawn()
    {
        Ok(child) => Some(child),
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            None
        }
    }
}
