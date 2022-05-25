#![feature(format_args_capture)]
fn format_args_capture() {
    let a = b'c';
    let b = r#"lssss"#;

    println!("{a}");
    println!("{b}");
}

fn main() {
    format_args_capture();
}

pub fn prefixes() {
    /*
    let a = "aaa";
    let b = b#123;
    println!("{}", f"{a}");
    println!("{}", s"aaaa");
    */
    // https://doc.rust-lang.org/beta/edition-guide/rust-2021/reserving-syntax.html
}
