extern crate short_commands;

use std::{env};

use short_commands::{run_command, if_exists};

fn main() {
    println!("===============================");
    println!("   Welcome to short commands   ");
    println!("===============================");
    
    let arg: Vec<String> = env::args().collect();
    
    let (command, command_arg1, command_arg2 ) = if_exists(arg);
    
    println!("Short command running..");
    
    run_command(command, command_arg1, command_arg2);
}
