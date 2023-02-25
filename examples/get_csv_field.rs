use libmake::utils::get_csv_field;
use std::path::Path;

fn main() {
    // Retrieve CSV field
    let file_path = "tests/data/mylibrary.csv";
    let field_author = "author";
    let value = if Path::new(file_path).exists() {
        get_csv_field(Some(file_path), field_author)
    } else {
        String::new()
    };
    println!("🦀 get_csv_field, ✅ {}: {}", field_author, value);
}
