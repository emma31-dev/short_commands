use std::process::{Command};

pub fn run_command(command: &String) { 
    if *command == "cc".to_string() {
        Command::new("ls")
            .current_dir("/Users/olaocha/Projects/short_commands")
            .status()
            .expect("unexpected error");
    } else {
        panic!("command not available");
    }
}
