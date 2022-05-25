use std::path::Path;

pub fn get_path(path: &Path) -> Vec<String> {
    use walkdir::WalkDir;

    let mut pf: Vec<String> = Vec::default();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        pf.push(
            entry
                .path()
                .to_path_buf()
                .into_os_string()
                .into_string()
                .unwrap(),
        );
    }

    pf
}
