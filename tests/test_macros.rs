#[cfg(test)]
mod tests {

    extern crate libmake;
    use libmake::generator::{
        generate_files, generate_from_csv,
        generate_from_json, generate_from_yaml,
    };
    use libmake::generator::{
        generate_from_config, FileGenerationParams,
    };
    use std::path::Path;

    use libmake::{
        assert_create_directory, assert_generate_files,
        assert_generate_from_config, assert_generate_from_csv,
        assert_generate_from_json, assert_generate_from_yaml,
    };

    // Unit test for the `create_directory()` function.
    #[test]
    fn test_create_directory() {
        assert_create_directory!("my_library");
    }
    // Unit test for the `generate_files()` function.
    #[test]
    fn test_generate_files() {
        let mut params = FileGenerationParams::new();
        params.output = Some("my_library".into());
        assert_generate_files!(params.clone());
    }
    // Unit test for the `generate_from_csv()` function.
    #[test]
    fn test_generate_from_csv() {
        assert_generate_from_csv!("./tests/data/mylibrary.csv");
    }
    // Unit test for the `generate_from_json()` function.
    #[test]
    fn test_generate_from_json() {
        assert_generate_from_json!("./tests/data/mylibrary.json");
    }
    // Unit test for the `generate_from_yaml()` function.
    #[test]
    fn test_generate_from_yaml() {
        assert_generate_from_yaml!("./tests/data/mylibrary.yaml");
    }
    // Unit test for the `generate_from_config()` function.
    #[test]
    fn test_generate_from_config() {
        assert_generate_from_config!(
            "./tests/data/mylibrary.yaml",
            "yaml"
        );
    }
    // Unit test for the `assert_create_directory!()` macro.
    #[test]
    fn test_assert_create_directory() {
        assert_create_directory!("./target/tmp");
        let _ = std::fs::remove_dir_all("./target/tmp");
    }
    // Unit test for the `assert_generate_from_csv!()` macro.
    #[test]
    fn test_assert_generate_from_csv() {
        assert_generate_from_csv!("./tests/data/mylibrary.csv");
        assert!(Path::new("my_library").exists());
    }
    // Unit test for the `assert_generate_from_json!()` macro.
    #[test]
    fn test_assert_generate_from_json() {
        assert_generate_from_json!("./tests/data/mylibrary.json");
        assert!(Path::new("my_library").exists());
    }
    // Unit test for the `assert_generate_from_yaml!()` macro.
    #[test]
    fn test_assert_generate_from_yaml() {
        assert_generate_from_yaml!("./tests/data/mylibrary.yaml");
        assert!(Path::new("my_library").exists());
    }
}
