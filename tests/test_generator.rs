#[cfg(test)]
mod tests {

    use libmake::utils::{get_csv_field, get_json_field, get_yaml_field};
    use std::path::Path;

    #[test]
    fn test_generate_via_csv() {
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
    fn test_generate_via_json() {
        let file_path = "/tests/data/mylibrary.json";
        let field_name = "mylibrary";
        let value = if Path::new(file_path).exists() {
            get_json_field(Some(file_path), field_name)
        } else {
            String::new()
        };
        assert_eq!(value, "");
    }

    #[test]
    fn test_generate_via_yaml() {
        let file_path = "/tests/data/mylibrary.yaml";
        let field_name = "mylibrary";
        let value = if Path::new(file_path).exists() {
            get_yaml_field(Some(file_path), field_name)
        } else {
            String::new()
        };
        assert_eq!(value, "");
    }
}
