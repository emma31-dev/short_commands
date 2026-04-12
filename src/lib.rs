use std::env::current_dir;
use std::error::Error;
use std::fs::{OpenOptions};
use std::io::BufReader;
use std::process;
use structures::{Command, Commands};
use std::slice::SliceIndex;
pub mod create;
pub mod structures;

pub fn if_exists(mut arg: Vec<String>) -> Option<String> {
    Some(arg.swap_remove(1))
}

fn query<T>(sc: T) -> Command
    where 
        T: SliceIndex<[Command]>
    {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("commands/data.json").expect("Failed");
        let reader = BufReader::new(&file);
    
        let commands: Commands = serde_json::from_reader(reader).expect("FAILED");
        let fetcthed_command = commands.commands.get(sc).expect("couldn't be found");
        
        fetcthed_command.unwrap_or_default()
}

#[allow(dead_code)]
pub fn get_commands(shortcommand: Option<String>) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn Error>> {
    let sc: String = shortcommand.unwrap_or_default();
    let command = query(sc);
    
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

    fn test_run_command() -> Result<(), ()> {
        let argument = Some("check".to_string());
        let command = Some("cargo".to_string());
        run_command(argument, command);

        Ok(())
    }
}
