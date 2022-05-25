use std::{
    fs::File,
    io::{self, Read, Write},
};
use tempfile::{tempdir, tempfile, NamedTempFile};

fn main() {
    create_and_write().unwrap();
    create_file_handle().unwrap();
    creta_dir().unwrap();
}

fn create_and_write() -> io::Result<File> {
    // Create a file inside of `std::env::temp_dir()`.
    let file = tempfile()?;

    Ok(file)
}

fn create_file_handle() -> io::Result<NamedTempFile> {
    let text = "Brian was here. Briefly.";

    // Create a file inside of `std::env::temp_dir()`.
    let mut file1 = NamedTempFile::new()?;

    // Re-open it.
    let mut file2 = file1.reopen()?;

    // Write some test data to the first handle.
    file1.write_all(text.as_bytes())?;

    // Read the test data using the second handle.
    let mut buf = String::new();
    file2.read_to_string(&mut buf)?;
    assert_eq!(buf, text);
    Ok(file1)
}

fn creta_dir() -> io::Result<()> {
    // Create a directory inside of `std::env::temp_dir()`.
    let dir = tempdir()?;

    let file_path = dir.path().join("my-temporary-note.txt");
    let mut file = File::create(file_path)?;
    writeln!(file, "Brian was here. Briefly.")?;

    // By closing the `TempDir` explicitly, we can check that it has
    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.
    drop(file);
    dir.close()?;
    Ok(())
}
