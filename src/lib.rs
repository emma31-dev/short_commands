use std::process::Command;

mod structures;

pub fn if_exists(mut arg: Vec<String>) -> (Option<String>, Option<String>, Option<String>) {
    if !arg[3].is_empty() {
        (
            Some(arg.swap_remove(3)),
            Some(arg[2].clone()),
            Some(arg[1].clone()),
        )
    } else if !arg[2].is_empty() {
        (Some(arg.swap_remove(2)), Some(arg[2].clone()), None)
    } else if !arg[1].is_empty() {
        (Some(arg.swap_remove(1)), None, None)
    } else {
        (None, None, None)
    }
}

pub fn run_command(
    command_arg2: Option<String>,
    command_arg1: Option<String>,
    command: Option<String>,
) {
    if command == Some("cc".to_string()) {
        Command::new("cargo")
            .arg("check")
            .current_dir("/Users/olaocha/Projects/short_commands")
            .status()
            .expect("unexpected error");
    } else {
        println!(
            "command not available for {} and {}",
            command_arg1.unwrap_or_default(),
            command_arg2.unwrap_or_default()
        );
    }
}
