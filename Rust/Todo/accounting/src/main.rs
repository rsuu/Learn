use std::{
    env, fs, io,
    path::{Path},
    process,
    process::ExitStatus,
};

fn main() {
    test_main().unwrap();
}

fn test_main() -> Result<(), io::Error> {
    let file_path: &Path = Path::new("/tmp/test.rs");

    open_editor(file_path)?;
    let text = read_from_file(file_path)?;
    println!("{}", text);
    Ok(())
}

fn open_editor(file_path: &Path) -> io::Result<ExitStatus> {
    let editor = match env::var("EDITOR") {
        Ok(v) => v,
        Err(_) => process::exit(1),
    };

    process::Command::new(editor).arg(file_path).status()
}

fn read_from_file(file: &Path) -> io::Result<String> {
    fs::read_to_string(file)
}
