use clap::{Arg, ArgMatches, Command, Error};

/// Builds and returns a set of command-line arguments using the Clap
/// library.
///
/// # Arguments
///
/// None
///
/// # Returns
///
/// * `Result<ArgMatches, Error>` - A struct containing the parsed
/// command-line arguments and their values, or an error if the
/// arguments could not be parsed.
///
/// # Examples
///
/// ```
/// use libmake::cli;
/// let matches = cli::build_cli().unwrap();
/// ```
pub fn build_cli() -> Result<ArgMatches, Error> {
    let matches = Command::new("My Library")
        .author("Your Name")
        .about(
            "A Rust library generator that helps create high-quality Rust libraries quickly and easily.",
        )
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
                .help("Sets the CSV file to use for generating the library")
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
                .default_value("0.1.0")
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
        .after_help(
            "Longer explanation to appear after the options when \
        displaying the help information from --help or -h",
        )
        .get_matches();
    Ok(matches)
}
