use super::utils;
use std::fs;

gflags::define! {
    -t, --title: utils::FlagText
}

pub fn delete_file() -> Result<(), utils::Error> {
    gflags::parse();
    let file_name = &TITLE.flag.value;
    let base_path = utils::get_base_path();
    let base_path_and_file = format!("{}/{}", base_path, file_name);
    fs::remove_file(base_path_and_file)?;
    Ok(())
}
