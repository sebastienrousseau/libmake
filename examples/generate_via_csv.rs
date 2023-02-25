use libmake::generator::generate_via_csv;
fn main() {
    let csv_file_path = "tests/data/mylibrary.csv";
    generate_via_csv(csv_file_path).expect("Failed to generate the template files");
}
