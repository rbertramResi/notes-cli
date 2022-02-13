use super::utils;
use std::fs::File;
use std::io;
use std::io::Write;


gflags::define! {
    -t, --text: utils::FlagText
}

pub fn add_entry(title: &str, text: &str) -> Result<(), utils::Error> {
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

