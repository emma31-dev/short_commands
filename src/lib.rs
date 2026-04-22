pub mod create;
pub mod structures;

use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::process;
use structures::{Command, Commands};

pub fn get_from(mut arg: Vec<String>) -> Option<String> {
    Some(arg.swap_remove(1))
}

fn get_commands(shortcommand: Option<String>) -> Option<Command> {
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

pub fn run_command(shortcommand: Option<String>) {
    let command = get_commands(shortcommand).expect("could not find command");
    let path = current_dir().expect("couldn't get file path");
    process::Command::new(command.command)
        .arg(command.argument)
        .current_dir(path)
        .status()
        .expect("Failed to run command");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command() -> Result<(), ()> {
        let short_com = Some("cc".to_string());
        let _ = get_commands(short_com);

        Ok(())
    }
}
