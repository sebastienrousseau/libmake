#[cfg(test)]
mod tests {
    use libmake::cli::build_cli;

    #[test]
    // Test that the arguments for the build CLI are correctly set
    fn test_build_cli_args() {
        // Define the expected argument values
        let arg_specs = [
            ("author", "Me"),
            ("build", "build.rs"),
            ("categories", "['category 1', 'category 2']"),
            ("description", "A library for doing things"),
            ("documentation", "https://lib.rs/crates/my_library"),
            ("edition", "2021"),
            ("email", "test@test.com"),
            ("homepage", "https://test.com"),
            ("keywords", "['keyword1', 'keyword2']"),
            ("license", "MIT OR Apache-2.0"),
            ("name", "my_library"),
            ("output", "my_library"),
            ("readme", "README.md"),
            ("repository", "https://github.com/test/test"),
            ("rustversion", "1.69.0"),
            ("version", "0.2.0"),
            ("website", "https://test.com"),
        ];

        // Call the build_cli function to get the command-line arguments
        let args = build_cli().unwrap();

        // Iterate through the expected argument values
        for (arg_name, expected_value) in arg_specs.iter() {
            // Get the actual value for the argument
            let arg_value: Option<&String> = args.get_one(arg_name);

            // Compare the actual and expected values
            assert_eq!(
                Some(&expected_value.to_string()),
                arg_value,
                "Incorrect value for argument {}",
                arg_name
            );
        }
    }
}
