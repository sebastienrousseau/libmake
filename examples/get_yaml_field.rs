use libmake::utils::get_yaml_field;
use std::path::Path;

fn main() {
    // Retrieve YAML field
    let file_path = "tests/data/mylibrary.yaml";
    let field_author = "author";
    let value = if Path::new(file_path).exists() {
        get_yaml_field(Some(file_path), field_author)
    } else {
        String::new()
    };
    println!("🦀 get_yaml_field, ✅ {}: {}", field_author, value);
}
