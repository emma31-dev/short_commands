extern crate short_commands;

use short_commands::{create::create_new_command, get_commands, if_exists};
use std::env::args;

fn main() {
    let arg: Vec<String> = args().collect();
    let command = if_exists(arg);

    if command == Some("new".to_string()) {
        create_new_command().expect("Failed to create new command");
    } else {
        get_commands(command);
    }
}
