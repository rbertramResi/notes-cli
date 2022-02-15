use super::utils;

pub fn display_help() -> Result<(), utils::Error> {
    println!("~~~ COMMANDS ~~~");
    println!("read --title");
    println!("delete --title");
    println!("add --title --text");
    println!("list");
    println!("help");
    Ok(())
}
