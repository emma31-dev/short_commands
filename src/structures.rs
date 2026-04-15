use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::{OpenOptions, Permissions, set_permissions};
use std::io::BufReader;
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    pub short_com: String,
    pub command: String,
    pub argument: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Commands {
    pub commands: Vec<Command>,
}

pub fn new_command(short: String, command: String, argument: String) -> Result<(), Box<dyn Error>> {
    let com = Command {
        short_com: short,
        command: command,
        argument: argument,
    };

    let permissions = Permissions::from_mode(0o755);
    set_permissions("./commands/data.json", permissions)?;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("./commands/data.json")?;
    let reader = BufReader::new(&file);

    let mut commands: Commands = serde_json::from_reader(reader)?;
    commands.commands.push(com);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_new_command() -> Result<(), Box<dyn Error>> {
        let result = new_command(
            "short".to_string(),
            "command".to_string(),
            "argument".to_string(),
        )?;
        assert_eq!(result, ());

        Ok(())
    }

    #[test]
    fn test_adding_new_command() -> Result<(), Box<dyn Error>> {
        let short = String::from("ts");
        let command = String::from("test command");
        let argument = String::from("test_argument");
        let result = new_command(short, command, argument)?;

        assert_eq!(result, ());

        Ok(())
    }
}
