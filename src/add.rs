use super::utils;
use std::fs::File;
use std::io;
use std::io::Write;

gflags::define! {
    -t, --title: utils::FlagText
}

gflags::define! {
    -t, --text: utils::FlagText
}

pub fn add_entry() -> Result<(), utils::Error> {
    gflags::parse();
    let title = &TITLE.flag.value;
    let text = &TEXT.flag.value;
    println!("{:?}", title);
    let file_name_with_path = format!("{}/{}", utils::get_base_path(), title);
    match File::create(&file_name_with_path) {
        Ok(mut f) => {
            f.write_all(text.as_bytes())?;
            Ok(())
        },
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                utils::handle_missing_base();
                match File::create(&file_name_with_path) {
                    Ok(mut file_retry) => {
                       Ok(file_retry.write_all(text.as_bytes())?)
                    },
                    Err(e) => panic!("{}", e),
                }
            } else {
                 Err(utils::Error::IoError(e))
            }
         }
    }
}

