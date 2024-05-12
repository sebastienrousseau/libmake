// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2023-2024 LibMake. All rights reserved.

use serde::{Deserialize, Serialize};

fn deserialize_name<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let name = String::deserialize(deserializer)?;
    Ok(Some(name.trim_matches('"').to_string()))
}

/// Structure for holding the parameters for generating the project files.
///
/// # Description
///
/// * The `output` directory is the directory where the project files will be created.
/// * The other parameters are optional and will be used to replace the placeholders in the template files.
/// * The template files are located in the template directory of the project.
/// * The template files are copied to the output directory and the placeholders are replaced with the values of the parameters.
///
#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct FileGenerationParams {
    /// The author of the project (optional).
    pub author: Option<String>,
    /// The build command to be used for building the project (optional).
    pub build: Option<String>,
    /// The categories that the project belongs to (optional).
    pub categories: Option<String>,
    /// A short description of the project (optional).
    pub description: Option<String>,
    /// The documentation URL of the project (optional).
    pub documentation: Option<String>,
    /// The edition of the project (optional).
    pub edition: Option<String>,
    /// The email address of the author (optional).
    pub email: Option<String>,
    /// The homepage of the project (optional).
    pub homepage: Option<String>,
    /// Keywords that describe the project (optional).
    pub keywords: Option<String>,
    /// The license under which the project is released (optional).
    pub license: Option<String>,
    #[serde(deserialize_with = "deserialize_name")]
    /// The name of the project (optional).
    pub name: Option<String>,
    /// The output directory where the project files will be created (optional).
    pub output: Option<String>,
    /// The name of the readme file (optional).
    pub readme: Option<String>,
    #[serde(deserialize_with = "deserialize_name")]
    /// The URL of the project's repository (optional).
    pub repository: Option<String>,
    /// The minimum Rust version required by the project (optional).
    pub rustversion: Option<String>,
    /// The initial version of the project (optional).
    pub version: Option<String>,
    /// The website of the project (optional).
    pub website: Option<String>,
}

impl FileGenerationParams {
    /// Creates a default instance with default values for all fields.
    /// Fields that are truly optional without a default are initialized as `None`.
    pub fn default_params() -> Self {
        Self {
            author: Some("John Smith".to_string()),
            build: Some("build.rs".to_string()),
            categories: Some(
                "[\"category1\",\"category2\",\"category3\"]"
                    .to_string(),
            ),
            description: Some(
                "A Rust library for doing cool things".to_string(),
            ),
            documentation: Some(
                "https://docs.rs/my_library".to_string(),
            ),
            edition: Some("2021".to_string()),
            email: Some("john.smith@example.com".to_string()),
            homepage: Some("https://my_library.rs".to_string()),
            keywords: Some(
                "[\"rust\",\"library\",\"cool\"]".to_string(),
            ),
            license: Some("MIT".to_string()),
            name: Some("my_library".to_string()),
            output: Some("my_library".to_string()),
            readme: Some("README.md".to_string()),
            repository: Some(
                "https://github.com/example/my_library".to_string(),
            ),
            rustversion: Some("1.60".to_string()),
            version: Some("0.1.0".to_string()),
            website: Some("https://example.com/john-smith".to_string()),
        }
    }
    /// Parses the command line arguments and returns a new instance of
    /// the structure.

    /// Creates a new instance with default values.
    pub fn new() -> Self {
        Self::default_params()
    }
    /// Parses the command line arguments and returns a new instance of
    /// the structure.
    ///
    /// # Arguments
    /// * `args_str` - A string slice containing the command line arguments.
    ///
    /// # Errors
    /// Returns an `Err` if the arguments are not in the expected format
    /// or if mandatory arguments are missing. The error is a `String`
    /// describing what went wrong.
    ///
    pub fn from_args(args_str: &str) -> Result<Self, String> {
        let mut params = Self::new();
        let args: Vec<&str> = args_str.split_whitespace().collect();
        for arg in args {
            let mut arg_parts = arg.splitn(2, ' ');
            let arg_name = arg_parts
                .next()
                .ok_or_else(|| "Missing argument name".to_string())?;
            let arg_value = arg_parts
                .next()
                .ok_or_else(|| "Missing argument value".to_string())?;
            match arg_name {
                "--author" => {
                    params.author = Some(arg_value.to_string());
                }
                "--build" => params.build = Some(arg_value.to_string()),
                "--categories" => {
                    params.categories = Some(arg_value.to_string());
                }
                "--description" => {
                    params.description = Some(arg_value.to_string());
                }
                "--documentation" => {
                    params.documentation = Some(arg_value.to_string());
                }
                "--edition" => {
                    params.edition = Some(arg_value.to_string());
                }
                "--email" => params.email = Some(arg_value.to_string()),
                "--homepage" => {
                    params.homepage = Some(arg_value.to_string());
                }
                "--keywords" => {
                    params.keywords = Some(arg_value.to_string());
                }
                "--license" => {
                    params.license = Some(arg_value.to_string());
                }
                "--name" => params.name = Some(arg_value.to_string()),
                "--output" => {
                    params.output = Some(arg_value.to_string());
                }
                "--readme" => {
                    params.readme = Some(arg_value.to_string());
                }
                "--repository" => {
                    params.repository = Some(arg_value.to_string());
                }
                "--rustversion" => {
                    params.rustversion = Some(arg_value.to_string());
                }
                "--version" => {
                    params.version = Some(arg_value.to_string());
                }
                "--website" => {
                    params.website = Some(arg_value.to_string());
                }
                _ => (),
            }
        }
        Ok(params)
    }
}
