// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

// use std::path::Path;

use clap::{error::Error, Arg, ArgMatches, Command};

/// Constructs the command-line interface for the application using Clap,
/// including all necessary arguments.
///
/// # Examples
///
/// ```
/// use libmake::cli;
/// let matches = cli::build().expect("CLI parsing failed");
/// ```
///
/// # Errors
///
/// This function will return an error if the command-line argument parsing fails.
///
pub fn build() -> Result<ArgMatches, Error> {
    let manual_args = vec![
            create_arg_info("author", Some("Me"), "Sets the author of the library", 'a', "author", "AUTHOR"),
            create_arg_info("build", Some("build.rs"), "Sets the build script that is used to perform additional build-time operations.", 'b', "build", "BUILD"),
            create_arg_info("categories", Some("['category 1', 'category 2']"), "Sets the categories of the library", 'c', "categories", "CATEGORIES"),
            create_arg_info("description", Some("A library for doing things"), "Sets the description of the library", 'd', "description", "DESCRIPTION"),
            create_arg_info("documentation", Some("https://lib.rs/crates/my_library"), "Sets the documentation URL of the library", 'u', "documentation", "DOCUMENTATION"),
            create_arg_info("edition", Some("2021"), "Sets the edition of the library", 'e', "edition", "EDITION"),
            create_arg_info("email", Some("test@test.com"), "Sets the email of the library author", '@', "email", "EMAIL"),
            create_arg_info("homepage", Some("https://test.com"), "Sets the homepage of the library", 'p', "homepage", "HOMEPAGE"),
            create_arg_info("keywords", Some("['keyword1', 'keyword2']"), "Sets the keywords of the library", 'k', "keywords", "KEYWORDS"),
            create_arg_info("license", Some("MIT OR Apache-2.0"), "Sets the license of the library", 'l', "license", "LICENSE"),
            create_arg_info("name", Some("my_library"), "Sets the name of the library", 'n', "name", "NAME"),
            create_arg_info("output", Some("my_library"), "Sets the output directory for the library", 'o', "output", "OUTPUT"),
            create_arg_info("readme", Some("README.md"), "Sets the README file for the library", 'm', "readme", "README"),
            create_arg_info("repository", Some("https://github.com/example/my_library"), "Sets the repository URL of the library", 'g', "repository", "REPOSITORY"),
            create_arg_info("rustversion", Some("1.75.0"), "Sets the Rust version of the library", 'r', "rustversion", "RUSTVERSION"),
            create_arg_info("version", Some("0.2.2"), "Sets the version of the library", 'v', "version", "VERSION"),
            create_arg_info("website", Some("https://test.com"), "Sets the website of the library author", 'w', "website", "WEBSITE"),
        ];

    let file_args = vec![
        create_arg_info(
            "csv",
            Some(""),
            "Sets the CSV file to use for generating the library",
            'x',
            "csv",
            "CSV",
        ),
        create_arg_info(
            "ini",
            Some(""),
            "Sets the INI file to use for generating the library",
            'i',
            "ini",
            "INI",
        ),
        create_arg_info(
            "json",
            Some(""),
            "Sets the JSON file to use for generating the library",
            'j',
            "json",
            "JSON",
        ),
        create_arg_info(
            "yaml",
            Some(""),
            "Sets the YAML file to use for generating the library",
            'y',
            "yaml",
            "YAML",
        ),
        create_arg_info(
            "toml",
            Some(""),
            "Sets the TOML file to use for generating the library",
            't',
            "toml",
            "TOML",
        ),
    ];

    let command = Command::new("My Library")
        .author("Sebastien Rousseau")
        .about("A Rust library generator that helps create high-quality Rust libraries quickly and easily.")
        .after_help("By default, if no arguments are passed in, the CLI will throw an error. To see a list of available actions, run `--help`.")
        .subcommand(
            Command::new("manual")
                .about("Set library information manually")
                .args(manual_args.into_iter().map(create_arg).collect::<Vec<Arg>>()),
        )
        .subcommand(
            Command::new("file")
                .about("Set library information from a file")
                .args(file_args.into_iter().map(create_arg).collect::<Vec<Arg>>()),
        );

    // Assuming validate_args is a custom function that you have implemented
    let matches = command.clone().try_get_matches()?;

    Ok(matches)
}

/// Helper function to create a command-line argument.
const fn create_arg_info(
    name: &'static str,
    default: Option<&'static str>,
    help: &'static str,
    short: char,
    long: &'static str,
    value_name: &'static str,
) -> (
    &'static str,
    Option<&'static str>,
    &'static str,
    char,
    &'static str,
    &'static str,
) {
    (name, default, help, short, long, value_name)
}

/// Creates an argument based on provided information.
pub fn create_arg(
    arg_info: (
        &'static str,
        Option<&'static str>,
        &'static str,
        char,
        &'static str,
        &'static str,
    ),
) -> Arg {
    let (name, default, help, short, long, value_name) = arg_info;

    let mut arg = Arg::new(name)
        .help(help)
        .short(short)
        .long(long)
        .value_name(value_name);

    if let Some(default_value) = default {
        arg = arg.default_value(default_value);
    }
    arg
}
