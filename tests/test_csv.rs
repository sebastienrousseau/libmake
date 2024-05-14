#[cfg(test)]
mod tests {
    use libmake::generators::csv::generate_from_csv;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_generate_from_csv_valid() {
        // Create a temporary CSV file
        let csv_content = "\
author,build,categories,description,documentation,edition,email,homepage,keywords,license,name,output,readme,repository,rustversion,version,website\n\
John Doe,cargo build,web,Sample project,,2021,john@example.com,https://example.com,example,Rust-1.0,SampleProject,/tmp/output,README.md,https://github.com/example,1.51.0,0.1.0,https://example.com\n";

        let path = "/tmp/test_valid.csv";
        let mut file = File::create(path).unwrap();
        file.write_all(csv_content.as_bytes()).unwrap();

        // Call the function and check for success
        let result = generate_from_csv(path);
        assert!(result.is_ok());

        // Clean up
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_generate_from_csv_invalid_path() {
        let result = generate_from_csv("/invalid/path/to/csv");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_from_csv_malformed_csv() {
        // Create a temporary malformed CSV file
        let csv_content = "author,build,categories,description,documentation,edition,email,homepage,keywords,license,name,output,readme,repository,rustversion,version,website\n\
John Doe,cargo build,web,Sample project,,2021,john@example.com,https://example.com,example,Rust-1.0,SampleProject,/tmp/output,README.md,https://github.com/example,1.51.0,0.1.0";

        let path = "/tmp/test_malformed.csv";
        let mut file = File::create(path).unwrap();
        file.write_all(csv_content.as_bytes()).unwrap();

        // Call the function and check for error
        let result = generate_from_csv(path);
        assert!(result.is_err());

        // Clean up
        std::fs::remove_file(path).unwrap();
    }
}
