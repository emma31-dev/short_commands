extern crate short_commands;

use short_commands::{if_exists, run_command, structures::Command};
use std::env::args;

fn main() {
    let arg: Vec<String> = args().collect();
    let (argument, command) = if_exists(arg);

    if command == Some("new".to_string()) {
        let new_command = (command, argument);
        new_command.add()
    } else {
        run_command();
    }
}
