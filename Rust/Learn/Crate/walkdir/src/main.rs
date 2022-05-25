use walkdir::WalkDir;

fn main() {
    for e in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            println!("{}", e.path().display());
        }
    }
}
