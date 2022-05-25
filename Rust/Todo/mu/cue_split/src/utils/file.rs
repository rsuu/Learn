use std::{fs, path::Path};

pub fn read_from_file(file: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(file)
}
