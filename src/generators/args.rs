use crate::{
    generator::generate_files,
    models::model_params::FileGenerationParams,
};
use std::io;

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
