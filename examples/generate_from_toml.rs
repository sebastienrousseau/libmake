use libmake::generator::generate_from_toml;
fn main() {
    let toml_file_path = "./tests/data/mylibrary.toml";
    generate_from_toml(toml_file_path)
        .expect("Failed to generate the template files");
}
