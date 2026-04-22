extern crate short_commands;

use short_commands::create::create_new_command;
use short_commands::{get_from, run_command};
use std::env::args;

fn main() {
    let arg = args().collect();
    let command = get_from(arg);

    match command.clone() {
        Some(x) if x == "new".to_string() => {
            let _ = create_new_command();
        }
        Some(_) => run_command(command),
        None => println!("Cannot be null"),
    }
}
