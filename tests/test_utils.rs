#[cfg(test)]
mod tests {

    use libmake::utils::{
        get_config_field, get_csv_field, get_json_field, get_yaml_field,
    };
    // Unit test for the `get_csv_field()` function.
    #[test]
    fn test_get_csv_field() {
        let file_path = "./tests/data/mylibrary.csv";

        // Test with valid field index
        let field_index = 0;
        let expected_value = Some(vec!["Me".to_string()]);
        let actual_value = get_csv_field(Some(file_path), field_index);
        assert_eq!(expected_value, actual_value);

        // Test with invalid field index
        let field_index = 100;
        let expected_value = None;
        let actual_value = get_csv_field(Some(file_path), field_index);
        assert_eq!(expected_value, actual_value);
    }

    // Unit test for the `get_csv_fields()` function.
    #[test]
    fn test_get_csv_fields() {
        let file_path = "./tests/data/mylibrary.csv";

        // Test the function with various input values
        assert_eq!(
            get_csv_field(Some(file_path), 0),
            Some(vec!["Me".to_string()])
        );
        print!("{:?}", get_csv_field(Some(file_path), 0));
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
            Some(vec!["1.75.0".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 15),
            Some(vec!["0.2.5".to_string()])
        );
        assert_eq!(
            get_csv_field(Some(file_path), 16),
            Some(vec!["https://test.com".to_string()])
        );
    }

    // Unit test for the `get_json_field()` function.
    #[test]
    fn test_get_json_field_existing() {
        let file_path = Some("./tests/data/mylibrary.json");
        let field_name = "name";
        let expected_value = Ok("my_library".to_string());
        let actual_value = get_json_field(file_path, field_name)
            .map_err(|err| err.to_string());
        assert_eq!(expected_value, actual_value);
    }

    // Unit test for the `get_yaml_field()` function.
    #[test]
    fn test_get_yaml_field_existing() {
        let file_path = Some("./tests/data/mylibrary.yaml");
        let field_name = "description";
        let expected_value =
            Ok("A library for doing things".to_string());
        let actual_value = get_yaml_field(file_path, field_name)
            .map_err(|err| err.to_string());
        assert_eq!(expected_value, actual_value);
    }

    // Unit test for the `get_config_field()` function.
    // Unit test for the `get_config_field()` function.
    #[test]
    fn test_get_config_field_existing() {
        let file_path = Some("./tests/data/mylibrary.yaml");
        let field_name = "license";
        let expected_value = "MIT OR Apache-2.0";
        let actual_yaml_value =
            get_config_field(file_path, Some("yaml"), field_name)
                .unwrap_or_else(|_| {
                    panic!("Failed to get config field: {}", field_name)
                });

        assert_eq!(expected_value, actual_yaml_value);

        let json_file_path = Some("./tests/data/mylibrary.json");
        let actual_json_value =
            get_config_field(json_file_path, Some("json"), field_name)
                .map_err(|err| err.to_string())
                .unwrap();
        assert_eq!(expected_value, actual_json_value);

        // Test with an empty file format
        let actual_empty_format_value =
            get_config_field(file_path, None, field_name)
                .map_err(|err| err.to_string())
                .unwrap_err();
        assert_eq!(
            actual_empty_format_value,
            "File format is not provided"
        );

        // Test with an unknown file format
        let actual_unknown_format_value =
            get_config_field(file_path, Some("unknown"), field_name)
                .map_err(|err| err.to_string())
                .unwrap_err();
        assert_eq!(actual_unknown_format_value, "Unsupported file format: unknown. Supported formats are 'json', 'yaml', and 'ini'.");
    }
}
