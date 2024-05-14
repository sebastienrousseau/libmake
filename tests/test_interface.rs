#[cfg(test)]
mod tests {
    use libmake::{
        interface::replace_placeholders,
        models::model_params::FileGenerationParams,
    };
    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    };

    fn create_test_file(path: &PathBuf, content: &str) {
        let mut file =
            File::create(path).expect("Unable to create test file");
        file.write_all(content.as_bytes())
            .expect("Unable to write to test file");
    }

    #[test]
    fn test_successful_replacement() {
        let template_path = PathBuf::from("./tests/test_template.txt");
        let output_path = PathBuf::from("./tests/test_output.txt");

        // Create a template file with placeholders
        create_test_file(
            &template_path,
            "Hello, {{name}}!\nWelcome to {{website}}.",
        );

        // Setup FileGenerationParams with values to replace placeholders
        let params = FileGenerationParams {
            author: None,
            build: None,
            categories: None,
            description: None,
            documentation: None,
            edition: None,
            email: None,
            homepage: None,
            keywords: None,
            license: None,
            name: Some(String::from("Alice")),
            output: None,
            readme: None,
            repository: None,
            rustversion: None,
            version: None,
            website: Some(String::from("example.com")),
        };

        // Call the function and check the result
        if let Err(e) =
            replace_placeholders(&template_path, &output_path, &params)
        {
            eprintln!("Function failed with error: {:?}", e);
            panic!("Function failed");
        }

        // Check if the output file was created
        if !output_path.exists() {
            panic!("Output file was not created");
        }

        let output_content = fs::read_to_string(&output_path)
            .expect("Unable to read output file");
        assert_eq!(
            output_content,
            "Hello, {Alice}!\nWelcome to {example.com}.\n"
        );

        // Clean up
        fs::remove_file(template_path)
            .expect("Unable to delete test template file");
        fs::remove_file(output_path)
            .expect("Unable to delete test output file");
    }

    #[test]
    fn test_file_read_error() {
        let template_path = PathBuf::from("non_existent_template.txt");
        let output_path = PathBuf::from("test_output.txt");
        let params = FileGenerationParams::default();

        let result =
            replace_placeholders(&template_path, &output_path, &params);
        assert!(result.is_err());
    }

    #[test]
    fn test_file_write_error() {
        let template_path = PathBuf::from("test_template.txt");
        let output_path = PathBuf::from("/invalid/test_output.txt");

        create_test_file(&template_path, "Hello, {{name}}!");

        let params = FileGenerationParams {
            name: Some(String::from("Alice")),
            ..Default::default()
        };

        let result =
            replace_placeholders(&template_path, &output_path, &params);
        assert!(result.is_err());

        // Clean up
        fs::remove_file(template_path)
            .expect("Unable to delete test template file");
    }

    #[test]
    fn test_empty_template_file() {
        let template_path = PathBuf::from("empty_template.txt");
        let output_path = PathBuf::from("test_output.txt");

        // Create an empty template file
        create_test_file(&template_path, "");

        let params = FileGenerationParams::default();

        replace_placeholders(&template_path, &output_path, &params)
            .expect("Function failed");

        let output_content = fs::read_to_string(&output_path)
            .expect("Unable to read output file");
        assert_eq!(output_content, "");

        // Clean up
        fs::remove_file(template_path)
            .expect("Unable to delete test template file");
        fs::remove_file(output_path)
            .expect("Unable to delete test output file");
    }

    #[test]
    fn test_no_placeholders() {
        let template_path = PathBuf::from("no_placeholders.txt");
        let output_path = PathBuf::from("no_placeholders_output.txt");

        // Create a template file with no placeholders
        create_test_file(&template_path, "Hello, World!");

        let params = FileGenerationParams::default();

        replace_placeholders(&template_path, &output_path, &params)
            .expect("Function failed");

        let output_content = fs::read_to_string(&output_path)
            .expect("Unable to read output file");
        assert_eq!(output_content, "Hello, World!\n");

        // Clean up
        fs::remove_file(template_path)
            .expect("Unable to delete test template file");
        fs::remove_file(output_path)
            .expect("Unable to delete test output file");
    }

    #[test]
    fn test_missing_placeholders() {
        let template_path = PathBuf::from("missing_placeholders.txt");
        let output_path =
            PathBuf::from("missing_placeholders_output.txt");

        // Create a template file with some placeholders
        create_test_file(
            &template_path,
            "Hello, {{name}}! Your email is {{email}}.",
        );

        // Setup FileGenerationParams with values to replace some placeholders
        let params = FileGenerationParams {
            name: Some(String::from("Alice")),
            email: None,
            // Initialize other fields as necessary
            ..Default::default()
        };

        replace_placeholders(&template_path, &output_path, &params)
            .expect("Function failed");

        let output_content = fs::read_to_string(&output_path)
            .expect("Unable to read output file");
        assert_eq!(
            output_content,
            "Hello, {Alice}! Your email is {}.\n"
        );

        // Clean up
        fs::remove_file(template_path)
            .expect("Unable to delete test template file");
        fs::remove_file(output_path)
            .expect("Unable to delete test output file");
    }

    #[test]
    fn test_large_file() {
        let template_path = PathBuf::from("large_template.txt");
        let output_path = PathBuf::from("large_output.txt");

        // Create a large template file
        let large_content = "Hello, {name}! ".repeat(1000);
        create_test_file(&template_path, &large_content);

        let params = FileGenerationParams {
            name: Some(String::from("Alice")),
            // Initialize other fields as necessary
            ..Default::default()
        };

        replace_placeholders(&template_path, &output_path, &params)
            .expect("Function failed");

        let output_content = fs::read_to_string(&output_path)
            .expect("Unable to read output file");
        let looped_name = "Hello, Alice! ".repeat(1000);
        assert_eq!(output_content, format!("{}\n", looped_name));

        // Clean up
        fs::remove_file(template_path)
            .expect("Unable to delete test template file");
        fs::remove_file(output_path)
            .expect("Unable to delete test output file");
    }
}
