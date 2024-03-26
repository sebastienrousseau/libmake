// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

use super::{
    extract_param, generator::generate_files,
    generators::csv::generate_from_csv,
    generators::ini::generate_from_ini,
    generators::json::generate_from_json,
    generators::toml::generate_from_toml,
    generators::yaml::generate_from_yaml,
    models::model_params::FileGenerationParams,
};
use clap::ArgMatches;
use std::error::Error;

/// Processes the command line arguments provided to the program.
///
/// This function takes in command-line arguments parsed by `clap` and performs actions based on the arguments provided.
///
/// # Arguments
///
/// * `matches` - An instance of `clap::ArgMatches` containing the
/// parsed command line arguments.
///
/// # Errors
///
/// This function will return an error if there is an issue with processing the command line arguments or generating files.
///
/// # Panics
///
/// This function may panic if a required command line argument is not provided.
pub fn process_arguments(
    matches: &ArgMatches,
) -> Result<(), Box<dyn Error>> {
    match matches.subcommand() {
        Some(("file", file_matches)) => {
            let file_types = ["csv", "ini", "json", "yaml", "toml"];

            for file_type in file_types.iter() {
                if let Some(value) =
                    file_matches.get_one::<String>(file_type)
                {
                    match *file_type {
                        "csv" if !value.trim().is_empty() => {
                            generate_from_csv(value)?
                        }
                        "ini" if !value.trim().is_empty() => {
                            generate_from_ini(value)?
                        }
                        "json" if !value.trim().is_empty() => {
                            generate_from_json(value)?
                        }
                        "yaml" if !value.trim().is_empty() => {
                            generate_from_yaml(value)?
                        }
                        "toml" if !value.trim().is_empty() => {
                            generate_from_toml(value)?
                        }
                        _ => {}
                    }
                }
            }
        }
        Some(("manual", manual_matches)) => {
            let params = extract_manual_params(manual_matches)?;
            generate_files(params)?;
            println!("Template files generated successfully!");
        }
        _ => {
            eprintln!("No valid subcommand was used. Please use '--help' for usage information.");
            std::process::exit(1);
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
        if edition != "2015" && edition != "2018" && edition != "2021" {
            return Err(format!("Invalid edition: {}. Supported editions are 2015, 2018, and 2021.", edition).into());
        }
    }

    if let Some(rustversion) = &params.rustversion {
        if !rustversion.starts_with("1.") {
            return Err(format!("Invalid Rust version: {}. Rust version should start with '1.'.", rustversion).into());
        }
    }

    if let Some(email) = &params.email {
        if !email.contains('@') {
            return Err(format!("Invalid email address: {}. Email address should contain '@'.", email).into());
        }
    }

    if let Some(repository) = &params.repository {
        if !repository.starts_with("https://")
            && !repository.starts_with("git://")
        {
            return Err(format!("Invalid repository URL: {}. Repository URL should start with 'https://' or 'git://'.", repository).into());
        }
    }

    if let Some(homepage) = &params.homepage {
        if !homepage.starts_with("http://")
            && !homepage.starts_with("https://")
        {
            return Err(format!("Invalid homepage URL: {}. Homepage URL should start with 'http://' or 'https://'.", homepage).into());
        }
    }

    if let Some(documentation) = &params.documentation {
        if !documentation.starts_with("http://")
            && !documentation.starts_with("https://")
        {
            return Err(format!("Invalid documentation URL: {}. Documentation URL should start with 'http://' or 'https://'.", documentation).into());
        }
    }

    Ok(())
}
