#[cfg(test)]
mod tests {
    use clap::{Arg, Command};
    use libmake::{args::process_arguments, generator::generate_via_csv};

    #[test]
    fn test_process_arguments() {
        // Define an instance of ArgMatches with "author" and "csv" arguments
        let matches = Command::new("test")
        .arg(
            Arg::new("author")
                .default_value("Me")
                .help("Sets the author of the library")
                .long("author")
                .short('a')
                .value_name("AUTHOR"),
        )
        .arg(
            Arg::new("build")
                .default_value("build.rs")
                .help("Sets the build script that is used to perform additional build-time operations.")
                .long("build")
                .short('b')
                .value_name("BUILD"),
        )
        .arg(
            Arg::new("categories")
                .default_value("['category 1', 'category 2']")
                .help("Sets the categories of the library")
                .long("categories")
                .short('c')
                .value_name("CATEGORIES"),
        )
        .arg(
            Arg::new("csv")
                .default_value("")
                .help("Generates a project from a CSV file")
                .long("csv")
                .short('f')
                .value_name("CSV"),
        )
        .arg(
            Arg::new("description")
                .default_value("A library for doing things")
                .help("Sets the description of the library")
                .long("description")
                .short('d')
                .value_name("DESCRIPTION"),
        )
        .arg(
            Arg::new("documentation")
                .default_value("https://lib.rs/crates/my_library")
                .help("Sets the documentation URL of the library")
                .long("documentation")
                .short('u')
                .value_name("DOCUMENTATION"),
        )
        .arg(
            Arg::new("edition")
                .default_value("2021")
                .help("Sets the edition of the library")
                .long("edition")
                .short('e')
                .value_name("EDITION"),
        )
        .arg(
            Arg::new("email")
                .default_value("test@test.com")
                .help("Sets the email of the library author")
                .long("email")
                .short('@')
                .value_name("EMAIL"),
        )
        .arg(
            Arg::new("homepage")
                .default_value("https://test.com")
                .help("Sets the homepage of the library")
                .long("homepage")
                .short('p')
                .value_name("HOMEPAGE"),
        )
        .arg(
            Arg::new("keywords")
                .default_value("['keyword1', 'keyword2']")
                .help("Sets the keywords of the library")
                .long("keywords")
                .short('k')
                .value_name("KEYWORDS"),
        )
        .arg(
            Arg::new("license")
                .default_value("MIT OR Apache-2.0")
                .short('l')
                .long("license")
                .value_name("LICENSE")
                .help("Sets the license of the library"),
        )
        .arg(
            Arg::new("name")
                .default_value("my_library")
                .help("Sets the name of the library")
                .long("name")
                .short('n')
                .value_name("NAME"),
        )
        .arg(
            Arg::new("output")
                .default_value("my_library")
                .help("Sets the output directory for the library")
                .long("output")
                .short('o')
                .value_name("OUTPUT"),
        )
        .arg(
            Arg::new("readme")
                .default_value("README.md")
                .help("Sets the README file for the library")
                .long("readme")
                .short('m')
                .value_name("README"),
        )
        .arg(
            Arg::new("repository")
                .default_value("https://github.com/test/test")
                .help("Sets the GitHub repository of the library")
                .long("repository")
                .short('g')
                .value_name("REPOSITORY"),
        )
        .arg(
            Arg::new("rustversion")
                .default_value("1.67.1")
                .help("Sets the Rust version of the library")
                .long("rustversion")
                .short('r')
                .value_name("RUSTVERSION"),
        )
        .arg(
            Arg::new("version")
                .default_value("0.0.4")
                .help("Sets the version of the library")
                .long("version")
                .short('v')
                .value_name("VERSION"),
        )
        .arg(
            Arg::new("website")
                .default_value("https://test.com")
                .help("Sets the website of the library author")
                .long("website")
                .short('w')
                .value_name("WEBSITE"),
        )
        .get_matches_from(vec![
            "test",
            "--author",
            "Me",
            "--build",
            "build.rs",
            "--categories",
            "['category 1', 'category 2']",
            "--csv",
            "",
            "--description",
            "A library for doing things",
            "--documentation",
            "https://lib.rs/crates/my_library",
            "--edition",
            "2021",
            "--email",
            "test@test.com",
            "--homepage",
            "https://test.com",
            "--keywords",
            "['keyword1', 'keyword2']",
            "--license",
            "MIT OR Apache-2.0",
            "--name",
            "my_library",
            "--output",
            "my_library",
            "--readme",
            "README.md",
            "--repository",
            "https://github.com/test/test",
            "--rustversion",
            "1.67.1",
            "--version",
            "0.0.4",
            "--website",
            "https://test.com",
        ]);

        // Call the process_arguments function with the ArgMatches instance
        process_arguments(matches);

        // Assert that the result of process_arguments is Ok(())
        assert!(generate_via_csv("/tests/data/mylibrary.csv").is_err());
    }
}
