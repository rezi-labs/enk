use crate::{
    file_finder::{EnvVar, FileInfo},
    language_detector::ProgrammingLanguage,
};

pub fn detect_envar(file: &FileInfo) -> Vec<EnvVar> {
    match file.get_language() {
        Some(ProgrammingLanguage::Rust) => detect_env_var_rust(file),
        Some(ProgrammingLanguage::Kotlin) => detect_env_var_kotlin(file),
        _ => vec![],
    }
}

pub fn detect_env_var_kotlin(file: &FileInfo) -> Vec<EnvVar> {
    let mut env_vars = Vec::new();
    let content = &file.content;

    // Split content into lines for line number tracking
    let lines: Vec<&str> = content.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        let line_num = (line_idx + 1) as u128; // 1-based line numbering
        let pattern = "System.getenv(\"";
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

    env_vars
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

#[cfg(test)]
mod kotlin_env_var_tests {
    use crate::env_var_detecter::detect_env_var_kotlin;
    use crate::file_finder::FileInfo;
    use crate::language_detector::ProgrammingLanguage;

    #[test]
    fn detects_kotlin_env_var() {
        let content = r#"val value = System.getenv("MY_ENV_VAR")"#;
        let file_info = FileInfo {
            full_path: std::path::PathBuf::from("Test.kt"),
            filename: "Test.kt".to_string(),
            file_type: "kt".to_string(),
            content: content.to_string(),
            language: Some(ProgrammingLanguage::Kotlin),
            envars: vec![],
        };
        let env_vars = detect_env_var_kotlin(&file_info);
        assert_eq!(env_vars.len(), 1);
        assert_eq!(env_vars[0].key, "MY_ENV_VAR");
    }
}
