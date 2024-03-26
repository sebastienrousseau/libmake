use std::fs;
use std::io;
use crate::models::model_params::FileGenerationParams;
use crate::macro_generate_files;

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
