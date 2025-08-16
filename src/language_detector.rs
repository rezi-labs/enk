use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ProgrammingLanguage {
    Rust,
    Python,
    Java,
    JavaScript,
    TypeScript,
    C,
    Cpp,
    Go,
    Swift,
    Kotlin,
    Php,
    Ruby,
    Scala,
    Haskell,
}

lazy_static! {
    static ref LANGUAGE_MAP: HashMap<&'static str, ProgrammingLanguage> = {
        let mut map = HashMap::new();
        map.insert("rs", ProgrammingLanguage::Rust);
        map.insert("py", ProgrammingLanguage::Python);
        map.insert("pyw", ProgrammingLanguage::Python);
        map.insert("java", ProgrammingLanguage::Java);
        map.insert("js", ProgrammingLanguage::JavaScript);
        map.insert("jsx", ProgrammingLanguage::JavaScript);
        map.insert("mjs", ProgrammingLanguage::JavaScript);
        map.insert("ts", ProgrammingLanguage::TypeScript);
        map.insert("tsx", ProgrammingLanguage::TypeScript);
        map.insert("c", ProgrammingLanguage::C);
        map.insert("h", ProgrammingLanguage::C);
        map.insert("cpp", ProgrammingLanguage::Cpp);
        map.insert("cxx", ProgrammingLanguage::Cpp);
        map.insert("cc", ProgrammingLanguage::Cpp);
        map.insert("hpp", ProgrammingLanguage::Cpp);
        map.insert("hxx", ProgrammingLanguage::Cpp);
        map.insert("go", ProgrammingLanguage::Go);
        map.insert("swift", ProgrammingLanguage::Swift);
        map.insert("kt", ProgrammingLanguage::Kotlin);
        map.insert("kts", ProgrammingLanguage::Kotlin);
        map.insert("php", ProgrammingLanguage::Php);
        map.insert("rb", ProgrammingLanguage::Ruby);
        map.insert("scala", ProgrammingLanguage::Scala);
        map.insert("sc", ProgrammingLanguage::Scala);
        map.insert("hs", ProgrammingLanguage::Haskell);
        map
    };
}

pub fn determine_language(file_extension: &str) -> Option<ProgrammingLanguage> {
    LANGUAGE_MAP.get(file_extension).cloned()
}
