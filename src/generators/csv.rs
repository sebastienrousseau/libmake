use std::io;
use csv::Reader;
use crate::models::model_params::FileGenerationParams;
use crate::macro_generate_files;

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
    let mut reader = Reader::from_path(path)?;
    for result in reader.records() {
        let record = result?;
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
        macro_generate_files!(params.clone())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
    Ok(())
}
