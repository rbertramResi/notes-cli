mod utils;
mod add;

use std::env;

fn main() -> Result<(), utils::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(utils::throw_io_error("Missing command"))
    }

    let command = &args[1];

    match command.as_str() {
        "add" => add::add_entry("Name", "This is the my file"),
        _ => Err(utils::throw_io_error(format!("Invalid command: {}", command).as_str()))
    } 
}
