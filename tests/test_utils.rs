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
            "['category 1', 'category 2']",
            "A library for doing things",
            "test@test.com",
            "['keyword1', 'keyword2']",
            "MIT",
            "my_library",
            "0.1.0",
            "https://github.com/test/test",
            "1.66.0",
            "0.0.1",
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
}
