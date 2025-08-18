mod env_var_detecter;
mod file_finder;
mod language_detector;
mod test_module;

use clap::{Parser, ValueEnum};
use file_finder::{FileInfo, find_all_files};
use language_detector::determine_language;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Json,
    Human,
    Csv,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Environment variable detector")]
struct Cli {
    #[arg(value_enum, short, long, default_value = "human")]
    format: OutputFormat,
    // Add subcommands here if needed
}

fn main() {
    let cli = Cli::parse();

    match find_all_files(".") {
        Ok(files) => {
            let files = detect_languages_in_files(files);
            let files = detect_env_vars_in_files(files);

            let file_with_env_vars: Vec<&FileInfo> = files
                .iter()
                .filter(|f| !f.envars.is_empty() && f.language.is_some())
                .collect();

            match cli.format {
                OutputFormat::Json => json_output_fn(&file_with_env_vars),
                OutputFormat::Human => human_output(file_with_env_vars),
                OutputFormat::Csv => csv_output_fn(&file_with_env_vars),
            }
        }
        Err(e) => eprintln!("Error reading files: {e}"),
    }
}

fn json_output_fn(files: &[&FileInfo]) {
    match serde_json::to_string_pretty(files) {
        Ok(json) => println!("{json}"),
        Err(e) => eprintln!("Error serializing to JSON: {e}"),
    }
}

fn human_output(file_with_env_vars: Vec<&FileInfo>) {
    let count = file_with_env_vars.iter().clone().count();
    let file_or_files = if count == 1 { "file" } else { "files" };

    println!("{count} {file_or_files} with environment variables");

    println!();

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

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn csv_output_fn(files: &[&FileInfo]) {
    println!("file_path,env_key,line,col");
    for file in files {
        let file_path = file.full_path.to_str().unwrap_or("");
        for env_var in &file.envars {
            println!(
                "{},{},{},{}",
                csv_escape(file_path),
                csv_escape(&env_var.key),
                env_var.line,
                env_var.col
            );
        }
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
