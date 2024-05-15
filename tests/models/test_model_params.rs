#[cfg(test)]
mod tests {
    use libmake::models::model_params::FileGenerationParams;

    #[test]
    fn test_file_generation_params_default() {
        let params = FileGenerationParams::default_params();
        assert_eq!(params.author, Some("John Smith".to_string()));
        assert_eq!(params.build, Some("build.rs".to_string()));
        assert_eq!(
            params.categories,
            Some(
                "[\"category1\",\"category2\",\"category3\"]"
                    .to_string()
            )
        );
        assert_eq!(
            params.description,
            Some("A Rust library for doing cool things".to_string())
        );
        assert_eq!(
            params.documentation,
            Some("https://docs.rs/my_library".to_string())
        );
        assert_eq!(params.edition, Some("2021".to_string()));
        assert_eq!(
            params.email,
            Some("john.smith@example.com".to_string())
        );
        assert_eq!(
            params.homepage,
            Some("https://my_library.rs".to_string())
        );
        assert_eq!(
            params.keywords,
            Some("[\"rust\",\"library\",\"cool\"]".to_string())
        );
        assert_eq!(params.license, Some("MIT".to_string()));
        assert_eq!(params.name, Some("my_library".to_string()));
        assert_eq!(params.output, Some("my_library".to_string()));
        assert_eq!(params.readme, Some("README.md".to_string()));
        assert_eq!(
            params.repository,
            Some("https://github.com/example/my_library".to_string())
        );
        assert_eq!(params.rustversion, Some("1.60".to_string()));
        assert_eq!(params.version, Some("0.1.0".to_string()));
        assert_eq!(
            params.website,
            Some("https://example.com/john-smith".to_string())
        );
    }

    #[test]
    fn test_file_generation_params_new() {
        let params = FileGenerationParams::new();
        assert_eq!(params.author, Some("John Smith".to_string()));
        assert_eq!(params.build, Some("build.rs".to_string()));
        assert_eq!(
            params.categories,
            Some(
                "[\"category1\",\"category2\",\"category3\"]"
                    .to_string()
            )
        );
        assert_eq!(
            params.description,
            Some("A Rust library for doing cool things".to_string())
        );
        assert_eq!(
            params.documentation,
            Some("https://docs.rs/my_library".to_string())
        );
        assert_eq!(params.edition, Some("2021".to_string()));
        assert_eq!(
            params.email,
            Some("john.smith@example.com".to_string())
        );
        assert_eq!(
            params.homepage,
            Some("https://my_library.rs".to_string())
        );
        assert_eq!(
            params.keywords,
            Some("[\"rust\",\"library\",\"cool\"]".to_string())
        );
        assert_eq!(params.license, Some("MIT".to_string()));
        assert_eq!(params.name, Some("my_library".to_string()));
        assert_eq!(params.output, Some("my_library".to_string()));
        assert_eq!(params.readme, Some("README.md".to_string()));
        assert_eq!(
            params.repository,
            Some("https://github.com/example/my_library".to_string())
        );
        assert_eq!(params.rustversion, Some("1.60".to_string()));
        assert_eq!(params.version, Some("0.1.0".to_string()));
        assert_eq!(
            params.website,
            Some("https://example.com/john-smith".to_string())
        );
    }

    #[test]
    fn test_file_generation_params_from_args_missing_arg_name() {
        let args_str = "\"Jane Doe\" --email \"jane.doe@example.com\"";
        let result = FileGenerationParams::from_args(args_str);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Missing argument value".to_string()
        );
    }

    #[test]
    fn test_file_generation_params_from_args_missing_arg_value() {
        let args_str = "--author \"Jane Doe\" --email";
        let result = FileGenerationParams::from_args(args_str);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Missing argument value".to_string()
        );
    }

    #[test]
    fn test_deserialize_name() {
        let name = "\"my_project\"";
        let deserialized_name: Option<String> =
            serde_json::from_str(name).unwrap();
        assert_eq!(deserialized_name, Some("my_project".to_string()));
    }
}
