#[macro_use]
extern crate clap;

use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::exit;

use clap::{App, AppSettings, Arg};

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

fn main() {
    let matches = App::new("Detect")
        .version(crate_version!())
        .setting(AppSettings::ColoredHelp)
        .about("Traverse up parents looking for a file/folder by name")
        .arg(
            Arg::with_name("file_name")
                .help("The name of the file/folder to look for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("location")
                .help("Get the directory of the found file, instead of full path to the file")
                .short("l")
                .long("location"),
        )
        .get_matches();

    let to_find = matches.value_of("file_name").unwrap();
    let location = matches.is_present("location");
    let start = ".";

    exit(match find_in_dir(to_find, start) {
        Some(mut result) => {
            if location {
                result.pop();
            }
            println!("{}", result.display());
            0
        }
        None => {
            eprintln!("File not found");
            1
        }
    });
}
