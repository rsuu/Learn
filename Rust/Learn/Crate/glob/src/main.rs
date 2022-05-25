fn main() {
    println!("Hello, world!");
}

pub fn get_glob() {
    use glob::glob;

    for entry in glob("./*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}
