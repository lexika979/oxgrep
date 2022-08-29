use std::error::Error;
use std::fs::File;
use std::fs::{self, DirEntry};
use std::io::{BufRead, BufReader};
use std::path::Path;

use clap::{arg, command};

fn start_folder_descent(needle: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    for path in fs::read_dir(path)?.flatten() {
        find_recurse(needle, &path);
    }

    Ok(())
}

fn find_needle_in_file(needle: &str, entry: &DirEntry) -> Result<(), Box<dyn Error>> {
    let path = entry.path();
    let file_name = match path.to_str() {
        None => return Ok(()),
        Some(file_name) => file_name,
    };

    let reader = BufReader::new(File::open(file_name)?);
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(needle) {
            println!("{file_name} @ {}: {}", line_num + 1, line);
        }
    }

    Ok(())
}

fn find_recurse(needle: &str, entry: &DirEntry) {
    let file_type = if let Ok(file_type) = entry.file_type() {
        file_type
    } else {
        eprintln!(
            "Failed to determine file type for {}, skipping",
            entry.path().display()
        );
        return;
    };

    if file_type.is_file() {
        find_needle_in_file(needle, entry).unwrap_or_else(|err| {
            eprintln!("Failed to search file {}: {}", entry.path().display(), err);
        });
    } else if file_type.is_dir() {
        start_folder_descent(needle, &entry.path()).unwrap_or_else(|err| {
            eprintln!(
                "Failed to search folder {}: {}",
                entry.path().display(),
                err
            );
        });
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

    let needle = matches
        .get_one::<String>("string")
        .expect("Expected input string");
    start_folder_descent(needle, Path::new("")).expect("Failed to start search");
}
