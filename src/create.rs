use super::structures::new_command;
use std::error::Error;
use std::io::stdin;

pub fn create_new_command() -> Result<(), Box<dyn Error>> {
    println!(">>> Enter command:");
    let mut command = String::new();
    stdin().read_line(&mut command)?;

    println!(">>> Enter argument:");
    let mut argument = String::new();
    stdin().read_line(&mut argument)?;

    println!(">>> Enter short replacement:");
    let mut short_com = String::new();
    stdin().read_line(&mut short_com)?;

    new_command(short_com, command, argument)?;

    Ok(())
}
