use libmake::generator::generate_files_from_csv;
fn main() {
    let csv_file_path = "../tests/data/mylibrary.csv";
    generate_files_from_csv(csv_file_path)
        .expect("Failed to generate the template files");
}
