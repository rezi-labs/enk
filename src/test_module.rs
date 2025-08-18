#[allow(unused)]
mod vars {
    use std::env;

    fn x() {
        env::var("PATH").unwrap_or_else(|_| String::from("No PATH found"));
    }

    fn y() {
        env::var("HOME").unwrap_or_else(|_| String::from("No HOME found"));
    }
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
