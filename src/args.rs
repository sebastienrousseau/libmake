// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

use super::generator::{
    generate_files, generate_from_csv, FileGenerationParams,
};
use crate::generator::{
    generate_from_json, generate_from_toml, generate_from_yaml,
};
use clap::ArgMatches;
use dtt::DateTime;
use rlg::{log_format::LogFormat, log_level::LogLevel, macro_log};
use std::error::Error;

/// Processes the command line arguments provided to the program.
///
/// # Arguments
///
/// * `matches` - An instance of `clap::ArgMatches` containing the
/// parsed command line arguments.
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - If a specified file path for a subcommand (CSV, YAML, JSON, TOML) is invalid or the file cannot be read.
/// - If there is an error in parsing the file contents into the respective file format (CSV, YAML, JSON, TOML).
/// - If there is an error in generating files based on the parameters derived from the command line arguments.
///
/// # Panics
///
/// This function will panic if a required command line argument is not provided.
/// For example, it will panic if the `csv` subcommand is used without specifying
/// a CSV file path.
///
pub fn process_arguments(
    matches: &ArgMatches,
) -> Result<(), Box<dyn Error>> {
    // Generating a new date and time and a new UUID.
    let date = DateTime::new();
    let iso = date.iso_8601;
    let uuid = uuid::Uuid::new_v4().to_string();

    // Extracting optional argument values from the parsed matches.
    let author = matches.get_one::<String>("author").cloned();
    let build = matches.get_one::<String>("build").cloned();
    let categories = matches.get_one::<String>("categories").cloned();
    let description = matches.get_one::<String>("description").cloned();
    let documentation =
        matches.get_one::<String>("documentation").cloned();
    let edition = matches.get_one::<String>("edition").cloned();
    let email = matches.get_one::<String>("email").cloned();
    let homepage = matches.get_one::<String>("homepage").cloned();
    let keywords = matches.get_one::<String>("keywords").cloned();
    let license = matches.get_one::<String>("license").cloned();
    let name = matches.get_one::<String>("name").cloned();
    let output = matches.get_one::<String>("output").cloned();
    let readme = matches.get_one::<String>("readme").cloned();
    let repository = matches.get_one::<String>("repository").cloned();
    let rustversion = matches.get_one::<String>("rustversion").cloned();
    let version = matches.get_one::<String>("version").cloned();
    let website = matches.get_one::<String>("website").cloned();

    // Check which subcommand was used and perform the corresponding action.
    if let Some(csv_file_path) = matches.get_one::<String>("csv") {
        generate_from_csv(csv_file_path)?;
    } else if let Some(yaml_file_path) =
        matches.get_one::<String>("yml")
    {
        generate_from_yaml(yaml_file_path)?;
    } else if let Some(json_file_path) =
        matches.get_one::<String>("json")
    {
        generate_from_json(json_file_path)?;
    } else if let Some(toml_file_path) =
        matches.get_one::<String>("toml")
    {
        generate_from_toml(toml_file_path)?;
    } else if !matches.args_present() {
        // If no subcommand is used and there are additional arguments,
        // create a parameter struct and generate files.
        let params = FileGenerationParams {
            author,
            build,
            categories,
            description,
            documentation,
            edition,
            email,
            homepage,
            keywords,
            license,
            name,
            output,
            readme,
            repository,
            rustversion,
            version,
            website,
        };
        generate_files(params)?;
        macro_log!(
            &uuid,
            &iso,
            &LogLevel::INFO,
            "args",
            "Template files generated successfully!",
            &LogFormat::CLF
        );
    } else {
        macro_log!(
            &uuid,
            &iso,
            &LogLevel::ERROR,
            "args",
            "No arguments provided. Please provide the required arguments to generate the template files.",
            &LogFormat::CLF
        );
    }

    Ok(())
}
