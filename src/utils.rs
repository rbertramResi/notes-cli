use dirs::home_dir;
use std::io;
use std::io::{ErrorKind};
use std::fs;
use gflags::custom::{Arg, Value};

pub fn throw_io_error(error_text: &str) -> Error {
    Error::IoError(io::Error::new(ErrorKind::Other, error_text))
}

pub fn get_base_path() -> String {
    match home_dir() {
        Some(d) => format!("{}/rob-utils/notes-cli", d.to_str().unwrap()),
        None => panic!("Unable to find OS home dir"),
    }
}

pub fn handle_missing_base() {
    println!("Base path does not exist. Adding path...");
    match fs::create_dir_all(get_base_path().as_str()) {
        Ok(_) => println!("path_created"),
        Err(e) => panic!("{}", format!("Failed to create path: {}", e)),
    }
}

#[derive(Debug)]
pub struct FlagText {
    pub value: String

}

impl Value for FlagText {
    fn parse(arg: Arg) -> gflags::custom::Result<Self> {
        match arg {
            s => Ok(FlagText { value: String::from(s.get_str())}),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
}
