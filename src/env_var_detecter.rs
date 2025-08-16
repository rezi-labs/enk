use crate::{
    file_finder::{EnvVar, FileInfo},
    language_detector::ProgrammingLanguage,
};

pub fn detect_envar(file: &FileInfo) -> Vec<EnvVar> {
    match file.get_language() {
        Some(ProgrammingLanguage::Rust) => detect_env_var_rust(file),
        _ => vec![],
    }
}

fn detect_env_var_rust(file: &FileInfo) -> Vec<EnvVar> {
    let mut env_vars = Vec::new();
    let content = &file.content;

    // Split content into lines for line number tracking
    let lines: Vec<&str> = content.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        let line_num = (line_idx + 1) as u128; // 1-based line numbering

        let patterns = [
            "env::var(\"",
            "env::var_os(\"",
            "std::env::var(\"",
            "std::env::var_os(\"",
        ];

        for pattern in &patterns {
            let mut search_start = 0;

            while let Some(start_pos) = line[search_start..].find(pattern) {
                let absolute_start = search_start + start_pos;
                let quote_start = absolute_start + pattern.len();

                // Find the closing quote
                if let Some(quote_end) = line[quote_start..].find('"') {
                    let env_key = &line[quote_start..quote_start + quote_end];
                    let col_num = (absolute_start + 1) as u128; // 1-based column numbering

                    env_vars.push(EnvVar::new(env_key.to_string(), line_num, col_num));
                }

                search_start = absolute_start + pattern.len();
            }
        }
    }

    env_vars
}
