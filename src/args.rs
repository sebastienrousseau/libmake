// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2023-2024 LibMake. All rights reserved.

use super::{
    extract_param, generate_file, generator::generate_files,
    generators::csv::generate_from_csv,
    generators::ini::generate_from_ini,
    generators::json::generate_from_json,
    generators::toml::generate_from_toml,
    generators::yaml::generate_from_yaml,
    models::model_params::FileGenerationParams,
};
use clap::ArgMatches;
use regex::Regex;
use std::error::Error;

/// Processes the command line arguments provided to the program.
///
/// This function takes in command-line arguments parsed by `clap` and performs actions based on the arguments provided.
///
/// # Arguments
///
/// * `matches` - An instance of `clap::ArgMatches` containing the parsed command line arguments.
///
/// # Errors
///
/// This function will return an error if there is an issue with processing the command line arguments or generating files.
pub fn process_arguments(
    matches: &ArgMatches,
) -> Result<(), Box<dyn Error>> {
    match matches.subcommand() {
        Some(("file", file_matches)) => {
            if let Some(value) = file_matches.get_one::<String>("csv") {
                generate_file!("csv", value, generate_from_csv);
            }
            if let Some(value) = file_matches.get_one::<String>("ini") {
                generate_file!("ini", value, generate_from_ini);
            }
            if let Some(value) = file_matches.get_one::<String>("json")
            {
                generate_file!("json", value, generate_from_json);
            }
            if let Some(value) = file_matches.get_one::<String>("yaml")
            {
                generate_file!("yaml", value, generate_from_yaml);
            }
            if let Some(value) = file_matches.get_one::<String>("toml")
            {
                generate_file!("toml", value, generate_from_toml);
            }
        }
        Some(("manual", manual_matches)) => {
            let params = extract_manual_params(manual_matches)?;
            if let Err(err) = generate_files(params) {
                eprintln!("Error generating template files: {}", err);
            } else {
                println!("Template files generated successfully!");
            }
        }
        _ => {
            eprintln!("No valid subcommand was used. Please use '--help' for usage information.");
        }
    }

    Ok(())
}

/// Extracts the parameters for manual generation from command line arguments.
pub fn extract_manual_params(
    matches: &ArgMatches,
) -> Result<FileGenerationParams, Box<dyn Error>> {
    let params = FileGenerationParams {
        author: extract_param!(matches, "author"),
        build: extract_param!(matches, "build"),
        categories: extract_param!(matches, "categories"),
        description: extract_param!(matches, "description"),
        documentation: extract_param!(matches, "documentation"),
        edition: extract_param!(matches, "edition"),
        email: extract_param!(matches, "email"),
        homepage: extract_param!(matches, "homepage"),
        keywords: extract_param!(matches, "keywords"),
        license: extract_param!(matches, "license"),
        name: extract_param!(matches, "name"),
        output: extract_param!(matches, "output"),
        readme: extract_param!(matches, "readme"),
        repository: extract_param!(matches, "repository"),
        rustversion: extract_param!(matches, "rustversion"),
        version: extract_param!(matches, "version"),
        website: extract_param!(matches, "website"),
    };
    validate_params(&params)?;
    Ok(params)
}

/// Validates the manual generation parameters.
pub fn validate_params(
    params: &FileGenerationParams,
) -> Result<(), Box<dyn Error>> {
    if params.name.is_none() {
        return Err("The name of the library is required for manual generation.".into());
    }

    if params.output.is_none() {
        return Err(
            "The output directory is required for manual generation."
                .into(),
        );
    }

    if let Some(edition) = &params.edition {
        let valid_editions = ["2015", "2018", "2021"];
        if !valid_editions.contains(&edition.as_str()) {
            return Err(format!(
                "Invalid edition: {}. Supported editions are: {}.",
                edition,
                valid_editions.join(", ")
            )
            .into());
        }
    }

    if let Some(rustversion) = &params.rustversion {
        let version_regex = Regex::new(r"^1\.\d+\.\d+$").unwrap();
        if !version_regex.is_match(rustversion) {
            return Err(format!(
                "Invalid Rust version: {}. Rust version should be in the format '1.x.y'.",
                rustversion
            )
            .into());
        }
    }

    if let Some(email) = &params.email {
        let email_regex = Regex::new(
            r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$",
        )
        .unwrap();
        if !email_regex.is_match(email) {
            return Err(
                format!("Invalid email address: {}.", email).into()
            );
        }
    }

    if let Some(repository) = &params.repository {
        let repo_regex =
            Regex::new(r"^(https://|git://|ssh://|git@).+\.git$")
                .unwrap();
        if !repo_regex.is_match(repository) {
            return Err(format!(
                "Invalid repository URL: {}. Repository URL should be a valid Git URL.",
                repository
            )
            .into());
        }
    }

    if let Some(homepage) = &params.homepage {
        let url_regex = Regex::new(r"^(http://|https://).+$").unwrap();
        if !url_regex.is_match(homepage) {
            return Err(format!(
                "Invalid homepage URL: {}. Homepage URL should start with 'http://' or 'https://'.",
                homepage
            )
            .into());
        }
    }

    if let Some(documentation) = &params.documentation {
        let url_regex = Regex::new(r"^(http://|https://).+$").unwrap();
        if !url_regex.is_match(documentation) {
            return Err(format!(
                "Invalid documentation URL: {}. Documentation URL should start with 'http://' or 'https://'.",
                documentation
            )
            .into());
        }
    }

    Ok(())
}
