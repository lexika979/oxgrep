use std::fs::{metadata, File};
use std::io::{BufRead, BufReader};
use std::{
    fs::{self, DirEntry},
    process,
};

use clap::{arg, command};

fn find_recurse(string: &String, path: &DirEntry) {
    let path = path.path();
    let file_name = path.to_str();
    if file_name.is_none() {
        return;
    }

    let file_name = file_name.unwrap();
    let file = File::open(file_name);
    if file.is_err() {
        if metadata(path.clone())
            .expect("Failed to read file metadata")
            .is_dir()
        {
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

        return;
    }

    let reader = BufReader::new(file.unwrap());

    let mut line_num = 0_u64;
    for line in reader.lines().flatten() {
        line_num += 1;

        if line.contains(string) {
            println!("{file_name} @ {line_num}: {line}");
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

    // clap forces the user to enter this argument, otherwise we will never reach this code, so we can just unwrap here
    let string = matches.get_one::<String>("string").unwrap();

    let paths = match fs::read_dir("") {
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
