#[cfg(test)]
mod tests {
    use libmake::macro_get_field;
    use serde_json::from_reader;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempfile::tempdir;

    fn read_file<F, R>(
        file_path: &Path,
        f: F,
    ) -> Result<R, Box<dyn std::error::Error>>
    where
        F: FnOnce(File) -> Result<R, Box<dyn std::error::Error>>,
    {
        let file = File::open(file_path)?;
        f(file)
    }

    macro_get_field!(get_field, from_reader);

    #[test]
    fn test_macro_get_field_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");

        let json_data = r#"
        {
            "name": "Alice",
            "age": 30
        }
        "#;

        let mut file = File::create(&file_path).unwrap();
        file.write_all(json_data.as_bytes()).unwrap();

        let result =
            get_field(Some(file_path.to_str().unwrap()), "name");
        assert_eq!(result.unwrap(), "Alice");

        let result =
            get_field(Some(file_path.to_str().unwrap()), "age");
        assert_eq!(result.unwrap(), "30");
    }

    #[test]
    fn test_macro_get_field_field_not_found() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");

        let json_data = r#"
        {
            "name": "Alice",
            "age": 30
        }
        "#;

        let mut file = File::create(&file_path).unwrap();
        file.write_all(json_data.as_bytes()).unwrap();

        let result =
            get_field(Some(file_path.to_str().unwrap()), "address");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Field 'address' not found"
        );
    }

    #[test]
    fn test_macro_get_field_no_file_path() {
        let result = get_field(None, "name");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }
}
