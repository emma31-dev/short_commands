extern crate short_commands;

use short_commands::{create::create_new_command, if_exists, run_command};
use std::env::args;

fn main() {
    let arg: Vec<String> = args().collect();
    let command = if_exists(arg);

    match command.clone() {
        Some(x) if x == "new".to_string() => {
            create_new_command().expect("Failed to create new command")
        }
        Some(_) => run_command(command),
        None => println!("Cannot be null"),
    }
}
