use serde::{Deserialize, Serialize};
use serde_json;
use std::{error::Error, fs::OpenOptions, io::BufReader};

#[derive(Debug, Deserialize, Serialize)]
struct Command {
    short_com: String,
    command: String,
    argument: String,
}

pub fn new_command(short: String, command: String, argument: String) -> Result<(), Box<dyn Error>> {
    let com = Command {
        short_com: short,
        command: command,
        argument: argument,
    };

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("commands/data.json")?;
    let reader = BufReader::new(&file);

    let mut commands: Vec<Command> = serde_json::from_reader(reader)?;
    commands.push(com);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_new_command() -> Result<(), Box<dyn Error>> {
        let result = new_command(
            "short".to_string(),
            "command".to_string(),
            "argument".to_string(),
        )?;
        assert_eq!(result, ());

        Ok(())
    }
}
