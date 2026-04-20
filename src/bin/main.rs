extern crate short_commands;

use short_commands::{create::create_new_command, if_exists, run_command};
use std::env::args;

fn main() {
    let arg: Vec<String> = args().collect();
    let mut command = if_exists(arg);

    match command.take() {
        Some(x) if x == "new".to_string() => {
            create_new_command().expect("Failed to create new command")
        }
        Some(x) if !x.is_empty() => run_command(command),
        Some(_) => println!("Cannot parse input"),
        None => println!("Cannot be null"),
    }
}
