#[cfg(test)]
mod tests {

    use libmake::utils::{get_csv_field, get_json_field, get_yaml_field};
    use std::path::Path;

    #[test]
    fn test_generate_via_csv() {
        let file_path = "./tests/data/mylibrary.csv";
        let field_author_index = 0;
        let value = get_csv_field(Some(file_path), field_author_index);
        assert_eq!(value, Some(vec!["Me".to_string()]));
    }

    // #[test]
    // fn test_generate_via_csv() {
    //     let mut file_path = PathBuf::from(env::current_dir().unwrap());
    //     file_path.push("tests/data/mylibrary.csv");
    //     let field_name = "mylibrary";
    //     let value = if file_path.exists() {
    //         get_csv_field(Some(file_path.to_str().unwrap()), field_name)
    //     } else {
    //         Some(String::new())
    //     };
    //     assert_eq!(value, Some(String::new()));
    // }

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
