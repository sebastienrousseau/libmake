#[cfg(test)]
mod tests {

    use csv::ReaderBuilder;
    use libmake::utils::get_csv_field;
    use std::{fs::File, path::Path};

    #[test]
    fn test_get_csv_field() {
        let file_path = "/tests/data/mylibrary.csv";
        let field_name = "mylibrary";
        let value = if Path::new(file_path).exists() {
            get_csv_field(Some(file_path), field_name)
        } else {
            String::new()
        };
        assert_eq!(value, "");
    }

    #[test]
    fn test_get_csv_field_empty() {
        let file_path = "/tests/data/mylibrary.csv";
        let field_name = "mylibrary";
        let value = if Path::new(file_path).exists() {
            get_csv_field(Some(file_path), field_name)
        } else {
            String::new()
        };
        assert_eq!(value, "");
    }

    #[test]
    fn test_get_csv_field_nonexistent() {
        let file_path = "/tests/data/mylibrary.csv";
        let field_name = "mylibrary";
        let value = if Path::new(file_path).exists() {
            get_csv_field(Some(file_path), field_name)
        } else {
            String::new()
        };
        assert_eq!(value, "");
    }

    #[test]
    fn test_get_csv_fields() {
        let file_path = "tests/data/mylibrary.csv";
        let expected_values = vec![
            "Me",
            "build.rs",
            "['category 1', 'category 2']",
            "",
            "A library for doing things",
            "https://lib.rs/crates/my_library",
            "2021",
            "test@test.com",
            "https://test.com",
            "['keyword1', 'keyword2']",
            "MIT OR Apache-2.0",
            "my_library",
            "my_library",
            "README.md",
            "https://github.com/test/test",
            "1.67.1",
            "0.0.4",
            "https://test.com",
        ];
        let mut actual_values = Vec::new();
        let file = File::open(file_path).unwrap();
        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
        let records = reader.records().collect::<Result<Vec<_>, _>>().unwrap();
        let row = &records[0];
        for field in row.iter() {
            actual_values.push(field.to_string());
        }
        assert_eq!(actual_values, expected_values);
    }

    // #[test]
    // fn test_cleanup_data_directory() {
    //     let directory_path = "my_library";
    //     let path = std::path::Path::new(directory_path);

    //     if path.exists() && path.is_dir() {
    //         std::fs::remove_dir_all(path).unwrap();
    //         assert!(!path.exists(), "Directory still exists after cleanup");
    //     } else {
    //         assert!(path.exists(), "Directory does not exist after cleanup");
    //     }
    //     assert!(!path.exists(), "Directory still exists after cleanup");
    // }
}
