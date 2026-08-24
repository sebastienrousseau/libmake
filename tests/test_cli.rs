//! Integration tests: test cli.

#[cfg(test)]
mod tests {
    use libmake::cli::{build_from, create_arg};

    // These use `build_from` rather than `build`. `build` parses
    // `std::env::args_os()`, which inside a test binary belongs to the
    // test harness — so `cargo test -- --nocapture` made clap reject
    // `--nocapture` and this assertion failed, even though nothing about
    // the CLI had changed.
    #[test]
    fn test_build_no_subcommand() {
        let matches = build_from(["libmake"]);
        assert!(matches.is_ok());
        assert!(matches.unwrap().subcommand_name().is_none());
    }

    #[test]
    fn test_build_manual_subcommand() {
        let matches =
            build_from(["libmake", "manual", "--name", "demo"]);
        assert!(matches.is_ok());
        let matches = matches.unwrap();
        assert_eq!(matches.subcommand_name(), Some("manual"));
        let sub = matches.subcommand_matches("manual").unwrap();
        assert_eq!(
            sub.get_one::<String>("name").map(String::as_str),
            Some("demo")
        );
    }

    #[test]
    fn test_build_file_subcommand() {
        let matches =
            build_from(["libmake", "file", "--toml", "cfg.toml"]);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().subcommand_name(), Some("file"));
    }

    #[test]
    fn test_build_rejects_unknown_argument() {
        // The exact shape that broke the suite under `--nocapture`.
        let matches = build_from(["libmake", "--nocapture"]);
        assert!(matches.is_err());
    }

    #[test]
    fn test_create_arg() {
        // Test case 1: Create an argument with all fields
        let arg_info = (
            "name",
            Some("default"),
            "help message",
            'n',
            "name",
            "NAME",
        );
        let arg = create_arg(arg_info);
        assert_eq!(arg.get_id(), "name");
        assert_eq!(arg.get_help().unwrap().to_string(), "help message");
        assert_eq!(arg.get_short().unwrap(), 'n');
        assert_eq!(arg.get_long().unwrap(), "name");

        // Test case 2: Create an argument without a default value
        let arg_info =
            ("name", None, "help message", 'n', "name", "NAME");
        let arg = create_arg(arg_info);
        assert_eq!(arg.get_id(), "name");
        assert_eq!(arg.get_help().unwrap().to_string(), "help message");
        assert_eq!(arg.get_short().unwrap(), 'n');
        assert_eq!(arg.get_long().unwrap(), "name");

        // Test case 3: Create an argument with only required fields
        let arg_info = ("name", None, "", 'n', "name", "NAME");
        let arg = create_arg(arg_info);
        assert_eq!(arg.get_id(), "name");
        assert_eq!(arg.get_short().unwrap(), 'n');
        assert_eq!(arg.get_long().unwrap(), "name");

        // Test case 4: Create an argument with a multi-word long flag
        let arg_info =
            ("name", None, "help message", 'n', "long-flag", "NAME");
        let arg = create_arg(arg_info);
        assert_eq!(arg.get_long().unwrap(), "long-flag");

        // Test case 5: Argument with an empty help message
        let arg_info = (
            "name", None, "", // Empty help message
            'n', "name", "NAME",
        );
        let arg = create_arg(arg_info);

        // Updated assertion
        assert!(arg.get_help().is_some()); // Help should be set, even if empty

        // Optionally, depending on the implementation of StyledStr:
        assert_eq!(arg.get_help().unwrap().to_string(), ""); // Check if the help string is indeed empty

        // Test case 6: Long argument name
        let arg_info = (
            "very-long-argument-name",
            None,
            "help",
            'v',
            "very-long-argument-name",
            "NAME",
        );
        let arg = create_arg(arg_info);
        assert_eq!(arg.get_id(), "very-long-argument-name");
    }
}
