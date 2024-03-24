use clap::{Arg, Command};
use libmake::args::extract_manual_params;
use libmake::{
    args::{process_arguments, validate_params},
    generator::FileGenerationParams,
};

// Tests the process_arguments function with valid arguments
#[test]
fn test_process_arguments() {
    let matches = Command::new("libmake")
        .subcommand(
            Command::new("file")
                .arg(
                    Arg::new("csv")
                        .long("csv")
                        .value_name("FILE")
                        .default_value("tests/data/mylibrary.csv"),
                )
                .arg(
                    Arg::new("ini")
                        .long("ini")
                        .value_name("FILE")
                        .default_value("tests/data/mylibrary.ini"),
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .value_name("FILE")
                        .default_value("tests/data/mylibrary.json"),
                )
                .arg(
                    Arg::new("yaml")
                        .long("yaml")
                        .value_name("FILE")
                        .default_value("tests/data/mylibrary.yaml"),
                )
                .arg(
                    Arg::new("toml")
                        .long("toml")
                        .value_name("FILE")
                        .default_value("tests/data/mylibrary.toml"),
                ),
        )
        .get_matches_from(vec![
            "libmake",
            "file",
            "--csv",
            "tests/data/mylibrary.csv",
            "--ini",
            "tests/data/mylibrary.ini",
            "--json",
            "tests/data/mylibrary.json",
            "--yaml",
            "tests/data/mylibrary.yaml",
            "--toml",
            "tests/data/mylibrary.toml",
        ]);

    let result = process_arguments(&matches);
    assert!(result.is_ok());
}

// Tests the validation of parameters with an invalid edition
#[test]
fn test_validate_params_invalid_edition() {
    let params = FileGenerationParams {
        name: Some("test_lib".to_string()),
        output: Some("output_dir".to_string()),
        author: None,
        build: None,
        categories: None,
        description: None,
        documentation: None,
        edition: Some("2023".to_string()), // Invalid edition
        email: None,
        homepage: None,
        keywords: None,
        license: None,
        readme: None,
        repository: None,
        rustversion: None,
        version: None,
        website: None,
    };

    let result = validate_params(&params);

    assert!(result.is_err());
    assert_eq!(
       result.unwrap_err().to_string(),
       "Invalid edition: 2023. Supported editions are 2015, 2018, and 2021.".to_string()
    );
}

// Tests the validation of parameters with an invalid documentation URL
#[test]
fn test_validate_params_invalid_documentation() {
    let params = FileGenerationParams {
        name: Some("test_lib".to_string()),
        output: Some("output_dir".to_string()),
        author: None,
        build: None,
        categories: None,
        description: None,
        documentation: Some("123".to_string()), // Invalid documentation
        edition: None,
        email: None,
        homepage: None,
        keywords: None,
        license: None,
        readme: None,
        repository: None,
        rustversion: None,
        version: None,
        website: None,
    };
    let result = validate_params(&params);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid documentation URL: 123. Documentation URL should start with 'http://' or 'https://'.".to_string()
    );
}

// Tests the validation of parameters with an invalid email address
#[test]
fn test_validate_params_invalid_email() {
    let params = FileGenerationParams {
        name: Some("test_lib".to_string()),
        output: Some("output_dir".to_string()),
        author: None,
        build: None,
        categories: None,
        description: None,
        documentation: None,
        edition: None,
        email: Some("<EMAIL>".to_string()), // Invalid email
        homepage: None,
        keywords: None,
        license: None,
        readme: None,
        repository: None,
        rustversion: None,
        version: None,
        website: None,
    };
    let result = validate_params(&params);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid email address: <EMAIL>. Email address should contain '@'.".to_string()
    );
}

// Tests the validation of parameters with an invalid homepage URL
#[test]
fn test_validate_params_invalid_homepage() {
    let params = FileGenerationParams {
        name: Some("test_lib".to_string()),
        output: Some("output_dir".to_string()),
        author: None,
        build: None,
        categories: None,
        description: None,
        documentation: None,
        edition: None,
        email: None,
        homepage: Some("123".to_string()), // Invalid homepage
        keywords: None,
        license: None,
        readme: None,
        repository: None,
        rustversion: None,
        version: None,
        website: None,
    };
    let result = validate_params(&params);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid homepage URL: 123. Homepage URL should start with 'http://' or 'https://'.".to_string()
    );
}

// Tests the validation of parameters with an invalid repository URL
#[test]
fn test_validate_params_invalid_repository() {
    let params = FileGenerationParams {
        name: Some("test_lib".to_string()),
        output: Some("output_dir".to_string()),
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
        readme: None,
        repository: Some("123".to_string()), // Invalid repository
        rustversion: None,
        version: None,
        website: None,
    };
    let result = validate_params(&params);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid repository URL: 123. Repository URL should start with 'https://' or 'git://'.".to_string()
    );
}

// Tests the validation of parameters with an invalid Rust version
#[test]
fn test_validate_params_invalid_rustversion() {
    let params = FileGenerationParams {
        name: Some("test_lib".to_string()),
        output: Some("output_dir".to_string()),
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
        readme: None,
        repository: None,
        rustversion: Some("2.0".to_string()), // Invalid Rust version
        version: None,
        website: None,
    };

    let result = validate_params(&params);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid Rust version: 2.0. Rust version should start with '1.'.".to_string()
    );
}

// Tests the validation of parameters with missing name
#[test]
fn test_validate_params_missing_name() {
    let params = FileGenerationParams {
        name: None,
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
        output: None,
        readme: None,
        repository: None,
        rustversion: None,
        version: None,
        website: None,
    };

    let result = validate_params(&params);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "The name of the library is required for manual generation."
            .to_string()
    );
}

