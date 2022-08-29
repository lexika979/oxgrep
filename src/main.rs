use std::fs::{metadata, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{
    fs::{self, DirEntry},
    process,
};

use clap::{arg, command};

fn start_folder_descent(string: &String, path: &PathBuf) {
    let paths = match fs::read_dir(path) {
        Ok(value) => value,
        Err(error) => {
            println!("Failed to read folder contents: {error}");
            process::exit(-1);
        }
    };

    for path in paths.flatten() {
        find_recurse(string, &path);
    }
}

fn find_recurse(string: &String, path: &DirEntry) {
    let path = path.path();
    let file_name = match path.to_str() {
        None => return,
        Some(file_name) => file_name,
    };

    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            if metadata(path.clone())
                .expect("Failed to read file metadata")
                .is_dir()
            {
                start_folder_descent(string, &path);
            }

            return;
        }
    };

    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if line.contains(string) {
                println!("{file_name} @ {}: {line}", line_num + 1);
            }
        }
    }
}

fn main() {
    let matches = command!()
        .arg(
            arg!(
                [string] "String to search for"
            )
            .required(true),
        )
        .get_matches();

    let string = matches
        .get_one::<String>("string")
        .expect("Expected input string");
    start_folder_descent(string, &PathBuf::from(""));
}
