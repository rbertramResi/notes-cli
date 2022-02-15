mod utils;
mod add;
mod list;
mod delete;
mod read;
mod help;

use std::env;

gflags::define! {
    --text: utils::FlagText
}
gflags::define! {
    --title: utils::FlagText
}

fn main() -> Result<(), utils::Error> {
    gflags::parse();

    let file_name = match &TITLE.is_present() {
        true => &TITLE.flag.value,
        _ => "",
    };
    
    let file_text = match &TEXT.is_present() {
        true => &TEXT.flag.value,
        _ => "",
    };

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(utils::throw_io_error("Missing command"))
    }

    let command = &args[1];

    match command.as_str() {
        "add" => add::add_entry(file_name, file_text),
        "list" => list::list_dirs(),
        "delete" => delete::delete_file(file_name),
        "read" => read::read_file(file_name),
        "help" => help::display_help(),
        _ => Err(utils::throw_io_error(format!("Invalid command: {}", command).as_str()))
    } 
}
