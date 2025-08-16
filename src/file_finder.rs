use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

use crate::language_detector::ProgrammingLanguage;

#[derive(Debug, Clone)]
pub struct EnvVar {
    pub key: String,
    pub line: u128,
    pub col: u128,
}

impl EnvVar {
    pub fn new(key: String, line: u128, col: u128) -> Self {
        EnvVar { key, line, col }
    }
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub full_path: PathBuf,
    pub filename: String,
    pub file_type: String,
    pub content: String,
    pub language: Option<ProgrammingLanguage>,
    pub envars: Vec<EnvVar>,
}

impl FileInfo {
    fn new(path: PathBuf, content: String) -> Self {
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();

        let file_type = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();

        FileInfo {
            full_path: path,
            filename,
            file_type,
            content,
            language: None,
            envars: Vec::new(),
        }
    }

    pub fn set_language(&mut self, language: Option<ProgrammingLanguage>) {
        self.language = language;
    }

    pub fn add_env_var(&mut self, env_var: Vec<EnvVar>) {
        self.envars.extend(env_var);
    }
    pub fn get_language(&self) -> Option<ProgrammingLanguage> {
        self.language.clone()
    }
}

pub fn find_all_files<P: AsRef<Path>>(dir: P) -> Result<Vec<FileInfo>> {
    let mut files = Vec::new();
    find_files_recursive(dir.as_ref(), &mut files)?;
    Ok(files)
}

fn find_files_recursive(dir: &Path, files: &mut Vec<FileInfo>) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                find_files_recursive(&path, files)?;
            } else if path.is_file() {
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        files.push(FileInfo::new(path, content));
                    }
                    Err(_) => {
                        files.push(FileInfo::new(
                            path,
                            String::from("[Binary file or read error]"),
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}
