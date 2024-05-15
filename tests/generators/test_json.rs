#[cfg(test)]
mod tests {
    use libmake::generators::json::generate_from_json;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_generate_from_json_success() {
        let file_path = "./tests/data/mylibrary.json";
        generate_from_json(file_path).unwrap();
        assert_eq!(true, true);
    }

    #[test]
    fn test_generate_from_json_file_not_found() {
        let result = generate_from_json("non_existent_file.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_from_json_cannot_read() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");

        // Create a directory with the same name to cause a read error
        std::fs::create_dir(&file_path).unwrap();

        let result = generate_from_json(file_path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_from_json_invalid_utf8() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");

        // Write invalid UTF-8 data to the file
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&[0x80, 0x81, 0x82]).unwrap();

        let result = generate_from_json(file_path.to_str().unwrap());
        assert!(result.is_err());
    }
}
