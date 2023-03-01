#[cfg(test)]
mod tests {

    extern crate libmake;
    use libmake::generator::FileGenerationParams;
    use libmake::generator::{
        create_directory, generate_files, generate_files_from_csv,
        generate_via_json, generate_via_yaml,
    };
    use std::path::Path;

    use libmake::{
        assert_create_directory, assert_generate_files,
        assert_generate_files_from_csv, assert_generate_via_json,
        assert_generate_via_yaml,
    };

    #[test]
    fn test_create_directory() {
        assert_create_directory!("my_library");
    }

    #[test]
    fn test_generate_files() {
        let mut params = FileGenerationParams::default();
        params.output = Some("my_library".into());
        assert_generate_files!(params);
    }

    #[test]
    fn test_generate_files_from_csv() {
        assert_generate_files_from_csv!("./tests/data/mylibrary.csv");
    }

    #[test]
    fn test_generate_via_json() {
        assert_generate_via_json!("./tests/data/mylibrary.json");
    }

    #[test]
    fn test_generate_via_yaml() {
        assert_generate_via_yaml!("./tests/data/mylibrary.yaml");
    }
}
