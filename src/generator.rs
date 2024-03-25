// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

use super::interface::replace_placeholders;
use serde::{Deserialize, Serialize};
use serde_ini::from_str;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::macro_generate_from_csv;
use crate::macro_generate_from_ini;
use crate::macro_generate_from_json;
use crate::macro_generate_from_yaml;
use crate::macro_generate_from_toml;
use crate::macro_generate_files;

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
    /// The name of the project (optional).
    pub name: Option<String>,
    /// The output directory where the project files will be created (optional).
    pub output: Option<String>,
    /// The name of the readme file (optional).
    pub readme: Option<String>,
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
            rustversion: Some("1.75.0".to_string()),
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

/// Creates a directory at the specified path.
///
/// # Arguments
///
/// * `path` - The path where the directory should be created.
///
/// # Errors
///
/// Returns an `io::Error` if the directory cannot be created. This could be due to
/// various reasons such as insufficient permissions, the directory already existing,
/// or other I/O-related errors.
///
pub fn create_directory(path: &Path) -> io::Result<()> {
    fs::create_dir(path).or_else(|e| match e.kind() {
        io::ErrorKind::AlreadyExists => Ok(()),
        _ => Err(e),
    })
}
/// Creates the template folder and downloads necessary template files.
///
/// This function attempts to create a template directory in the current working directory
/// and then downloads a set of predefined template files into this directory.
///
/// # Errors
///
/// Returns an `io::Error` if the template directory cannot be created, or if there's an error
/// during the download and writing of the template files. Possible causes include
/// network issues, file permission errors, or other I/O-related problems.
///
pub fn create_template_folder() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    // println!("Current directory: {:?}", current_dir);
    let template_dir_path = current_dir.join("template");
    // println!("Creating template directory: {:?}", template_dir_path);
    create_directory(&template_dir_path)?;
    let url = "https://raw.githubusercontent.com/sebastienrousseau/libmake/main/template/";
    let files = [
        "AUTHORS.tpl",
        "build.tpl",
        "Cargo.tpl",
        "ci.tpl",
        "CONTRIBUTING.tpl",
        "criterion.tpl",
        "deepsource.tpl",
        "deny.tpl",
        "example.tpl",
        "gitignore.tpl",
        "lib.tpl",
        "loggers.tpl",
        "macros.tpl",
        "main.tpl",
        "README.tpl",
        "rustfmt.tpl",
        "TEMPLATE.tpl",
        "test.tpl",
        "test_loggers.tpl",
    ];
    for file in &files {
        let file_path = template_dir_path.join(file);
        // Check if the file already exists
        if !file_path.exists() {
            let file_url = format!("{url}{file}");
            let response =
                reqwest::blocking::get(&file_url).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "Failed to download template file: {e}"
                        ),
                    )
                })?;

            let file_contents = response.text().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to read response body: {e}"),
                )
            })?;

            // Write the file contents, trimming any leading or trailing newline characters
            fs::write(
                &file_path,
                file_contents
                    .trim_start_matches('\n')
                    .trim_end_matches('\n'),
            )?;
        }
    }
    Ok(())
}

/// Copies a template file to the output directory and replaces the
/// placeholders with the given parameters (if any).
///
/// # Arguments
///
/// * `template_file` - The name of the template file.
/// * `dest_file` - The name of the destination file.
/// * `project_directory` - The path to the project directory.
/// * `params` - The parameters to be used for replacing the placeholders.
///
/// # Errors
///
/// Returns an `io::Error` in the following cases:
///
/// - If the template file cannot be found, read, or copied.
/// - If the destination file cannot be created or written to.
/// - If there is an error in replacing placeholders in the destination file.
///
pub fn copy_and_replace_template(
    template_file: &str,
    dest_file: &str,
    project_directory: &PathBuf,
    params: &FileGenerationParams,
) -> io::Result<()> {
    let mut tpl_file = PathBuf::new();
    tpl_file.push("template");
    tpl_file.push(template_file);

    let mut dest_path = PathBuf::new();
    dest_path.push(project_directory);
    dest_path.push(dest_file);

    fs::copy(&tpl_file, &dest_path)?;

    replace_placeholders(&tpl_file, &dest_path, params)?;

    Ok(())
}

