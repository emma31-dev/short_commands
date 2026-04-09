use std::env::current_dir;
use std::process::Command;

mod create;
mod structures;

pub fn if_exists(mut arg: Vec<String>) -> (Option<String>, Option<String>) {
    if !arg[2].is_empty() {
        (Some(arg.swap_remove(2)), Some(arg.swap_remove(1)))
    } else if !arg[1].is_empty() {
        (None, Some(arg.swap_remove(1)))
    } else {
        (None, None)
    }
}

#[allow(dead_code)]
pub fn get_commands(shortcommand: String) -> (Option<String>, Option<String>) {
    (None, None)
}

pub fn run_command(argument: Option<String>, command: Option<String>) {
    if command != None {
        let path = current_dir().unwrap_or_default();
        Command::new("cargo")
            .arg("check")
            .current_dir(path)
            .status()
            .expect("unexpected error");
    } else {
        panic!(
            "command not available for {} and {}",
            command.unwrap_or_default(),
            argument.unwrap_or_default()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_run_command() -> Result<(), ()> {
        let argument = Some("check".to_string());
        let command = Some("cargo".to_string());
        run_command(argument, command);

        Ok(())
    }
}
