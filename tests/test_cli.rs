#[cfg(test)]
mod tests {

    use libmake::cli::build_cli;

    #[test]
    fn test_build_cli_args() {
        let arg_specs = [
            ("author", ""),
            ("categories", ""),
            ("csv", ""),
            ("description", ""),
            ("email", ""),
            ("keywords", ""),
            ("license", "MIT OR Apache-2.0"),
            ("name", "my_library"),
            ("output", "my_library"),
            ("repository", ""),
            ("rustversion", "1.66.1"),
            ("version", "0.1.0"),
            ("website", ""),
        ];

        let args = build_cli().unwrap();
        for (arg_name, expected_value) in arg_specs.iter() {
            let arg_value: Option<&String> = args.get_one(arg_name);
            assert_eq!(Some(&expected_value.to_string()), arg_value);
        }
    }
}