/// Generates files for a new Rust project based on given arguments.
///
/// # Arguments
///
/// - `params` - Parameters containing project details and configurations.
///
/// # Errors
///
/// Returns an `io::Result` error if:
///
/// - There are issues creating the project directory or subdirectories.
/// - There are issues copying or replacing the template files.
/// - Any other I/O related errors occur during the file generation process.
///
pub fn generate_files(params: FileGenerationParams) -> io::Result<()> {
    let Some(ref output) = params.output else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Output directory is not specified",
        ));
    };

    // Get the project directory path from the output parameter,
    // create a PathBuf from it, and assign it to the project_directory variable
    let project_directory = PathBuf::from(output.clone().trim_matches('\"'));

    // Creating the project directory
    create_directory(&project_directory)?;

    // Creating the template directory
    create_template_folder()?;

    // Define the subdirectories to be created within the project directory
    let subdirectories = [
        "src",
        "benches",
        "examples",
        "tests",
        ".github/",
        ".github/workflows",
    ];

    // Iterate over the subdirectories and create them
    for subdir in &subdirectories {
        let dir_path = project_directory.join(subdir);
        create_directory(&dir_path)?;
    }

    // Copying the template files to the new library directory
    let templates = [
        ("AUTHORS.tpl", "AUTHORS.md"),
        ("build.tpl", "build.rs"),
        ("Cargo.tpl", "Cargo.toml"),
        ("ci.tpl", ".github/workflows/ci.yml"),
        ("CONTRIBUTING.tpl", "CONTRIBUTING.md"),
        ("criterion.tpl", "benches/criterion.rs"),
        ("deepsource.tpl", ".deepsource.toml"),
        ("deny.tpl", "deny.toml"),
        ("example.tpl", "examples/example.rs"),
        ("gitignore.tpl", ".gitignore"),
        ("lib.tpl", "src/lib.rs"),
        ("loggers.tpl", "src/loggers.rs"),
        ("macros.tpl", "src/macros.rs"),
        ("main.tpl", "src/main.rs"),
        ("README.tpl", "README.md"),
        ("rustfmt.tpl", "rustfmt.toml"),
        ("TEMPLATE.tpl", "TEMPLATE.md"),
        ("test_loggers.tpl", "tests/test_loggers.rs"),
        ("test.tpl", "tests/test.rs"),
    ];

    for (template, target) in templates {
        copy_and_replace_template(
            template,
            target,
            &project_directory,
            &params,
        )?;
    }

    // Displaying the argument and value pairs
    println!("{:<15}Value", "Argument");
    println!("{:<15}{}", "author", params.author.unwrap_or_default());
    println!("{:<15}{}", "build", params.build.unwrap_or_default());
    println!(
        "{:<15}{}",
        "categories",
        params.categories.unwrap_or_default()
    );
    println!(
        "{:<15}{}",
        "description",
        params.description.unwrap_or_default()
    );
    println!(
        "{:<15}{}",
        "documentation",
        params.documentation.unwrap_or_default()
    );
    println!("{:<15}{}", "edition", params.edition.unwrap_or_default());
    println!("{:<15}{}", "email", params.email.unwrap_or_default());
    println!(
        "{:<15}{}",
        "homepage",
        params.homepage.unwrap_or_default()
    );
    println!(
        "{:<15}{}",
        "keywords",
        params.keywords.unwrap_or_default()
    );
    println!("{:<15}{}", "license", params.license.unwrap_or_default());
    println!("{:<15}{}", "name", params.name.unwrap_or_default());
    println!("{:<15}{}", "output", output.clone());
    println!("{:<15}{}", "readme", params.readme.unwrap_or_default());
    println!(
        "{:<15}{}",
        "repository",
        params.repository.unwrap_or_default()
    );
    println!(
        "{:<15}{}",
        "rustversion",
        params.rustversion.unwrap_or_default()
    );
    println!("{:<15}{}", "version", params.version.unwrap_or_default());
    println!("{:<15}{}", "website", params.website.unwrap_or_default());

    Ok(())
}

