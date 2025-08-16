mod file_finder;
mod language_detector;

use file_finder::{FileInfo, find_all_files};
use language_detector::determine_language;

#[macro_use]
extern crate lazy_static;

fn main() {
    match find_all_files(".") {
        Ok(files) => {
            let files = detect_languages_in_files(files);

            println!("Found {} files:", files.len());
            for file in files {
                println!("Path: {:?}", file.full_path);
                println!("Filename: {}", file.filename);
                println!("Type: {}", file.file_type);
                println!("Language: {:?}", file.language);
                println!(
                    "Content preview: {}...",
                    file.content.chars().take(50).collect::<String>()
                );
                println!("---");
            }
        }
        Err(e) => eprintln!("Error reading files: {e}"),
    }
}

pub fn detect_languages_in_files(mut files: Vec<FileInfo>) -> Vec<FileInfo> {
    for file in files.iter_mut() {
        let language = determine_language(&file.file_type);
        file.set_language(language);
    }

    files.clone()
}
