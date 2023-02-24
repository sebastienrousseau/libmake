use std::{fs, path::PathBuf};

use crate::interface::replace_placeholders;
use prettytable::{row, Table};

/// Generates files for a new Rust project based on given arguments.
///
/// # Arguments
///
/// - `author` - The author of the project (optional).
/// - `categories` - The categories that the project belongs to (optional).
/// - `csv` - The CSV file to be used for generating the project (optional).
/// - `description` - A short description of the project (optional).
/// - `email` - The email address of the author (optional).
/// - `keywords` - Keywords that describe the project (optional).
/// - `license` - The license under which the project is released (optional).
/// - `name` - The name of the project (optional).
/// - `output` - The output directory where the project files will be created (required).
/// - `repository` - The URL of the project's repository (optional).
/// - `rustversion` - The minimum Rust version required by the project (optional).
/// - `version` - The initial version of the project (optional).
/// - `website` - The website of the project (optional).
///
pub fn generate_files(
    author: Option<String>,
    categories: Option<String>,
    csv: Option<String>,
    description: Option<String>,
    email: Option<String>,
    keywords: Option<String>,
    license: Option<String>,
    name: Option<String>,
    output: Option<String>,
    repository: Option<String>,
    rustversion: Option<String>,
    version: Option<String>,
    website: Option<String>,
) -> std::io::Result<()> {
    let project_directory = output.clone().unwrap();
    println!("Project directory: {}", project_directory);

    // Creating the project directory
    fs::create_dir(&project_directory)?;

    // Copying the Cargo.toml template file
    let mut cargo_template_file = PathBuf::new();
    cargo_template_file.push("template");
    cargo_template_file.push("Cargo.tpl");

    let mut cargo_template_readme_file = PathBuf::new();
    cargo_template_readme_file.push("template");
    cargo_template_readme_file.push("README.tpl");

    let mut cargo_file = PathBuf::new();
    cargo_file.push(&project_directory);
    cargo_file.push("Cargo.toml");
    fs::copy(&cargo_template_file, &cargo_file)?;

    let mut cargo_readme_file = PathBuf::new();
    cargo_readme_file.push(&project_directory);
    cargo_readme_file.push("README.md");
    fs::copy(&cargo_template_readme_file, &cargo_readme_file)?;

    // Replace the placeholders in the Cargo.toml file
    replace_placeholders(
        &cargo_template_file,
        &cargo_file,
        &author,
        &categories,
        &csv,
        &description,
        &email,
        &keywords,
        &license,
        &name,
        &repository,
        &rustversion,
        &version,
        &website,
    )?;

    // Replace the placeholders in the Readme.md file
    replace_placeholders(
        &cargo_template_readme_file,
        &cargo_readme_file,
        &author,
        &categories,
        &csv,
        &description,
        &email,
        &keywords,
        &license,
        &name,
        &repository,
        &rustversion,
        &version,
        &website,
    )?;

    // Print a table of the arguments and values
    let mut table = Table::new();
    table.add_row(row![b => "Argument", "Value"]);
    table.add_row(row!["author", author.unwrap_or_default()]);
    table.add_row(row!["categories", categories.unwrap_or_default()]);
    table.add_row(row!["csv", csv.unwrap_or_default()]);
    table.add_row(row!["description", description.unwrap_or_default()]);
    table.add_row(row!["email", email.unwrap_or_default()]);
    table.add_row(row!["keywords", keywords.unwrap_or_default()]);
    table.add_row(row!["license", license.unwrap_or_default()]);
    table.add_row(row!["name", name.unwrap_or_default()]);
    table.add_row(row!["output", output.unwrap()]);
    table.add_row(row!["repository", repository.unwrap_or_default()]);
    table.add_row(row!["rustversion", rustversion.unwrap_or_default()]);
    table.add_row(row!["version", version.unwrap_or_default()]);
    table.add_row(row!["website", website.unwrap_or_default()]);
    table.printstd();

    Ok(())
}

/// Generates files for a new Rust project based on a CSV file.
///
/// # Arguments
/// - `author` - the author of the project (optional).
/// - `categories` - the categories that the project belongs to (optional).
/// - `csv` - the CSV file to be used for generating the project (optional).
/// - `description` - a short description of the project (optional).
/// - `email` - the email address of the author (optional).
/// - `keywords` - keywords that describe the project (optional).
/// - `license` - the license under which the project is released (optional).
/// - `name` - the name of the project (optional).
/// - `output` - the output directory where the project files will be created (required).
/// - `repository` - the url of the project's repository (optional).
/// - `rustversion` - the minimum Rust version required by the project (optional).
/// - `version` - the initial version of the project (optional).
/// - `website` - the website of the project (optional).
///
pub fn generate_via_csv(path: &str) -> std::io::Result<()> {
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records() {
        let record = result?;
        let author = record.get(0).map(|s| s.to_string());
        let categories = record.get(1).map(|s| s.to_string());
        let csv = record.get(2).map(|s| s.to_string());
        let description = record.get(2).map(|s| s.to_string());
        let email = record.get(3).map(|s| s.to_string());
        let keywords = record.get(4).map(|s| s.to_string());
        let license = record.get(5).map(|s| s.to_string());
        let name = record.get(6).map(|s| s.to_string());
        let output = record.get(7).map(|s| s.to_string());
        let repository = record.get(8).map(|s| s.to_string());
        let rustversion = record.get(9).map(|s| s.to_string());
        let version = record.get(10).map(|s| s.to_string());
        let website = record.get(11).map(|s| s.to_string());

        generate_files(
            author,
            categories,
            csv,
            description,
            email,
            keywords,
            license,
            name,
            output,
            repository,
            rustversion,
            version,
            website,
        )?;
    }
    Ok(())
}
