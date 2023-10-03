// Copyright © 2023 LibMake. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::interface::replace_placeholders;
use reqwest::blocking::get;
// use assert_cmd::Command;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Structure for holding the parameters for generating the project files.
///
/// # Description
///
/// * The parameters are optional, but the output directory is required.
/// * The output directory is the directory where the project files will be created.
/// * The other parameters are optional and will be used to replace the placeholders in the template files.
/// * The template files are located in the template directory of the project.
/// * The template files are copied to the output directory and the placeholders are replaced with the values of the parameters.
///
///
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
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
    /// The output directory where the project files will be created (required).
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
    /// Creates a default instance with default values.
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
            rustversion: Some("1.69.0".to_string()),
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
    /// Creates a new instance from the command line arguments.
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
                    params.author = Some(arg_value.to_string())
                }
                "--build" => params.build = Some(arg_value.to_string()),
                "--categories" => {
                    params.categories = Some(arg_value.to_string())
                }
                "--description" => {
                    params.description = Some(arg_value.to_string())
                }
                "--documentation" => {
                    params.documentation = Some(arg_value.to_string())
                }
                "--edition" => {
                    params.edition = Some(arg_value.to_string())
                }
                "--email" => params.email = Some(arg_value.to_string()),
                "--homepage" => {
                    params.homepage = Some(arg_value.to_string())
                }
                "--keywords" => {
                    params.keywords = Some(arg_value.to_string())
                }
                "--license" => {
                    params.license = Some(arg_value.to_string())
                }
                "--name" => params.name = Some(arg_value.to_string()),
                "--output" => {
                    params.output = Some(arg_value.to_string())
                }
                "--readme" => {
                    params.readme = Some(arg_value.to_string())
                }
                "--repository" => {
                    params.repository = Some(arg_value.to_string())
                }
                "--rustversion" => {
                    params.rustversion = Some(arg_value.to_string())
                }
                "--version" => {
                    params.version = Some(arg_value.to_string())
                }
                "--website" => {
                    params.website = Some(arg_value.to_string())
                }
                _ => (),
            }
        }
        Ok(params)
    }
}

/// Creates a directory if it does not exist. If the directory already
/// exists, the function will return `Ok(())`.
///
/// # Arguments
///
/// - `path` - The path to the directory.
///
pub fn create_directory(path: &Path) -> io::Result<()> {
    fs::create_dir(path).or_else(|e| match e.kind() {
        io::ErrorKind::AlreadyExists => Ok(()),
        _ => Err(e),
    })
}
/// Creates the template directory and downloads the template files.
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
        "macros.tpl",
        "main.tpl",
        "README.tpl",
        "rustfmt.tpl",
        "TEMPLATE.tpl",
        "test.tpl",
    ];
    for file in &files {
        let file_url = format!("{}{}", url, file);
        let file_path = template_dir_path.join(file);
        let response = get(&file_url).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to download template file: {}", e),
            )
        })?;
        let file_contents = response.text().map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to read response body: {}", e),
            )
        })?;
        std::fs::write(
            &file_path,
            file_contents
                .trim_start_matches('\n')
                .trim_end_matches('\n'),
        )?;
    }
    Ok(())
}

