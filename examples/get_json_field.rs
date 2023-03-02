use libmake::utils::get_json_field;
use std::path::Path;

fn main() {
    // Retrieve JSON field
    let file_path = "../tests/data/mylibrary.json";
    let field_author = "author";
    let value = if Path::new(file_path).exists() {
        get_json_field(Some(file_path), field_author)
    } else {
        String::new()
    };
    println!("🦀 get_json_field, ✅ {}: {}", field_author, value);
}
