use super::utils;
use colored::Colorize;

fn print_help_text(text: &str) {
    println!("{}", text.bright_blue());
}

pub fn display_help() -> Result<(), utils::Error> {
    println!("{}", "~~~ COMMANDS ~~~".yellow().bold());
    print_help_text("read -t (title)");
    print_help_text("delete -t (title)");
    print_help_text("add -t (title)");
    print_help_text("list");
    print_help_text("help");
    Ok(())
}
