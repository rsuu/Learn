use std::{ffi::OsStr, path::Path};

fn main() {
    // Note: this example does work on Windows
    let path = Path::new("./foo/bar.txt");

    let parent = path.parent();
    assert_eq!(parent, Some(Path::new("./foo")));

    let file_stem = path.file_stem();
    assert_eq!(file_stem, Some(OsStr::new("bar")));

    let extension = path.extension();
    assert_eq!(extension, Some(OsStr::new("txt")));
}
