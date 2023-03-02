use libmake::generator::generate_via_json;
fn main() {
    let json_file_path = "../tests/data/mylibrary.json";
    generate_via_json(json_file_path)
        .expect("Failed to generate the template files");
}
