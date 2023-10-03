// Copyright © 2023 LibMake. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use clap::ArgMatches;

use crate::generator::{
    generate_from_json, generate_from_toml, generate_from_yaml,
};

use super::generator::{
    generate_files, generate_from_csv, FileGenerationParams,
};

/// Processes the command line arguments provided to the program.
///
/// # Arguments
///
/// * `matches` - An instance of `clap::ArgMatches` containing the
/// parsed command line arguments.
///
pub fn process_arguments(matches: &ArgMatches) {
    // Extracting optional argument values from the parsed matches.
    let author = matches.get_one::<String>("author");
    let build = matches.get_one::<String>("build");
    let categories = matches.get_one::<String>("categories");
    let description = matches.get_one::<String>("description");
    let documentation = matches.get_one::<String>("documentation");
    let edition = matches.get_one::<String>("edition");
    let email = matches.get_one::<String>("email");
    let homepage = matches.get_one::<String>("homepage");
    let keywords = matches.get_one::<String>("keywords");
    let license = matches.get_one::<String>("license");
    let name = matches.get_one::<String>("name");
    let output = matches.get_one::<String>("output");
    let readme = matches.get_one::<String>("readme");
    let repository = matches.get_one::<String>("repository");
    let rustversion = matches.get_one::<String>("rustversion");
    let version = matches.get_one::<String>("version");
    let website = matches.get_one::<String>("website");

    // Check which subcommand was used and perform the corresponding action.
    if matches.contains_id("csv") {
        let csv_file_path = matches.get_one::<String>("csv").unwrap();
        generate_from_csv(csv_file_path)
            .expect("Failed to generate the template files");
    } else if let Some(yaml_file_path) =
        matches.get_one::<String>("yml")
    {
        generate_from_yaml(yaml_file_path)
            .expect("Failed to generate the template files");
    } else if let Some(json_file_path) =
        matches.get_one::<String>("json")
    {
        generate_from_json(json_file_path)
            .expect("Failed to generate the template files");
    } else if let Some(toml_file_path) =
        matches.get_one::<String>("toml")
    {
        generate_from_toml(toml_file_path)
            .expect("Failed to generate the template files");
    } else if !matches.args_present() {
        // If no subcommand is used and there are additional arguments,
        // create a parameter struct and generate files.
        let params = FileGenerationParams {
            author: author.cloned(),
            build: build.cloned(),
            categories: categories.cloned(),
            description: description.cloned(),
            documentation: documentation.cloned(),
            edition: edition.cloned(),
            email: email.cloned(),
            homepage: homepage.cloned(),
            keywords: keywords.cloned(),
            license: license.cloned(),
            name: name.cloned(),
            output: output.cloned(),
            readme: readme.cloned(),
            repository: repository.cloned(),
            rustversion: rustversion.cloned(),
            version: version.cloned(),
            website: website.cloned(),
        };
        generate_files(params)
            .expect("Failed to generate the template files");
        println!("\n\nTemplate files generated successfully!");
    } else {
        println!("❌ No arguments provided. Please provide the required arguments to generate the template files.");
    }
}
