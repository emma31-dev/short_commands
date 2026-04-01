extern crate short_commands;

use std::env;

use short_commands::run_command;

fn main() {
    println!("===============================");
    println!("   Welcome to short commands   ");
    println!("===============================");
    
    let arg: Vec<String> = env::args().collect();
    
    let command = &arg[1];
    
    println!("Short command running..");
    
    run_command(command);
}
