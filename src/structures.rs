use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::{OpenOptions, Permissions, set_permissions};
use std::io::{BufReader, Seek, SeekFrom, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

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

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o755)
        .open("./commands/data.json")?;
    let reader = BufReader::new(&file);

    let mut commands: Commands = serde_json::from_reader(reader)?;
    commands.commands.push(com);

    let serialized = serde_json::to_string_pretty(&commands)?;
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_new_command() {
        let result = new_command(
            "short".to_string(),
            "command".to_string(),
            "argument".to_string(),
        );
        assert!(result.is_ok());
    }
}
