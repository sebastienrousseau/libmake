use std::{fs, io, path::PathBuf};

use crate::interface::replace_placeholders;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;

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
#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FileGenerationParams {
    /// The author of the project (optional).
    pub author: Option<String>,
    /// The build command to be used for building the project (optional).
    pub build: Option<String>,
    /// The categories that the project belongs to (optional).
    pub categories: Option<String>,
    /// The CSV file to be used for generating the project (optional).
    pub csv: Option<String>,
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
fn copy_and_replace_template(
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
/// - `csv` - The CSV file to be used for generating the project (optional).
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
pub fn generate_files(params: FileGenerationParams) -> std::io::Result<()> {
    let project_directory_str = params.output.clone().unwrap();
    let project_directory = PathBuf::from(project_directory_str.clone());

    // Creating the project directory
    fs::create_dir(&project_directory)?;

    // Creating the src directory
    let mut src_directory = project_directory_str;
    src_directory.push_str("/src");
    fs::create_dir(&src_directory)?;

    // Copying the template files to the new library directory
    copy_and_replace_template("gitignore.tpl", ".gitignore", &project_directory, &params)?;
    copy_and_replace_template("Cargo.tpl", "Cargo.toml", &project_directory, &params)?;
    copy_and_replace_template(
        "CONTRIBUTING.tpl",
        "CONTRIBUTING.md",
        &project_directory,
        &params,
    )?;
    copy_and_replace_template("README.tpl", "README.md", &project_directory, &params)?;
    copy_and_replace_template("lib.tpl", "src/lib.rs", &project_directory, &params)?;

    // Storing values of the command-line arguments into variables
    let author = params.author.unwrap_or_default();
    let build = params.build.unwrap_or_default();
    let categories = params.categories.unwrap_or_default();
    let csv = params.csv.unwrap_or_default();
    let description = params.description.unwrap_or_default();
    let documentation = params.documentation.unwrap_or_default();
    let edition = params.edition.unwrap_or_default();
    let email = params.email.unwrap_or_default();
    let homepage = params.homepage.unwrap_or_default();
    let keywords = params.keywords.unwrap_or_default();
    let license = params.license.unwrap_or_default();
    let name = params.name.unwrap_or_default();
    let output = params.output.unwrap();
    let readme = params.readme.unwrap_or_default();
    let repository = params.repository.unwrap_or_default();
    let rustversion = params.rustversion.unwrap_or_default();
    let version = params.version.unwrap_or_default();
    let website = params.website.unwrap_or_default();

    // Displaying the argument and value pairs
    println!("{:<15}Value", "Argument");
    println!("{:<15}{}", "author", author);
    println!("{:<15}{}", "build", build);
    println!("{:<15}{}", "categories", categories);
    println!("{:<15}{}", "csv", csv);
    println!("{:<15}{}", "description", description);
    println!("{:<15}{}", "documentation", documentation);
    println!("{:<15}{}", "edition", edition);
    println!("{:<15}{}", "email", email);
    println!("{:<15}{}", "homepage", homepage);
    println!("{:<15}{}", "keywords", keywords);
    println!("{:<15}{}", "license", license);
    println!("{:<15}{}", "name", name);
    println!("{:<15}{}", "output", output);
    println!("{:<15}{}", "readme", readme);
    println!("{:<15}{}", "repository", repository);
    println!("{:<15}{}", "rustversion", rustversion);
    println!("{:<15}{}", "version", version);
    println!("{:<15}{}", "website", website);

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
/// - `csv` - the CSV file to be used for generating the project (optional).
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
pub fn generate_via_csv(path: &str) -> std::io::Result<()> {
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records() {
        let record = result?;
        let params = FileGenerationParams {
            author: record.get(0).map(|s| s.to_string()),
            build: record.get(1).map(|s| s.to_string()),
            categories: record.get(2).map(|s| s.to_string()),
            csv: record.get(3).map(|s| s.to_string()),
            description: record.get(4).map(|s| s.to_string()),
            documentation: record.get(5).map(|s| s.to_string()),
            edition: record.get(6).map(|s| s.to_string()),
            email: record.get(7).map(|s| s.to_string()),
            homepage: record.get(8).map(|s| s.to_string()),
            keywords: record.get(9).map(|s| s.to_string()),
            license: record.get(10).map(|s| s.to_string()),
            name: record.get(11).map(|s| s.to_string()),
            output: record.get(12).map(|s| s.to_string()),
            readme: record.get(13).map(|s| s.to_string()),
            repository: record.get(14).map(|s| s.to_string()),
            rustversion: record.get(15).map(|s| s.to_string()),
            version: record.get(16).map(|s| s.to_string()),
            website: record.get(17).map(|s| s.to_string()),
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
/// - `build` - the build command to be used for building the project (optional).
/// - `categories` - the categories that the project belongs to (optional).
/// - `csv` - the CSV file to be used for generating the project (optional).
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
/// - `build` - the build command to be used for building the project (optional).
/// - `categories` - the categories that the project belongs to (optional).
/// - `csv` - the CSV file to be used for generating the project (optional).
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
pub fn generate_via_yaml(path: &str) -> std::io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let params: FileGenerationParams = serde_yaml::from_str(&contents)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    generate_files(params)?;
    Ok(())
}
