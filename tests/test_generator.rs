#[cfg(test)]
mod tests {

    use libmake::{
        generator::{generate_from_config, generate_from_yaml},
        utils::{get_csv_field, get_json_field, get_yaml_field},
    };
    use std::path::Path;

    #[test]
    fn test_get_csv_field() {
        let file_path = "./tests/data/mylibrary.csv";
        let field_author_index = 0;
        let value = get_csv_field(Some(file_path), field_author_index);
        assert_eq!(value, Some(vec!["Me".to_string()]));
    }

    #[test]
    fn test_get_json_field() {
        let file_path = "./tests/data/mylibrary.json";
        let field_name = "mylibrary";
        let value = if Path::new(file_path).exists() {
            get_json_field(Some(file_path), field_name)
        } else {
            String::new()
        };
        assert_eq!(value, "null");
    }

    #[test]
    fn test_get_yaml_field() {
        let file_path = "./tests/data/mylibrary.yaml";
        let field_name = "mylibrary";
        let value = if Path::new(file_path).exists() {
            get_yaml_field(Some(file_path), field_name)
        } else {
            String::new()
        };
        assert_eq!(value, "null");
    }

    #[test]
    fn test_generate_from_config() {
        let file_path = "./tests/data/mylibrary.yaml";
        let file_type = "yaml";
        generate_from_config(file_path, file_type).unwrap();
        assert_eq!(true, true); // If we get here without panicking, the test has passed
    }
    #[test]
    fn test_generate_from_csv() {
        let file_path = "./tests/data/mylibrary.csv";
        generate_from_config(file_path, "csv").unwrap();
        assert_eq!(true, true); // If we get here without panicking, the test has passed
    }
    #[test]
    fn test_generate_from_json() {
        let file_path = "./tests/data/mylibrary.json";
        generate_from_config(file_path, "json").unwrap();
        assert_eq!(true, true); // If we get here without panicking, the test has passed
    }
    #[test]
    fn test_generate_from_yaml() {
        let file_path = "./tests/data/mylibrary.yaml";
        generate_from_yaml(file_path).unwrap();
        assert_eq!(true, true); // If we get here without panicking, the test has passed
    }
}
