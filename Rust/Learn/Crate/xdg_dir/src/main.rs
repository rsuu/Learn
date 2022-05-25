fn main() {
    match dirs_next::config_dir() {
        Some(dir) => {
            println!("{:?}", dir);
        },
        _ => {}
    }
}
