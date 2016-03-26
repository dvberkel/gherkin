use std::collections::HashMap;

pub struct GherkinDialect {
    keywords: HashMap<String, Vec<String>>,
    language: String
}
