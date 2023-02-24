#[cfg(test)]
mod tests {

    use libmake::cli::build_cli;

    #[test]
    fn test_build_cli_args() {
        let arg_specs = [
            ("author", "Me"),
            ("categories", "['category 1', 'category 2']"),
            ("csv", ""),
            ("description", "A library for doing things"),
            ("email", "test@test.com"),
            ("keywords", "['keyword1', 'keyword2']"),
            ("license", "MIT OR Apache-2.0"),
            ("name", "my_library"),
            ("output", "my_library"),
            ("repository", "https://github.com/test/test"),
            ("rustversion", "1.67.1"),
            ("version", "0.0.1"),
            ("website", "https://test.com"),
        ];

        let args = build_cli().unwrap();
        for (arg_name, expected_value) in arg_specs.iter() {
            let arg_value: Option<&String> = args.get_one(arg_name);
            assert_eq!(Some(&expected_value.to_string()), arg_value);
        }
    }
}
