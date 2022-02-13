use super::utils;
use std::fs;

pub fn read_file(file_name: &str) -> Result<(), utils::Error> {
    let read_path = utils::get_file_location(file_name);
    let file_content = fs::read_to_string(read_path).expect("Failed to read file");
    println!("{:?}", file_content);
    Ok(())
}

