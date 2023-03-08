use libmake::generator::generate_from_csv;
fn main() {
    let csv_file_path = "./tests/data/mylibrary.csv";
    generate_from_csv(csv_file_path)
        .expect("Failed to generate the template files");
}
