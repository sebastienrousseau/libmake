use std::{fs, path::PathBuf};

use crate::interface::replace_placeholders;
use prettytable::{row, Table};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;

/// Structure for holding the parameters for generating files.
/// The parameters are optional, but the output directory is required.
/// The output directory is the directory where the project files will be created.
/// The other parameters are optional and will be used to replace the placeholders in the template files.
/// The placeholders are defined in the template files.
/// The template files are located in the template directory.
/// The template files are copied to the output directory and the placeholders are replaced with the values of the parameters.
/// The template files are Cargo.toml and README.md.
#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FileGenerationParams {
    /// The author of the project (optional).
    pub author: Option<String>,
    /// The categories that the project belongs to (optional).
    pub categories: Option<String>,
    /// The CSV file to be used for generating the project (optional).
    pub csv: Option<String>,
    /// A short description of the project (optional).
    pub description: Option<String>,
    /// The email address of the author (optional).
    pub email: Option<String>,
    /// Keywords that describe the project (optional).
    pub keywords: Option<String>,
    /// The license under which the project is released (optional).
    pub license: Option<String>,
    /// The name of the project (optional).
    pub name: Option<String>,
    /// The output directory where the project files will be created (required).
    pub output: Option<String>,
    /// The URL of the project's repository (optional).
    pub repository: Option<String>,
    /// The minimum Rust version required by the project (optional).
    pub rustversion: Option<String>,
    /// The initial version of the project (optional).
    pub version: Option<String>,
    /// The website of the project (optional).
    pub website: Option<String>,
}

/// Generates files for a new Rust project based on given arguments.
///
/// # Arguments
///
/// The arguments have the following format:
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
pub fn generate_files(params: FileGenerationParams) -> std::io::Result<()> {
    let project_directory = params.output.clone().unwrap();
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
    replace_placeholders(&cargo_template_file, &cargo_file, &params)?;

    // Replace the placeholders in the Readme.md file
    replace_placeholders(&cargo_template_readme_file, &cargo_readme_file, &params)?;

    // Print a table of the arguments and values
    let mut table = Table::new();
    table.add_row(row![b => "Argument", "Value"]);
    table.add_row(row!["author", params.author.unwrap_or_default()]);
    table.add_row(row!["categories", params.categories.unwrap_or_default()]);
    table.add_row(row!["csv", params.csv.unwrap_or_default()]);
    table.add_row(row!["description", params.description.unwrap_or_default()]);
    table.add_row(row!["email", params.email.unwrap_or_default()]);
    table.add_row(row!["keywords", params.keywords.unwrap_or_default()]);
    table.add_row(row!["license", params.license.unwrap_or_default()]);
    table.add_row(row!["name", params.name.unwrap_or_default()]);
    table.add_row(row!["output", params.output.unwrap()]);
    table.add_row(row!["repository", params.repository.unwrap_or_default()]);
    table.add_row(row!["rustversion", params.rustversion.unwrap_or_default()]);
    table.add_row(row!["version", params.version.unwrap_or_default()]);
    table.add_row(row!["website", params.website.unwrap_or_default()]);
    table.printstd();

    Ok(())
}

/// Generates files for a new Rust project based on a CSV file.
///
/// # Arguments
///
/// The CSV file must contain the following columns:
///
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
        let params = FileGenerationParams {
            author: record.get(0).map(|s| s.to_string()),
            categories: record.get(1).map(|s| s.to_string()),
            csv: record.get(2).map(|s| s.to_string()),
            description: record.get(2).map(|s| s.to_string()),
            email: record.get(3).map(|s| s.to_string()),
            keywords: record.get(4).map(|s| s.to_string()),
            license: record.get(5).map(|s| s.to_string()),
            name: record.get(6).map(|s| s.to_string()),
            output: record.get(7).map(|s| s.to_string()),
            repository: record.get(8).map(|s| s.to_string()),
            rustversion: record.get(9).map(|s| s.to_string()),
            version: record.get(10).map(|s| s.to_string()),
            website: record.get(11).map(|s| s.to_string()),
        };
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
pub fn generate_via_json(path: &str) -> std::io::Result<()> {
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
pub fn generate_via_yaml(path: &str) -> std::io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let params: FileGenerationParams = serde_yaml::from_str(&contents)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    generate_files(params)?;
    Ok(())
}
