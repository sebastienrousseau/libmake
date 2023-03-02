#[cfg(test)]
mod tests {
    use std::path::Path;

    use clap::{Arg, Command};
    use libmake::{
        args::process_arguments, generator::generate_files_from_csv,
    };

    #[test]
    fn test_generate_files_from_csv() {
        let csv_file = "./tests/data/mylibrary.csv";
        let result = generate_files_from_csv(csv_file);
        assert!(
            result.is_ok(),
            "generate_files_from_csv was expected to return an Ok result, but it returned an error"
        );
    }

    #[test]
    fn test_process_arguments() {
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
            .get_matches_from(vec!["myapp", "-a", file_path]);
        assert!(matches.contains_id("author"));
        assert_eq!(
            matches.get_one::<String>("author"),
            Some(&file_path.to_string())
        );

        process_arguments(matches);
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

        process_arguments(matches);
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

    #[test]
    fn test_process_arguments_with_csv_empty() {
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
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .default_value("my_library"),
            )
            .arg(Arg::new("readme").short('r').long("readme"))
            .arg(Arg::new("repository").short('R').long("repository"))
            .arg(Arg::new("rustversion").short('V').long("rustversion"))
            .arg(Arg::new("version").short('v').long("version"))
            .arg(Arg::new("website").short('w').long("website"))
            .arg(Arg::new("csv").short('c').long("csv"))
            .get_matches_from(vec!["myapp", "-c", ""]);
        assert!(matches.contains_id("csv"));
        assert_eq!(
            matches.get_one::<String>("csv"),
            Some(&"".to_string())
        );
        process_arguments(matches);
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
}
