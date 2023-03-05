#[cfg(test)]
mod tests {

    use libmake::utils::{
        get_config_field, get_csv_field, get_json_field, get_yaml_field,
    };

    #[test]
    fn test_get_csv_field_from_csv() {
        let file_path = "./tests/data/mylibrary.csv";
        let field_author_index = 0;
        let value = get_csv_field(Some(file_path), field_author_index);
        assert_eq!(value, Some(vec!["Me".to_string()]));
    }

    #[test]
    fn test_get_csv_field_empty() {
        let file_path = "./tests/data/mylibrary.csv";
        let field_author_index = 20;
        let value = get_csv_field(Some(file_path), field_author_index);
        assert_eq!(value, Some(vec!["".to_string()]));
    }

    #[test]
    fn test_get_csv_field_nonexistent() {
        let file_path = "./tests/data/mylibrary.csv";
        let field_author_index = 0;
        let value = get_csv_field(Some(file_path), field_author_index);
        assert_eq!(value, Some(vec!["Me".to_string()]));
    }

    #[test]
    fn test_get_csv_field() {
        // Test with valid field index
        let file_path = "./tests/data/mylibrary.csv";
        let field_index = 0;
        let expected_value = Some(vec!["Me".to_string()]);
        let actual_value = get_csv_field(Some(file_path), field_index);
        assert_eq!(expected_value, actual_value);

        // Test with invalid field index
        let field_index = 100;
        let expected_value = Some(vec!["".to_string()]);
        let actual_value = get_csv_field(Some(file_path), field_index);
        assert_eq!(expected_value, actual_value);
    }

    #[test]
    fn test_get_csv_fields() {
        let file_path = "./tests/data/mylibrary.csv";

        // Test the function with various input values
        assert_eq!(file_path, "./tests/data/mylibrary.csv");
        assert_eq!(
            get_csv_field(Some(file_path), 0),
            Some(vec!["Me".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 1),
            Some(vec!["build.rs".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 2),
            Some(vec!["['category 1', 'category 2']".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 3),
            Some(vec!["A library for doing things".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 4),
            Some(vec!["https://lib.rs/crates/my_library".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 5),
            Some(vec!["2021".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 6),
            Some(vec!["test@test.com".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 7),
            Some(vec!["https://test.com".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 8),
            Some(vec!["['keyword1', 'keyword2']".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 9),
            Some(vec!["MIT OR Apache-2.0".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 10),
            Some(vec!["my_library".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 11),
            Some(vec!["my_library".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 12),
            Some(vec!["README.md".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 13),
            Some(vec!["https://github.com/test/test".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 14),
            Some(vec!["1.67.1".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 15),
            Some(vec!["0.1.1".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 16),
            Some(vec!["https://test.com".to_string()])
        );
    }

    #[test]
    fn test_get_json_field_existing() {
        let file_path = "./tests/data/mylibrary.json";
        let field_name = "repository";
        let expected_value =
            "\"https://github.com/test/test\"".to_string();
        let actual_value = get_json_field(Some(file_path), field_name);
        assert_eq!(expected_value, actual_value);
    }

    #[test]
    fn test_get_json_field_nonexistent() {
        let file_path = "./tests/data/mylibrary.json";
        let field_name = "null";
        let expected_value = "null".to_string();
        let actual_value = get_json_field(Some(file_path), field_name);
        assert_eq!(expected_value, actual_value);
    }

    #[test]
    fn test_get_yaml_field_existing() {
        let file_path = "./tests/data/mylibrary.yaml";
        let field_name = "description";
        let expected_value = "A library for doing things".to_string();
        let actual_value = get_yaml_field(Some(file_path), field_name);
        assert_eq!(expected_value, actual_value);
    }

    #[test]
    fn test_get_yaml_field_nonexistent() {
        let file_path = "./tests/data/mylibrary.yaml";
        let field_name = "null";
        let expected_value = "null".to_string();
        let actual_value = get_yaml_field(Some(file_path), field_name);
        assert_eq!(expected_value, actual_value);
    }

    #[test]
    fn test_get_config_field_existing() {
        let file_path = "./tests/data/mylibrary.yaml";
        let field_name = "description";
        let expected_value = "A library for doing things".to_string();
        let actual_value =
            get_config_field(Some(file_path), Some("yaml"), field_name);
        assert_eq!(expected_value, actual_value);
    }

    #[test]
    fn test_cleanup_data_directory() {
        let directory_path = "my_library";
        let path_dir = std::path::Path::new(directory_path);

        // Remove the directory
        println!("Removing directory: {:?}", path_dir);
        std::fs::remove_dir_all(path_dir).unwrap();

        assert!(
            !std::path::Path::new(path_dir).exists(),
            "Directory still exists after cleanup"
        );
    }
}
