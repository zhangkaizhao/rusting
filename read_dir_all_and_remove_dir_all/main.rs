use std::fs;
use std::path::Path;

fn read_dir_all_and_remove_dir_all() {
    let path = Path::new(".");
    let mut subdirs = Vec::new();
    _append_subdirs(&mut subdirs, path);
    println!("{:?}", subdirs);
    subdirs.sort_unstable();
    while let Some(subdir) = subdirs.pop() {
        println!("{}", &subdir);
        fs::remove_dir(subdir).unwrap();
    }
}

fn _append_subdirs(subdirs: &mut Vec<String>, dir: &Path) {
    for entry in fs::read_dir(&dir).unwrap() {
        let subpath = entry.unwrap().path();
        if subpath.is_dir() {
            let subdir = subpath.to_str().unwrap().to_string();
            subdirs.push(subdir);
            _append_subdirs(subdirs, &subpath);
        }
    }
}

fn main() {
    read_dir_all_and_remove_dir_all();
}
