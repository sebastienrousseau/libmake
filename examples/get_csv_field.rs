use libmake::utils::get_csv_field;

fn main() {
    // Retrieve CSV field
    let file_path = "./tests/data/mylibrary.csv";
    println!(
        "🦀 get_csv_field, ✅ {:?}",
        get_csv_field(Some(file_path), 0)
    );
}