/// Generates files for a new Rust project based on a configuration file.
///
/// # Arguments
///
/// - `path` - The path to the configuration file.
/// - `file_type` - The type of the configuration file (e.g., JSON, YAML, CSV).
///
/// # Errors
///
/// Returns an `io::Result` error if:
///
/// - The specified configuration file cannot be found, read, or is not in a valid format.
/// - There are issues parsing the configuration file into the `FileGenerationParams` struct.
/// - Any errors occur during the file generation process based on the configuration.
///
pub fn generate_from_config(path: &str, file_type: &str) -> Result<(), String> {
    match file_type {
        "csv" => macro_generate_from_csv!(path)?,
        "ini" => macro_generate_from_ini!(path)?,
        "json" => macro_generate_from_json!(path)?,
        "yaml" => macro_generate_from_yaml!(path)?,
        "toml" => macro_generate_from_toml!(path)?,
        _ => {
            return Err(format!(
                "Unsupported configuration file type: {}",
                file_type
            ))
        }
    }
    Ok(())
}

/// Generates files for a new Rust project based on a CSV file.
///
/// # Arguments
///
/// The CSV file must contain the following columns:
///
/// - `author` - the author of the project (optional).
/// - `build` - the build command to be used for building the project (optional).
/// - `categories` - the categories that the project belongs to (optional).
/// - `description` - a short description of the project (optional).
/// - `documentation` - the documentation URL of the project (optional).
/// - `edition` - the edition of the project (optional).
/// - `email` - the email address of the author (optional).
/// - `homepage` - the homepage of the project (optional).
/// - `keywords` - keywords that describe the project (optional).
/// - `license` - the license under which the project is released (optional).
/// - `name` - the name of the project (optional).
/// - `output` - the output directory where the project files will be created (required).
/// - `readme` - the name of the readme file (optional).
/// - `repository` - the url of the project's repository (optional).
/// - `rustversion` - the minimum Rust version required by the project (optional).
/// - `version` - the initial version of the project (optional).
/// - `website` - the website of the project (optional).
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - If the specified CSV file cannot be found, read, or is not valid CSV.
/// - If an error occurs while parsing the CSV data into the `FileGenerationParams` struct.
/// - If there is an error in generating files based on the parameters from each CSV record.
///
pub fn generate_from_csv(path: &str) -> io::Result<()> {
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records() {
        let record = result?;
        // println!("{:?}", record);
        let params = FileGenerationParams {
            author: record.get(0).map(ToString::to_string),
            build: record.get(1).map(ToString::to_string),
            categories: record.get(2).map(ToString::to_string),
            description: record.get(3).map(ToString::to_string),
            documentation: record.get(4).map(ToString::to_string),
            edition: record.get(5).map(ToString::to_string),
            email: record.get(6).map(ToString::to_string),
            homepage: record.get(7).map(ToString::to_string),
            keywords: record.get(8).map(ToString::to_string),
            license: record.get(9).map(ToString::to_string),
            name: record.get(10).map(ToString::to_string),
            output: record.get(11).map(ToString::to_string),
            readme: record.get(12).map(ToString::to_string),
            repository: record.get(13).map(ToString::to_string),
            rustversion: record.get(14).map(ToString::to_string),
            version: record.get(15).map(ToString::to_string),
            website: record.get(16).map(ToString::to_string),
        };
        // println!("Params: {:?}", params);
        macro_generate_files!(params.clone())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
    Ok(())
}

/// Generates files for a new Rust project based on a JSON file.
///
/// # Arguments
///
/// The JSON file must contain a single object with the following
/// properties:
///
/// - `author` - the author of the project (optional).
/// - `build` - the build command to be used for building the project (optional).
/// - `categories` - the categories that the project belongs to (optional).
/// - `description` - a short description of the project (optional).
/// - `documentation` - the documentation URL of the project (optional).
/// - `edition` - the edition of the project (optional).
/// - `email` - the email address of the author (optional).
/// - `homepage` - the homepage of the project (optional).
/// - `keywords` - keywords that describe the project (optional).
/// - `license` - the license under which the project is released (optional).
/// - `name` - the name of the project (optional).
/// - `output` - the output directory where the project files will be created (required).
/// - `readme` - the name of the readme file (optional).
/// - `repository` - the url of the project's repository (optional).
/// - `rustversion` - the minimum Rust version required by the project (optional).
/// - `version` - the initial version of the project (optional).
/// - `website` - the website of the project (optional).
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - If the specified JSON file cannot be found, read, or is not valid UTF-8.
/// - If the JSON data cannot be deserialized into the `FileGenerationParams` struct.
/// - If there is an error in generating files based on the parameters.
///
pub fn generate_from_json(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let params: FileGenerationParams = serde_json::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    macro_generate_files!(params.clone()).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e)
    })?;
    Ok(())
}

