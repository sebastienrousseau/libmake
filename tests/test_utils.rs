#[cfg(test)]
mod tests {

    use libmake::utils::{
        get_config_field, get_csv_field, get_json_field, get_yaml_field,
    };
    // Unit test for the `get_csv_field()` function.
    #[test]
    fn test_get_csv_field_from_csv() {
        let file_path = "./tests/data/mylibrary.csv";
        let field_author_index = 0;
        let value = get_csv_field(Some(file_path), field_author_index);
        assert_eq!(value, Some(vec!["Me".to_string()]));
    }
    // Unit test for the `get_csv_field()` function.
    #[test]
    fn test_get_csv_field_empty() {
        let file_path = "./tests/data/mylibrary.csv";
        let field_author_index = 20;
        let value = get_csv_field(Some(file_path), field_author_index);
        assert_eq!(value, Some(vec!["".to_string()]));
    }
    // Unit test for the `get_csv_field()` function.
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
    // Unit test for the `get_csv_fields()` function.
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
            Some(vec!["1.69.0".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 15),
            Some(vec!["0.1.9".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 16),
            Some(vec!["https://test.com".to_string()])
        );
    }
    // Unit test for the `get_json_field()` function.
    #[test]
    fn test_get_json_field_existing() {
        let file_path = None;
        let field_name = "null";
        let expected_value = "".to_string();
        let actual_value = get_json_field(file_path, field_name);
        assert_eq!(expected_value, actual_value);
    }
    // Unit test for the `get_yaml_field()` function.
    #[test]
    fn test_get_yaml_field_existing() {
        let file_path = None;
        let field_name = "null";
        let expected_value = "".to_string();
        let actual_value = get_yaml_field(file_path, field_name);
        assert_eq!(expected_value, actual_value);
    }
    // Unit test for the `get_config_field()` function.
    #[test]
    fn test_get_config_field_existing() {
        let file_path = None;
        let field_name = "nonexistent";
        let expected_value = "".to_string();

        let actual_yaml_value =
            get_config_field(file_path, Some("yaml"), field_name);
        assert_eq!(expected_value, actual_yaml_value);

        let actual_json_value =
            get_config_field(file_path, Some("json"), field_name);
        assert_eq!(expected_value, actual_json_value);

        let actual_empty_value =
            get_config_field(None, None, field_name);
        assert_eq!(expected_value, actual_empty_value);

        let actual_unknown_value =
            get_config_field(file_path, Some("unknown"), field_name);
        assert_eq!(expected_value, actual_unknown_value);
    }
}
