use super::utils;
use std::fs;

pub fn delete_file(file_name: &str) -> Result<(), utils::Error> {
    let base_path_and_file = utils::get_file_location(file_name);
    fs::remove_file(base_path_and_file)?;
    Ok(())
}
