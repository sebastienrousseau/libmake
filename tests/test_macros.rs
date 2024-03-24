#[cfg(test)]
mod tests {

    use libmake::generator::FileGenerationParams;
    use std::path::Path;

    use libmake::macro_create_directories;

    use libmake::{
        macro_generate_files, macro_generate_from_config,
        macro_generate_from_csv, macro_generate_from_json,
        macro_generate_from_yaml,
    };

    // Unit test for the `create_directory()` function.
    #[test]
    fn test_create_directory() {
        let result = macro_create_directories!("my_library");
        match result {
            Ok(()) => println!("Directory created successfully!"),
            Err(err) => println!("Error creating directory: {}", err),
        }
    }
    // Unit test for the `generate_files()` function.
    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_generate_files() -> Result<(), String> {
        let mut params = FileGenerationParams::new();
        params.output = Some("my_library".into());
        macro_generate_files!(params.clone())?;
        Ok(())
    }
    // Unit test for the `generate_from_csv()` function.
    #[test]
    fn test_generate_from_csv() -> Result<(), String> {
        macro_generate_from_csv!("./tests/data/mylibrary.csv")?;
        Ok(())
    }
    // Unit test for the `generate_from_json()` function.
    #[test]
    fn test_generate_from_json() -> Result<(), String> {
        macro_generate_from_json!("./tests/data/mylibrary.json")?;
        Ok(())
    }
    // Unit test for the `generate_from_yaml()` function.
    #[test]
    fn test_generate_from_yaml() -> Result<(), String> {
        macro_generate_from_yaml!("./tests/data/mylibrary.yaml")?;
        Ok(())
    }
    // Unit test for the `generate_from_config()` function.
    #[test]
    fn test_generate_from_config() -> Result<(), String> {
        macro_generate_from_config!(
            "./tests/data/mylibrary.yaml",
            "yaml"
        )?;
        Ok(())
    }
    // Unit test for the `macro_create_directories!()` macro.
    #[test]
    fn test_macro_create_directories() {
        let result = macro_create_directories!("./target/tmp");
        match result {
            Ok(()) => println!("Directory created successfully!"),
            Err(err) => println!("Error creating directory: {}", err),
        }
        let _ = std::fs::remove_dir_all("./target/tmp");
    }
    // Unit test for the `macro_generate_from_csv!()` macro.
    #[test]
    fn test_macro_generate_from_csv() -> Result<(), String> {
        macro_generate_from_csv!("./tests/data/mylibrary.csv")?;
        assert!(Path::new("my_library").exists());
        Ok(())
    }
    // Unit test for the `macro_generate_from_json!()` macro.
    #[test]
    fn test_macro_generate_from_json() -> Result<(), String> {
        macro_generate_from_json!("./tests/data/mylibrary.json")?;
        assert!(Path::new("my_library").exists());
        Ok(())
    }
    // Unit test for the `macro_generate_from_yaml!()` macro.
    #[test]
    fn test_macro_generate_from_yaml() -> Result<(), String> {
        macro_generate_from_yaml!("./tests/data/mylibrary.yaml")?;
        assert!(Path::new("my_library").exists());
        Ok(())
    }
}
