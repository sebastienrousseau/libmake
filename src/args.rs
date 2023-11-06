
// Copyright © 2023 LibMake. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use clap::ArgMatches;
use crate::generator::{
    generate_from_json, generate_from_toml, generate_from_yaml,
};
use super::generator::{
    generate_files, generate_from_csv, FileGenerationParams,
};
use std::error::Error;

/// Processes the command line arguments provided to the program.
///
/// # Arguments
///
/// * `matches` - An instance of `clap::ArgMatches` containing the
/// parsed command line arguments.
///
pub fn process_arguments(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // Extracting optional argument values from the parsed matches.
    let author = matches.get_one::<String>("author").cloned();
    let build = matches.get_one::<String>("build").cloned();
    let categories = matches.get_one::<String>("categories").cloned();
    let description = matches.get_one::<String>("description").cloned();
    let documentation = matches.get_one::<String>("documentation").cloned();
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
    if matches.contains_id("csv") {
        let csv_file_path = matches.get_one::<String>("csv").unwrap();
        generate_from_csv(csv_file_path)?;
    } else if let Some(yaml_file_path) = matches.get_one::<String>("yml") {
        generate_from_yaml(yaml_file_path)?;
    } else if let Some(json_file_path) = matches.get_one::<String>("json") {
        generate_from_json(json_file_path)?;
    } else if let Some(toml_file_path) = matches.get_one::<String>("toml") {
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
        println!("\n\nTemplate files generated successfully!");
    } else {
        println!("❌ No arguments provided. Please provide the required arguments to generate the template files.");
    }

    Ok(())
}
