use libmake::generator::generate_from_yaml;
fn main() {
    let yaml_file_path = "./tests/data/mylibrary.yaml";
    generate_from_yaml(yaml_file_path)
        .expect("Failed to generate the template files");
}
