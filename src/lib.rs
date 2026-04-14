use std::env::current_dir;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::process;
use structures::{Command, Commands};
pub mod create;
pub mod structures;

pub fn if_exists(mut arg: Vec<String>) -> Option<String> {
    Some(arg.swap_remove(1))
}

pub fn get_commands(shortcommand: Result<String, Box<dyn Error>>) -> Option<Command> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("./commands/data.json")
        .expect("Failed");
    let reader = BufReader::new(&file);

    let got_commands: Commands = serde_json::from_reader(reader).expect("FAILED");
    let sc = shortcommand.as_ref().expect("Enter valid string");
    let fetched_command = got_commands
        .commands
        .into_iter()
        .find(|c| c.short_com == sc.as_str());

    fetched_command
}

pub fn run_command(argument: Option<String>, command: Option<String>) {
    if command != None {
        let path = current_dir().unwrap_or_default();
        process::Command::new("cargo")
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

    #[test]
    fn test_run_command() -> Result<(), ()> {
        let argument = Some("check".to_string());
        let command = Some("cargo".to_string());
        run_command(argument, command);

        Ok(())
    }
}
