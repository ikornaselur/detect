#[macro_use]
extern crate clap;

use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::exit;

use clap::{App, AppSettings, Arg};

fn find_in_dir(file_name: &str, dir_path: &str, mut max_depth: usize) -> Option<PathBuf> {
    let mut current_path = Path::new(dir_path).canonicalize().unwrap();
    let file_name = OsStr::new(file_name);

    loop {
        let dir = read_dir(current_path.to_owned()).unwrap();

        for entry in dir {
            let entry = entry.unwrap();
            if entry.file_name() == file_name {
                return Some(entry.path().canonicalize().unwrap());
            }
        }
        max_depth -= 1;

        if !current_path.pop() || max_depth == 0 {
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
        ).arg(
            Arg::with_name("location")
                .help("Get the directory of the found file, instead of full path to the file")
                .short("l")
                .long("location"),
        ).arg(
            Arg::with_name("max_depth")
                .help("The max depth to search up, 1 being only current directory")
                .short("m")
                .long("max-depth")
                .value_name("max_depth")
                .takes_value(true),
        ).get_matches();

    let to_find = matches.value_of("file_name").unwrap();
    let location = matches.is_present("location");
    let max_depth = if matches.is_present("max_depth") {
        value_t!(matches, "max_depth", usize).unwrap_or_else(|e| e.exit())
    } else {
        10_000 // Just default to a huge number
    };
    let start = ".";

    exit(match find_in_dir(to_find, start, max_depth) {
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
