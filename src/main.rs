use crate::fns::*;

mod fns;

fn main() {
    println!("Hello, world!");

    loop {
        let cmd = read_input("crabby🦀> ");
        let mut prev_cmd = None;

        let mut commands = cmd.trim().split(" | ").peekable();

        while let Some(command) = commands.next() {
            let mut cmd = command.trim().split_whitespace().into_iter().peekable();

            let command = cmd.next().expect("Unable to unwrap the command");
            let mut args = cmd;

            match command {
                "exit" => {
                    println!("EXITTING...");
                    return;
                }
                "cd" => {
                    handle_cd(args.next())
                        prev_cmd = None;
                },
                command => process_cmd(command, args, prev_cmd, &mut commands),
            }
        }
    }
}
