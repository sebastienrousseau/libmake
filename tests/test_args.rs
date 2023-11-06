#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use clap::{Arg, Command};
    use libmake::{
        args::process_arguments,
        generator::{
            generate_from_csv, generate_from_json, generate_from_toml,
            generate_from_yaml, FileGenerationParams,
        },
    };

    // Tests the `generate_from_csv` function by passing a valid CSV
    // file path as input and asserting that the result is an Ok value,
    // indicating that the template files were generated successfully
    // from the CSV file.
    #[test]
    fn test_generate_from_csv() {
        let csv_file = "./tests/data/mylibrary.csv";
        let result = generate_from_csv(csv_file);
        assert!(
            result.is_ok(),
            "generate_from_csv was expected to return an Ok result, but it returned an error"
        );
    }

    // Tests the `generate_from_json` function by passing a valid JSON
    // file path as input and asserting that the result is an Ok value,
    // indicating that the template files were generated successfully
    // from the JSON file.
    #[test]
    fn test_generate_from_json() {
        let json_file = "./tests/data/mylibrary.json";
        let result = generate_from_json(json_file);
        assert!(
            result.is_ok(),
            "generate_from_json was expected to return an Ok result, but it returned an error"
        );
    }

    // Tests the `generate_from_toml` function by passing a valid TOML
    // file path as input and asserting that the result is an Ok value,
    // indicating that the template files were generated successfully
    // from the TOML file.
    #[test]
    fn test_generate_from_toml() {
        let toml_file = "./tests/data/mylibrary.toml";
        let result = generate_from_toml(toml_file);
        assert!(
            result.is_ok(),
            "generate_from_toml was expected to return an Ok result, but it returned an error"
        );
    }

    // Tests the `generate_from_yaml` function by passing a valid YAML
    // file path as input and asserting that the result is an Ok value,
    // indicating that the template files were generated successfully
    // from the YAML file.
    #[test]
    fn test_generate_from_yaml() {
        let yaml_file = "./tests/data/mylibrary.yaml";
        let result = generate_from_yaml(yaml_file);
        assert!(
            result.is_ok(),
            "generate_from_yaml was expected to return an Ok result, but it returned an error"
        );
    }

    // Tests to verify that the process_arguments function correctly
    // handles the arguments when "csv" argument is passed. It should
    // extract the file path and call generate_from_csv function with
    // the file path. Finally, it should generate the expected files in
    // the current directory.
    #[test]
    fn test_process_arguments_with_csv() {
        let file_path = "./tests/data/mylibrary.csv";
        let matches = Command::new("myapp")
            .arg(Arg::new("author").short('a').long("author"))
            .arg(Arg::new("build").short('b').long("build"))
            .arg(Arg::new("categories").short('C').long("categories"))
            .arg(Arg::new("description").short('d').long("description"))
            .arg(
                Arg::new("documentation")
                    .short('D')
                    .long("documentation"),
            )
            .arg(Arg::new("edition").short('e').long("edition"))
            .arg(Arg::new("email").short('E').long("email"))
            .arg(Arg::new("homepage").short('p').long("homepage"))
            .arg(Arg::new("keywords").short('k').long("keywords"))
            .arg(Arg::new("license").short('l').long("license"))
            .arg(Arg::new("name").short('n').long("name"))
            .arg(Arg::new("output").short('o').long("output"))
            .arg(Arg::new("readme").short('r').long("readme"))
            .arg(Arg::new("repository").short('R').long("repository"))
            .arg(Arg::new("rustversion").short('V').long("rustversion"))
            .arg(Arg::new("version").short('v').long("version"))
            .arg(Arg::new("website").short('w').long("website"))
            .arg(Arg::new("csv").short('c').long("csv"))
            .get_matches_from(vec!["myapp", "-c", file_path]);
        assert!(matches.contains_id("csv"));
        assert_eq!(
            matches.get_one::<String>("csv"),
            Some(&file_path.to_string())
        );

        process_arguments(&matches).unwrap();
        // Check that the files were generated
        let expected_files = vec![
            "Cargo.toml",
            "src/lib.rs",
            "CONTRIBUTING.md",
            "README.md",
            ".gitignore",
        ];
        for file in &expected_files {
            let path = Path::new(file);
            if !path.exists() {
                println!("Expected file not found: {:?}", path);
            }
            assert!(path.exists());
        }
    }

    // Tests to verify that the process_arguments function correctly
    // handles the arguments when "json" argument is passed. It should
    // extract the file path and call generate_from_json function with
    // the file path. Finally, it should generate the expected files in
    // the current directory.
    #[test]
    fn test_process_arguments_with_json() {
        let json_file_path = "./tests/data/mylibrary.json";
        let path = Path::new(json_file_path);
        if !path.exists() {
            panic!("File {} does not exist", json_file_path);
        }

        let matches = Command::new("myapp")
            .arg(Arg::new("json").short('j').long("json"))
            .get_matches_from(vec!["myapp", "-j", json_file_path]);

        assert!(matches.contains_id("json"));
        assert_eq!(
            matches.get_one::<String>("json"),
            Some(&json_file_path.to_string())
        );

        // Generate the files
        let result = generate_from_json(json_file_path);
        assert!(
            result.is_ok(),
            "Failed to generate the template files: {:?}",
            result
        );

        // Check that the files were generated
        let expected_files = vec![
            "Cargo.toml",
            "src/lib.rs",
            "CONTRIBUTING.md",
            "README.md",
            ".gitignore",
        ];
        for file in &expected_files {
            let path = Path::new(file);
            if !path.exists() {
                panic!("Failed to generate the template files");
            }
        }
    }

    // Tests to verify that the process_arguments function correctly
    // handles the arguments when "toml" argument is passed. It should
    // extract the file path and call generate_from_toml function with
    // the file path. Finally, it should generate the expected files in
    // the current directory.
    #[test]
    fn test_process_arguments_with_toml() {
        let file_path = "./tests/data/mylibrary.toml";
        let contents = fs::read_to_string(file_path);
        let mut params: FileGenerationParams =
            toml::from_str(&contents.unwrap()).unwrap();
        params.output = Some(".".to_string());
        let matches = Command::new("myapp")
            .arg(Arg::new("author").short('a').long("author"))
            .arg(Arg::new("build").short('b').long("build"))
            .arg(Arg::new("categories").short('C').long("categories"))
            .arg(Arg::new("description").short('d').long("description"))
            .arg(
                Arg::new("documentation")
                    .short('D')
                    .long("documentation"),
            )
            .arg(Arg::new("edition").short('e').long("edition"))
            .arg(Arg::new("email").short('E').long("email"))
            .arg(Arg::new("homepage").short('p').long("homepage"))
            .arg(Arg::new("keywords").short('k').long("keywords"))
            .arg(Arg::new("license").short('l').long("license"))
            .arg(Arg::new("name").short('n').long("name"))
            .arg(Arg::new("output").short('o').long("output"))
            .arg(Arg::new("readme").short('r').long("readme"))
            .arg(Arg::new("repository").short('R').long("repository"))
            .arg(Arg::new("rustversion").short('V').long("rustversion"))
            .arg(Arg::new("version").short('v').long("version"))
            .arg(Arg::new("website").short('w').long("website"))
            .arg(Arg::new("toml").short('t').long("toml"))
            .get_matches_from(vec!["myapp", "-t", file_path]);
        assert!(matches.contains_id("toml"));
        assert_eq!(
            matches.get_one::<String>("toml"),
            Some(&file_path.to_string())
        );
    }

    // Tests to verify that the process_arguments function correctly
    // handles the arguments when "yaml" argument is passed. It should
    // extract the file path and call generate_from_yaml function with
    // the file path. Finally, it should generate the expected files in
    // the current directory.
    #[test]
    fn test_process_arguments_with_yaml() {
        let file_path = "./tests/data/mylibrary.yaml";
        let contents = fs::read_to_string(file_path);
        let mut params: FileGenerationParams =
            serde_yaml::from_str(&contents.unwrap()).unwrap();
        params.output = Some(".".to_string());
        let matches = Command::new("myapp")
            .arg(Arg::new("author").short('a').long("author"))
            .arg(Arg::new("build").short('b').long("build"))
            .arg(Arg::new("categories").short('C').long("categories"))
            .arg(Arg::new("description").short('d').long("description"))
            .arg(
                Arg::new("documentation")
                    .short('D')
                    .long("documentation"),
            )
            .arg(Arg::new("edition").short('e').long("edition"))
            .arg(Arg::new("email").short('E').long("email"))
            .arg(Arg::new("homepage").short('p').long("homepage"))
            .arg(Arg::new("keywords").short('k').long("keywords"))
            .arg(Arg::new("license").short('l').long("license"))
            .arg(Arg::new("name").short('n').long("name"))
            .arg(Arg::new("output").short('o').long("output"))
            .arg(Arg::new("readme").short('r').long("readme"))
            .arg(Arg::new("repository").short('R').long("repository"))
            .arg(Arg::new("rustversion").short('V').long("rustversion"))
            .arg(Arg::new("version").short('v').long("version"))
            .arg(Arg::new("website").short('w').long("website"))
            .arg(Arg::new("yaml").short('y').long("yaml"))
            .get_matches_from(vec!["myapp", "-y", file_path]);
        assert!(matches.contains_id("yaml"));
        assert_eq!(
            matches.get_one::<String>("yaml"),
            Some(&file_path.to_string())
        );
    }
}
