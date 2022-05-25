use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use toml_edit::{value, Document};

fn main() {
    let filename = "tests/test.toml";
    if let Ok(txt) = fs::read_to_string(filename) {
        let mut toml = txt.parse::<Document>().expect("invalid doc");

        if toml["path"]["fleeting"].is_str() {
            toml["path"]["fleeting"] = value("b");
            write_to_file("./a", &toml.to_string()).unwrap();
        } else {
        }
    }
}

fn write_to_file(path: &str, text: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&text.as_bytes())
}

fn todo() {
    let toml = r#"
"hello" = 'toml!' # comment
['a'.b]
"#;
    let mut doc = toml.parse::<Document>().expect("invalid doc");
    assert_eq!(doc.to_string(), toml);
    // let's add a new key/value pair inside a.b: c = {d = "hello"}
    doc["a"]["b"]["c"]["d"] = value("hello");
    // autoformat inline table a.b.c: { d = "hello" }
    doc["a"]["b"]["c"].as_inline_table_mut().map(|t| t.fmt());
    let expected = r#"
"hello" = 'toml!' # comment
['a'.b]
c = { d = "hello" }
"#;
    assert_eq!(doc.to_string(), expected);

    let mut doc = toml_edit::Document::new();
    doc["foo"] = toml_edit::value("bar");
    let expected = r#"foo = "bar"
"#;
    assert_eq!(doc.to_string(), expected);

    let mut doc = toml_edit::Document::new();
    doc["foo"] = "'bar'".parse::<toml_edit::Item>().unwrap();
    let expected = r#"foo = 'bar'
"#;
    assert_eq!(doc.to_string(), expected);
}
