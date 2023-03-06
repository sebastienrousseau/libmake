#[cfg(test)]
mod tests {

    extern crate libmake;
    use libmake::generator::{
        create_directory, generate_files, generate_from_csv,
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

    #[test]
    fn test_create_directory() {
        assert_create_directory!("my_library");
    }

    #[test]
    fn test_generate_files() {
        let mut params = FileGenerationParams::new();
        params.output = Some("my_library".into());
        assert_generate_files!(params);
    }

    #[test]
    fn test_generate_from_csv() {
        assert_generate_from_csv!("./tests/data/mylibrary.csv");
    }

    #[test]
    fn test_generate_from_json() {
        assert_generate_from_json!("./tests/data/mylibrary.json");
    }

    #[test]
    fn test_generate_from_yaml() {
        assert_generate_from_yaml!("./tests/data/mylibrary.yaml");
    }

    #[test]
    fn test_generate_from_config() {
        assert_generate_from_config!(
            "./tests/data/mylibrary.yaml",
            "yaml"
        );
    }

    #[test]
    fn test_assert_create_directory() {
        assert_create_directory!("./target/tmp");
        let _ = std::fs::remove_dir_all("./target/tmp");
    }
    #[test]
    fn test_assert_generate_from_csv() {
        assert_generate_from_csv!("./tests/data/mylibrary.csv");
        assert!(Path::new("my_library").exists());
    }
    #[test]
    fn test_assert_generate_from_json() {
        assert_generate_from_json!("./tests/data/mylibrary.json");
        assert!(Path::new("my_library").exists());
    }
    #[test]
    fn test_assert_generate_from_yaml() {
        assert_generate_from_yaml!("./tests/data/mylibrary.yaml");
        assert!(Path::new("my_library").exists());
    }
}
