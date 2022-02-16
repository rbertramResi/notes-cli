use super::utils;
use std::fs;
use colored::*;

pub fn list_dirs() -> Result<(), utils::Error> {
    let base_path = utils::get_base_path();
    let paths = fs::read_dir(base_path)?;

    for path in paths {
        // todo gte rid of unwrapping
        println!("{}", path.unwrap().path().file_name().unwrap().to_str().unwrap().cyan().bold());
    }
    Ok(())
}
