use super::structures::new_command;
use std::{error::Error, io::stdin};

pub fn create_new_command() {
    println!("Enter command");
    let mut command = String::new();
    stdin()
        .read_line(&mut command)
        .expect("unable to get input");

    println!("Enter argument");
    let mut argument = String::new();
    stdin()
        .read_line(&mut argument)
        .expect("unable to get input");

    println!("Enter short replacement");
    let mut short_com = String::new();
    stdin()
        .read_line(&mut short_com)
        .expect("unable to get input");

    match new_command(short_com, command, argument) {
        Ok(()) => print!("good!"),
        Err() => panic!("bad!"),
    }
}
