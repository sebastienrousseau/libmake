// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2023-2024 LibMake. All rights reserved.

use crate::{
    interface::replace_placeholders, macro_generate_from_csv,
    macro_generate_from_ini, macro_generate_from_json,
    macro_generate_from_toml, macro_generate_from_yaml,
    models::model_params::FileGenerationParams,
    utils::create_directory,
};
use std::{fs, io, path::PathBuf};

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
        "macros.tpl",
        "main.tpl",
        "README.tpl",
        "rustfmt.tpl",
        "TEMPLATE.tpl",
        "test.tpl",
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
#[allow(unused_results)]
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
    let project_directory =
        PathBuf::from(output.clone().trim_matches('\"'));

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
        // --- #Start GitHub Actions workflows ---
        // Add audit GitHub Actions workflows template
        ("github/workflows/audit.tpl", ".github/workflows/audit.yml"),
        // Add check GitHub Actions workflows template
        ("github/workflows/check.tpl", ".github/workflows/check.yml"),
        // Add coverage GitHub Actions workflows template
        (
            "github/workflows/coverage.tpl",
            ".github/workflows/coverage.yml",
        ),
        // Add document GitHub Actions workflows template
        (
            "github/workflows/document.tpl",
            ".github/workflows/document.yml",
        ),
        // Add lint GitHub Actions workflows template
        ("github/workflows/lint.tpl", ".github/workflows/lint.yml"),
        // Add release GitHub Actions workflows template
        (
            "github/workflows/release.tpl",
            ".github/workflows/release.yml",
        ),
        // Add test GitHub Actions workflows template
        ("github/workflows/test.tpl", ".github/workflows/test.yml"),
        // --- #End GitHub Actions workflows ---
        // Add Authors template
        ("AUTHORS.tpl", "AUTHORS.md"),
        // Add build template
        ("build.tpl", "build.rs"),
        // Add Cargo template
        ("Cargo.tpl", "Cargo.toml"),
        // Add Contributing template
        ("CONTRIBUTING.tpl", "CONTRIBUTING.md"),
        // Add Criterion template
        ("criterion.tpl", "benches/criterion.rs"),
        // Add Deepsource template
        ("deepsource.tpl", ".deepsource.toml"),
        // Add Deny template
        ("deny.tpl", "deny.toml"),
        // Add Example template
        ("example.tpl", "examples/example.rs"),
        // Add Gitignore template
        ("gitignore.tpl", ".gitignore"),
        // Add Lib template
        ("lib.tpl", "src/lib.rs"),
        // Add Macros template
        ("macros.tpl", "src/macros.rs"),
        // Add Main template
        ("main.tpl", "src/main.rs"),
        // Add Readme template
        ("README.tpl", "README.md"),
        // Add Rustfmt template
        ("rustfmt.tpl", "rustfmt.toml"),
        // Add Template template
        ("TEMPLATE.tpl", "TEMPLATE.md"),
        // Add Test template
        ("test_test.tpl", "tests/test_test.rs"),
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
/// - `file_type` - The type of the configuration file (e.g., JSON, YAML, CSV, TOML and INI).
///
/// # Errors
///
/// Returns an `io::Result` error if:
///
/// - The specified configuration file cannot be found, read, or is not in a valid format.
/// - There are issues parsing the configuration file into the `FileGenerationParams` struct.
/// - Any errors occur during the file generation process based on the configuration.
///
pub fn generate_from_config(
    path: &str,
    file_type: &str,
) -> Result<(), String> {
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