/// Generates files for a new Rust project based on a YAML file.
///
/// The YAML file must contain a single object with the following
/// properties:
///
/// - `author` - the author of the project (optional).
/// - `build` - the build command to be used for building the project (optional).
/// - `categories` - the categories that the project belongs to (optional).
/// - `description` - a short description of the project (optional).
/// - `documentation` - the documentation URL of the project (optional).
/// - `edition` - the edition of the project (optional).
/// - `email` - the email address of the author (optional).
/// - `homepage` - the homepage of the project (optional).
/// - `keywords` - keywords that describe the project (optional).
/// - `license` - the license under which the project is released (optional).
/// - `name` - the name of the project (optional).
/// - `output` - the output directory where the project files will be created (required).
/// - `readme` - the name of the readme file (optional).
/// - `repository` - the url of the project's repository (optional).
/// - `rustversion` - the minimum Rust version required by the project (optional).
/// - `version` - the initial version of the project (optional).
/// - `website` - the website of the project (optional).
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - If the specified YAML file cannot be found, read, or is not valid UTF-8.
/// - If the YAML data cannot be deserialized into the `FileGenerationParams` struct.
/// - If there is an error in generating files based on the parameters.
///
pub fn generate_from_yaml(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let params: FileGenerationParams = serde_yaml::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    macro_generate_files!(params.clone()).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e)
    })?;
    Ok(())
}

/// Generates files for a new Rust project based on an INI file.
///
/// The INI file must contain a single section with the following
/// keys:
/// - `author` - the author of the project (optional).
/// - `build` - the build command to be used for building the project (optional).
/// - `categories` - the categories that the project belongs to (optional).
/// - `description` - a short description of the project (optional).
/// - `documentation` - the documentation URL of the project (optional).
/// - `edition` - the edition of the project (optional).
/// - `email` - the email address of the author (optional).
/// - `homepage` - the homepage of the project (optional).
/// - `keywords` - keywords that describe the project (optional).
/// - `license` - the license under which the project is released (optional).
/// - `name` - the name of the project (optional).
/// - `output` - the output directory where the project files will be created (required).
/// - `readme` - the name of the readme file (optional).
/// - `repository` - the url of the project's repository (optional).
/// - `rustversion` - the minimum Rust version required by the project (optional).
/// - `version` - the initial version of the project (optional).
/// - `website` - the website of the project (optional).
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - If the specified INI file cannot be found, read, or is not valid UTF-8.
/// - If the INI data cannot be parsed into the `FileGenerationParams` struct.
/// - If there is an error in generating files based on the parameters.
///
pub fn generate_from_ini(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let params: FileGenerationParams = from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    macro_generate_files!(params.clone()).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e)
    })?;
    Ok(())
}

