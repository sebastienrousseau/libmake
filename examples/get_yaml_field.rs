use libmake::utils::get_yaml_field;
use std::path::Path;

fn main() {
    let file_path = "tests/data/mylibrary.yaml";
    let field_keywords = "keywords";

    let value = if Path::new(file_path).exists() {
        let keywords: Vec<String> = get_yaml_field(Some(file_path), field_keywords)
            .split('\n')
            .map(|s| s.trim_start_matches("- "))
            .filter(|s| !s.is_empty())
            .map(|s| format!("\"{}\"", s))
            .collect();
        format!("[{}]", keywords.join(", "))
    } else {
        String::new()
    };

    println!("🦀 get_yaml_field, ✅ {}: {}", field_keywords, value);
}
