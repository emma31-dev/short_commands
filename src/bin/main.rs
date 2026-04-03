extern crate short_commands;

use std::{env};

use short_commands::{run_command, if_exists};

fn main() {
    let arg: Vec<String> = env::args().collect();
    let (command, command_arg1, command_arg2 ) = if_exists(arg);
    
    run_command(command, command_arg1, command_arg2);
}
