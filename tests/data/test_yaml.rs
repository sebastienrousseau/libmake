#[cfg(test)]
mod tests {
    use libmake::generators::yaml::generate_from_yaml;
    use std::io::{self, Write};
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_generate_from_yaml_success() {
        let dir = tempdir().unwrap();
        let file_path = "./tests/data/mylibrary.yaml";

        let yaml_data = r#"
            output: test_output_dir
        "#;

        let mut file = File::create(&file_path).unwrap();
        file.write_all(yaml_data.as_bytes()).unwrap();

        let result = generate_from_yaml(file_path.to_str().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_from_yaml_file_not_found() {
        let result = generate_from_yaml("non_existent_file.yaml");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_from_yaml_cannot_read() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.yaml");

        // Create a directory with the same name to cause a read error
        std::fs::create_dir(&file_path).unwrap();

        let result = generate_from_yaml(file_path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_from_yaml_invalid_utf8() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.yaml");

        // Write invalid UTF-8 data to the file
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&[0x80, 0x81, 0x82]).unwrap();

        let result = generate_from_yaml(file_path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_from_yaml_deserialize_error() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.yaml");

        let invalid_yaml_data = r#"
            invalid_field: invalid_value
        "#;

        let mut file = File::create(&file_path).unwrap();
        file.write_all(invalid_yaml_data.as_bytes()).unwrap();

        let result = generate_from_yaml(file_path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_from_yaml_generate_files_error() {
        // This test assumes that macro_generate_files! returns an error for certain parameters.
        // Adjust the test if necessary based on the actual implementation of the macro.
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.yaml");

        let yaml_data = r#"
            output: test_output_dir
        "#;

        let mut file = File::create(&file_path).unwrap();
        file.write_all(yaml_data.as_bytes()).unwrap();

        // Mocking or adjusting the macro_generate_files! to return an error would be necessary.
        // This part of the test may require more advanced setup depending on the macro implementation.

        let result = generate_from_yaml(file_path.to_str().unwrap());
        assert!(result.is_err());
    }
}
