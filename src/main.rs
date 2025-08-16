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

            let file_with_env_vars = files
                .iter()
                .filter(|f| !f.envars.is_empty() && f.language.is_some());
           
            human_output(file_with_env_vars.collect());
        }
        Err(e) => eprintln!("Error reading files: {e}"),
    }
}


fn human_output(file_with_env_vars: Vec<&FileInfo>) {
    let count = file_with_env_vars.iter().clone().count();
    let file_or_files = if count == 1 { "file" } else { "files" };

    println!("{count} {file_or_files} with environment variables");
    
    println!("");

    for file in file_with_env_vars {
        display_file_info(file);
    }
}

fn display_file_info(file: &FileInfo) {
    if file.envars.is_empty() {
        return;
    }
    println!();

    display_env_vars(file);
}

fn display_env_vars(file: &FileInfo) {
    let lines: Vec<&str> = file.content.lines().collect();
    println!("{}:", file.full_path.to_str().unwrap());
    for env_var in &file.envars {
        println!(
            "{} at line {} col {} ",
            env_var.key, env_var.line, env_var.col,
        );

        let line_idx = (env_var.line as usize).saturating_sub(1);
        let start_line = line_idx.saturating_sub(2);
        let end_line = std::cmp::min(line_idx + 3, lines.len());

        for i in start_line..end_line {
            let marker = if i == line_idx { ">" } else { " " };
            let line = lines.get(i).unwrap_or(&"...");
            println!("{} {:3}:{}", marker, i + 1, line);
        }
        println!(" {:4}:...", end_line + 1);
        println!();
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
