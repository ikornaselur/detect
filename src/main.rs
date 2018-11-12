use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

fn find_in_dir(file_name: &str, dir_path: &str) -> Option<PathBuf> {
    let mut current_path = Path::new(dir_path).canonicalize().unwrap();
    loop {
        let dir = read_dir(current_path.to_owned()).unwrap();

        for entry in dir {
            let entry = entry.unwrap();
            let entry_name = entry.file_name();
            if entry_name == OsStr::new(file_name) {
                return Some(entry.path().canonicalize().unwrap());
            }
        }
        if !current_path.pop() {
            break;
        }
    }
    None
}

fn main() -> std::io::Result<()> {
    let to_find = "adoijaisjd";
    let start = ".";

    match find_in_dir(to_find, start) {
        Some(result) => println!("Found: {:?}", result),
        None => println!("Didn't find it"),
    }

    Ok(())
}
