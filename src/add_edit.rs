use super::utils;

pub fn add_edit_note(file_name: &str) -> Result<(), utils::Error> {
    let file_path = utils::get_file_location(file_name); 

    std::process::Command::new("vim")
    .arg(file_path)
    .spawn()
    .expect("Error: Failed to run editor")
    .wait()
    .expect("Error: Editor returned a non-zero status");
    Ok(())
}
