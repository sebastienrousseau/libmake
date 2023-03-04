use libmake::generator::generate_from_config;
fn main() {
    let file_path = "./tests/data/mylibrary.yaml";
    let file_type = "yaml";

    generate_from_config(file_path, file_type)
        .expect("Failed to generate the template files");
}
