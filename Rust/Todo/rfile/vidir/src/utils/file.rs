use std::{fs, path::Path};

pub fn read_from_file(file: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(file)
}

pub enum Ft {
    S,
    F,
    D,
}

pub fn get_type(path: &str) -> Result<Ft, std::io::Error> {
    use std::fs::metadata;

    let filetype = metadata(path)?.file_type();

    match filetype.is_symlink() {
        false => {
            match filetype.is_dir() {
                false => {
                    Ok(Ft::F) // if file
                }
                true => Ok(Ft::D),
            }
        }
        true => Ok(Ft::S),
    }
}
