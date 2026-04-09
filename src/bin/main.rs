extern crate short_commands;

use short_commands::{
    create::create_new_command, get_commands, if_exists, run_command, structures::Command,
};
use std::env::args;

fn main() {
    let arg: Vec<String> = args().collect();
    let (argument, command) = if_exists(arg);

    if command == Some("new".to_string()) {
        create_new_command();
    } else {
        get_commands(command.unwrap_or_default());
    }
}