/// Copies a template file to the output directory and replaces the
/// placeholders with the given parameters (if any).
///
/// # Arguments
///
/// - `template_file` - The name of the template file.
/// - `dest_file` - The name of the destination file.
/// - `project_directory` - The path to the project directory.
/// - `params` - The parameters to be used for replacing the placeholders.
///
/// # Returns
///
/// An `io::Result` with the result of the operation. If the operation
/// was successful, the result will be `Ok(())`. If the operation failed,
/// the result will be `Err(io::Error)`.
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
/// The arguments have the following format:
///
/// - `author` - The author of the project (optional).
/// - `build` - The build command to be used for building the project (optional).
/// - `categories` - The categories that the project belongs to (optional).
/// - `description` - A short description of the project (optional).
/// - `documentation` - The documentation URL of the project (optional).
/// - `edition` - The edition of the project (optional).
/// - `email` - The email address of the author (optional).
/// - `homepage` - The homepage of the project (optional).
/// - `keywords` - Keywords that describe the project (optional).
/// - `license` - The license under which the project is released (optional).
/// - `name` - The name of the project (optional).
/// - `output` - The output directory where the project files will be created (required).
/// - `readme` - The name of the readme file (optional).
/// - `repository` - The URL of the project's repository (optional).
/// - `rustversion` - The minimum Rust version required by the project (optional).
/// - `version` - The initial version of the project (optional).
/// - `website` - The website of the project (optional).
///
pub fn generate_files(params: FileGenerationParams) -> io::Result<()> {
    let output = match params.output {
        Some(ref output) => output,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Output directory is not specified",
            ))
        }
    };

    let project_directory = PathBuf::from(output.clone());

    // Creating the project directory
    create_directory(&project_directory)?;

    // Creating the template directory
    create_template_folder()?;

    // Creating the src directory
    let src_directory = project_directory.join("src");
    create_directory(&src_directory)?;

    // Creating the benches directory
    let benches_directory = project_directory.join("benches");
    create_directory(&benches_directory)?;

    // Creating the examples directory
    let examples_directory = project_directory.join("examples");
    create_directory(&examples_directory)?;

    // Creating the tests directory
    let tests_directory = project_directory.join("tests");
    create_directory(&tests_directory)?;

    // Creating the .github directory
    let github_directory = project_directory.join(".github");
    create_directory(&github_directory)?;

    // Creating the .github/workflows directory
    let workflows_directory = github_directory.join("workflows");
    create_directory(&workflows_directory)?;

    // Copying the template files to the new library directory

    // Copying the `AUTHORS.tpl` file to the new library directory
    copy_and_replace_template(
        "AUTHORS.tpl",
        "AUTHORS.md",
        &project_directory,
        &params,
    )?;
    // Copying the `build.tpl` file to the new library directory
    copy_and_replace_template(
        "build.tpl",
        "build.rs",
        &project_directory,
        &params,
    )?;
    // Copying the `Cargo.tpl` file to the new library directory
    copy_and_replace_template(
        "Cargo.tpl",
        "Cargo.toml",
        &project_directory,
        &params,
    )?;
    // Copying the `ci.tpl` file to the new library directory
    copy_and_replace_template(
        "ci.tpl",
        ".github/workflows/ci.yml",
        &project_directory,
        &params,
    )?;
    // Copying the `CONTRIBUTING.tpl` file to the new library directory
    copy_and_replace_template(
        "CONTRIBUTING.tpl",
        "CONTRIBUTING.md",
        &project_directory,
        &params,
    )?;
    // Copying the `criterion.tpl` file to the new library directory
    copy_and_replace_template(
        "criterion.tpl",
        "benches/criterion.rs",
        &project_directory,
        &params,
    )?;
    // Copying the `deepsource.tpl` file to the new library directory
    copy_and_replace_template(
        "deepsource.tpl",
        ".deepsource.toml",
        &project_directory,
        &params,
    )?;
    // Copying the `deny.tpl` file to the new library directory
    copy_and_replace_template(
        "deny.tpl",
        "deny.toml",
        &project_directory,
        &params,
    )?;
    // Copying the `example.tpl` file to the new library directory
    copy_and_replace_template(
        "example.tpl",
        "examples/example.rs",
        &project_directory,
        &params,
    )?;
    // Copying the `gitignore.tpl` file to the new library directory
    copy_and_replace_template(
        "gitignore.tpl",
        ".gitignore",
        &project_directory,
        &params,
    )?;
    // Copying the `lib.tpl` file to the new library directory
    copy_and_replace_template(
        "lib.tpl",
        "src/lib.rs",
        &project_directory,
        &params,
    )?;
    // Copying the `macros.tpl` file to the new library directory
    copy_and_replace_template(
        "macros.tpl",
        "src/macros.rs",
        &project_directory,
        &params,
    )?;
    // Copying the `main.tpl` file to the new library directory
    copy_and_replace_template(
        "main.tpl",
        "src/main.rs",
        &project_directory,
        &params,
    )?;
    // Copying the `README.tpl` file to the new library directory
    copy_and_replace_template(
        "README.tpl",
        "README.md",
        &project_directory,
        &params,
    )?;
    // Copying the `rustfmt.tpl` file to the new library directory
    copy_and_replace_template(
        "rustfmt.tpl",
        "rustfmt.toml",
        &project_directory,
        &params,
    )?;
    // Copying the `TEMPLATE.tpl` file to the new library directory
    copy_and_replace_template(
        "TEMPLATE.tpl",
        "TEMPLATE.md",
        &project_directory,
        &params,
    )?;
    // Copying the `test.tpl` file to the new library directory
    copy_and_replace_template(
        "test.tpl",
        "tests/test.rs",
        &project_directory,
        &params,
    )?;

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
/// Supported configuration file formats: JSON, YAML, CSV.
///
/// # Arguments
///
/// - `path` - The path to the configuration file (required).
/// - `file_type` - The type of the configuration file (required). Supported types: `json`, `yaml`, `yml`, `csv`.
///
pub fn generate_from_config(
    path: &str,
    file_type: &str,
) -> io::Result<()> {
    match file_type {
        "csv" => generate_from_csv(path),
        "json" => generate_from_json(path),
        "yaml" | "yml" => generate_from_yaml(path),
        "toml" => generate_from_toml(path),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid configuration file format. Supported formats: CSV, JSON, TOML, YAML.",
        )),
    }
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
pub fn generate_from_csv(csv_path: &str) -> io::Result<()> {
    let mut reader = csv::Reader::from_path(csv_path)?;
    for result in reader.records() {
        let record = result?;
        // println!("{:?}", record);
        let params = FileGenerationParams {
            author: record.get(0).map(|s| s.to_string()),
            build: record.get(1).map(|s| s.to_string()),
            categories: record.get(2).map(|s| s.to_string()),
            description: record.get(3).map(|s| s.to_string()),
            documentation: record.get(4).map(|s| s.to_string()),
            edition: record.get(5).map(|s| s.to_string()),
            email: record.get(6).map(|s| s.to_string()),
            homepage: record.get(7).map(|s| s.to_string()),
            keywords: record.get(8).map(|s| s.to_string()),
            license: record.get(9).map(|s| s.to_string()),
            name: record.get(10).map(|s| s.to_string()),
            output: record.get(11).map(|s| s.to_string()),
            readme: record.get(12).map(|s| s.to_string()),
            repository: record.get(13).map(|s| s.to_string()),
            rustversion: record.get(14).map(|s| s.to_string()),
            version: record.get(15).map(|s| s.to_string()),
            website: record.get(16).map(|s| s.to_string()),
        };
        // println!("Params: {:?}", params);
        generate_files(params)?;
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
pub fn generate_from_json(path: &str) -> std::io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let params: FileGenerationParams = serde_json::from_str(&contents)?;
    generate_files(params)?;
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
pub fn generate_from_yaml(path: &str) -> std::io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let params: FileGenerationParams = serde_yaml::from_str(&contents)
        .map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
    generate_files(params)?;
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
/// - `version` - the initial version of the project (optional).
/// - `website` - the website of the project (optional).
///
pub fn generate_from_toml(path: &str) -> std::io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let params: FileGenerationParams = toml::from_str(&contents)
        .map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
    generate_files(params)?;
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
pub fn generate_from_args(args_str: &str) -> std::io::Result<()> {
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
                params.categories = Some(value.to_string())
            }
            "--description" => {
                params.description = Some(value.to_string())
            }
            "--documentation" => {
                params.documentation = Some(value.to_string())
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
                params.repository = Some(value.to_string())
            }
            "--rustversion" => {
                params.rustversion = Some(value.to_string())
            }
            "--version" => params.version = Some(value.to_string()),
            "--website" => params.website = Some(value.to_string()),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Invalid argument: {}", name),
                ))
            }
        }
    }
    println!("{:?}", params);
    generate_files(params)?;
    Ok(())
}
