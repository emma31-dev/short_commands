use serde::Deserialize;
use serde_json::{Map, Value};
use std::{fs::File, io::BufReader};

#[derive(Deserialize, Debug)]
pub struct Command {
    command: String,
    argument: String,
}

pub fn new_command(
    short: String,
    command: String,
    argument: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = Value::Object({
        let mut map = Map::new();
        map.insert(
            short,
            Value::Object({
                let mut map = Map::new();
                map.insert("command".to_string(), Value::String(command));
                map.insert("argument".to_string(), Value::String(argument));

                map
            }),
        );

        map
    });
    let json_data = serde_json::to_string_pretty(&data);
    let file = File::open("commands/data.json")?;
    let reader = BufReader::new(file);

    let commands = serde_json::from_reader(reader)?;

    Ok(())
}