/// Generates files for a new Rust project based on a TOML file.
///
/// The TOML file must contain a single object with the following
/// properties:
///
/// - `author` - the author of the project (optional).
/// - `build` - the build command to be used for building the project (optional).
/// - `categories` - the categories that the project belongs to (optional).
/// - `description` - a short description of the project (optional).
/// - `documentation` - the documentation URL of the project (optional).
/// - `edition` - the edition of the project (optional).
/// - `email` - the email address of the author (optional).
/// - `homepage` - the homepage of the project (optional).
/// - `keywords` - keywords that describe the project (optional).
/// - `license` - the license under which the project is released (optional).
/// - `name` - the name of the project (optional).
/// - `output` - the output directory where the project files will be created (required).
/// - `readme` - the name of the readme file (optional).
/// - `repository` - the url of the project's repository (optional).
/// - `rustversion` - the minimum Rust version required by the project (optional).
/// - version - the initial version of the project (optional).
/// - website - the website of the project (optional).
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - If the specified TOML file cannot be found, read, or is not valid UTF-8.
/// - If the TOML data cannot be deserialized into the FileGenerationParams struct.
/// - If there is an error in generating files based on the parameters.
///
pub fn generate_from_toml(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let params: FileGenerationParams = toml::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    macro_generate_files!(params.clone()).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e)
    })?;
    Ok(())
}

/// Generates files for a new Rust project based on command line arguments.
/// The arguments must be in the form `--name=value`.
/// The following arguments are supported:
/// - `author` - the author of the project (optional).
/// - `build` - the build command to be used for building the project (optional).
/// - `categories` - the categories that the project belongs to (optional).
/// - `description` - a short description of the project (optional).
/// - `documentation` - the documentation URL of the project (optional).
/// - `edition` - the edition of the project (optional).
/// - `email` - the email address of the author (optional).
/// - `homepage` - the homepage of the project (optional).
/// - `keywords` - keywords that describe the project (optional).
/// - `license` - the license under which the project is released (optional).
/// - `name` - the name of the project (optional).
/// - `output` - the output directory where the project files will be created (required).
/// - `readme` - the name of the readme file (optional).
/// - `repository` - the url of the project's repository (optional).
/// - `rustversion` - the minimum Rust version required by the project (optional).
/// - `version` - the initial version of the project (optional).
/// - `website` - the website of the project (optional).
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - If an invalid argument is provided. Each argument must be in the form `--name=value`.
/// - If there is an error in generating files based on the parameters derived from the arguments.
///
pub fn generate_from_args(args_str: &str) -> io::Result<()> {
    let args = args_str.split_whitespace();
    let mut params = FileGenerationParams::default();
    for arg in args {
        let mut parts = arg.splitn(2, '=');
        let name = parts.next().unwrap_or_default();
        let value = parts.next().unwrap_or_default();
        match name {
            "--author" => params.author = Some(value.to_string()),
            "--build" => params.build = Some(value.to_string()),
            "--categories" => {
                params.categories = Some(value.to_string());
            }
            "--description" => {
                params.description = Some(value.to_string());
            }
            "--documentation" => {
                params.documentation = Some(value.to_string());
            }
            "--edition" => params.edition = Some(value.to_string()),
            "--email" => params.email = Some(value.to_string()),
            "--homepage" => params.homepage = Some(value.to_string()),
            "--keywords" => params.keywords = Some(value.to_string()),
            "--license" => params.license = Some(value.to_string()),
            "--name" => params.name = Some(value.to_string()),
            "--output" => params.output = Some(value.to_string()),
            "--readme" => params.readme = Some(value.to_string()),
            "--repository" => {
                params.repository = Some(value.to_string());
            }
            "--rustversion" => {
                params.rustversion = Some(value.to_string());
            }
            "--version" => params.version = Some(value.to_string()),
            "--website" => params.website = Some(value.to_string()),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Invalid argument: {name}"),
                ))
            }
        }
    }
    println!("{params:?}");
    generate_files(params)?;
    Ok(())
}
