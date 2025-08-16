mod env_var_detecter;
mod file_finder;
mod language_detector;
mod test_module;

use file_finder::{FileInfo, find_all_files};
use language_detector::determine_language;

#[macro_use]
extern crate lazy_static;

fn main() {
    match find_all_files(".") {
        Ok(files) => {
            let files = detect_languages_in_files(files);
            let files = detect_env_vars_in_files(files);

            println!("Found {} files:", files.len());
            for file in files {
                if file.language.is_some() {
                    display_file_info(&file);
                }
            }
        }
        Err(e) => eprintln!("Error reading files: {e}"),
    }
}

fn display_file_info(file: &FileInfo) {
    println!("Environment variables found:");

    println!("Path: {:?}", file.full_path);
    display_env_vars(file);
}

fn display_env_vars(file: &FileInfo) {
    for env_var in &file.envars {
        println!(
            "  {} at line {} col {}",
            env_var.key, env_var.line, env_var.col
        );
        println!(" Preview");
    }
}

// driver functions

pub fn detect_languages_in_files(mut files: Vec<FileInfo>) -> Vec<FileInfo> {
    for file in files.iter_mut() {
        let language = determine_language(&file.file_type);
        file.set_language(language);
    }

    files.clone()
}

pub fn detect_env_vars_in_files(mut files: Vec<FileInfo>) -> Vec<FileInfo> {
    for file in files.iter_mut() {
        let env_var = env_var_detecter::detect_envar(file);
        file.add_env_var(env_var);
    }

    files.clone()
}
