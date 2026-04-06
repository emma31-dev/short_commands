extern crate short_commands;

use std::env::args;

use short_commands::{if_exists, run_command};

fn main() {
    let arg: Vec<String> = args().collect();
    let (command_arg2, command_arg1, command) = if_exists(arg);

    run_command(command_arg2, command_arg1, command);
}
