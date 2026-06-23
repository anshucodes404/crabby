
use crate::fns::*;

mod fns;


fn main() {
    println!("Hello, world!");

    loop{
        let cmd = read_input("crabby🦀> ");

        let cmd: Vec<String> = process_input(cmd);

        match cmd.first().map(|s| s.as_str()) {
            Some("exit") => break,
            Some("cd") => handle_cd(&cmd),
            Some(_) => process_cmd(&cmd),
            _ => println!("Unknown Command")
            
        }

    }
        println!("EXITTING CRABBY...");
}
