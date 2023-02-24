use clap::ArgMatches;

use super::generator::{generate_files, generate_via_csv, FileGenerationParams};

/// Processes the command line arguments provided to the program.
///
/// # Arguments
///
/// * `matches` - An instance of `clap::ArgMatches` containing the parsed command line arguments.
///
pub fn process_arguments(matches: ArgMatches) {
    let author = matches.get_one::<String>("author");
    let categories = matches.get_one::<String>("categories");
    let csv = matches.get_one::<String>("csv");
    let description = matches.get_one::<String>("description");
    let email = matches.get_one::<String>("email");
    let keywords = matches.get_one::<String>("keywords");
    let license = matches.get_one::<String>("license");
    let name = matches.get_one::<String>("name");
    let output = matches.get_one::<String>("output");
    let repository = matches.get_one::<String>("repository");
    let rustversion = matches.get_one::<String>("rustversion");
    let version = matches.get_one::<String>("version");
    let website = matches.get_one::<String>("website");

    if matches.contains_id("csv") {
        if matches.get_one::<String>("csv") == Some(&"".to_string()) {
            let params = FileGenerationParams {
                author: author.cloned(),
                categories: categories.cloned(),
                csv: csv.cloned(),
                description: description.cloned(),
                email: email.cloned(),
                keywords: keywords.cloned(),
                license: license.cloned(),
                name: name.cloned(),
                output: output.cloned(),
                repository: repository.cloned(),
                rustversion: rustversion.cloned(),
                version: version.cloned(),
                website: website.cloned(),
            };
            generate_files(params).expect("Failed to generate the template files");
        } else if let Some(csv_file) = matches.get_one::<String>("csv") {
            generate_via_csv(csv_file).expect("Failed to generate the template files");
        }
    }
}