// Tests the validation of parameters with missing output
#[test]
fn test_validate_params_missing_output() {
    let params = FileGenerationParams {
        name: Some("test_lib".to_string()),
        output: None,
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
        readme: None,
        repository: None,
        rustversion: None,
        version: None,
        website: None,
    };

    let result = validate_params(&params);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "The output directory is required for manual generation."
            .to_string()
    );
}

// Tests the validation of parameters with valid inputs
#[test]
fn test_validate_params_valid() {
    let params = FileGenerationParams {
        name: Some("test_lib".to_string()),
        output: Some("output_dir".to_string()),
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
        readme: None,
        repository: None,
        rustversion: None,
        version: None,
        website: None,
    };

    let result = validate_params(&params);

    assert!(result.is_ok());
}

// Tests extract manual parameters
#[test]
fn test_extract_manual_params_all_fields() {
    let matches = Command::new("libmake")
        .subcommand(
            Command::new("manual")
                .arg(
                    Arg::new("name")
                        .long("name")
                        .value_name("NAME")
                        .default_value("test_lib"),
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .value_name("OUTPUT")
                        .default_value("output_dir"),
                )
                .arg(
                    Arg::new("author")
                        .long("author")
                        .value_name("AUTHOR")
                        .default_value("John Doe"),
                )
                .arg(
                    Arg::new("build")
                        .long("build")
                        .value_name("BUILD")
                        .default_value("script.rs"),
                )
                .arg(
                    Arg::new("categories")
                        .long("categories")
                        .value_name("CATEGORIES")
                        .default_value("category1,category2"),
                )
                .arg(
                    Arg::new("description")
                        .long("description")
                        .value_name("DESCRIPTION")
                        .default_value("A test library"),
                )
                .arg(
                    Arg::new("documentation")
                        .long("documentation")
                        .value_name("DOCUMENTATION")
                        .default_value("https://docs.rs/test_lib"),
                )
                .arg(
                    Arg::new("edition")
                        .long("edition")
                        .value_name("EDITION")
                        .default_value("2021"),
                )
                .arg(
                    Arg::new("email")
                        .long("email")
                        .value_name("EMAIL")
                        .default_value("john@example.com"),
                )
                .arg(
                    Arg::new("homepage")
                        .long("homepage")
                        .value_name("HOMEPAGE")
                        .default_value("https://example.com"),
                )
                .arg(
                    Arg::new("keywords")
                        .long("keywords")
                        .value_name("KEYWORDS")
                        .default_value("keyword1,keyword2"),
                )
                .arg(
                    Arg::new("license")
                        .long("license")
                        .value_name("LICENSE")
                        .default_value("MIT"),
                )
                .arg(
                    Arg::new("readme")
                        .long("readme")
                        .value_name("README")
                        .default_value("README.md"),
                )
                .arg(
                    Arg::new("repository")
                        .long("repository")
                        .value_name("REPOSITORY")
                        .default_value(
                            "https://github.com/test/test_lib",
                        ),
                )
                .arg(
                    Arg::new("rustversion")
                        .long("rustversion")
                        .value_name("RUSTVERSION")
                        .default_value("1.60.0"),
                )
                .arg(
                    Arg::new("version")
                        .long("version")
                        .value_name("VERSION")
                        .default_value("0.1.0"),
                )
                .arg(
                    Arg::new("website")
                        .long("website")
                        .value_name("WEBSITE")
                        .default_value("https://example.com"),
                ),
        )
        .get_matches_from(vec![
            "libmake",
            "manual",
            "--name",
            "test_lib",
            "--output",
            "output_dir",
            "--author",
            "John Doe",
            "--build",
            "script.rs",
            "--categories",
            "category1,category2",
            "--description",
            "A test library",
            "--documentation",
            "https://docs.rs/test_lib",
            "--edition",
            "2021",
            "--email",
            "john@example.com",
            "--homepage",
            "https://example.com",
            "--keywords",
            "keyword1,keyword2",
            "--license",
            "MIT",
            "--readme",
            "README.md",
            "--repository",
            "https://github.com/test/test_lib",
            "--rustversion",
            "1.60.0",
            "--version",
            "0.1.0",
            "--website",
            "https://example.com",
        ]);

    let result = extract_manual_params(
        matches.subcommand_matches("manual").unwrap()
    );
    assert!(result.is_ok());

    let params = result.unwrap();
    assert_eq!(params.name, Some("test_lib".to_string()));
    assert_eq!(params.output, Some("output_dir".to_string()));
    assert_eq!(params.author, Some("John Doe".to_string()));
    assert_eq!(params.build, Some("script.rs".to_string()));
    assert_eq!(
        params.categories,
        Some("category1,category2".to_string())
    );
    assert_eq!(params.description, Some("A test library".to_string()));
    assert_eq!(
        params.documentation,
        Some("https://docs.rs/test_lib".to_string())
    );
    assert_eq!(params.edition, Some("2021".to_string()));
    assert_eq!(params.email, Some("john@example.com".to_string()));
    assert_eq!(
        params.homepage,
        Some("https://example.com".to_string())
    );
    assert_eq!(params.keywords, Some("keyword1,keyword2".to_string()));
    assert_eq!(params.license, Some("MIT".to_string()));
    assert_eq!(params.readme, Some("README.md".to_string()));
    assert_eq!(
        params.repository,
        Some("https://github.com/test/test_lib".to_string())
    );
    assert_eq!(params.rustversion, Some("1.60.0".to_string()));
    assert_eq!(params.version, Some("0.1.0".to_string()));
    assert_eq!(params.website, Some("https://example.com".to_string()));
}
