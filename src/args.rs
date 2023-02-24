use clap::ArgMatches;

use super::generator::{generate_files, generate_via_csv};

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
            generate_files(
                author.cloned(),      // The name of the package's author.
                categories.cloned(),  // The categories that the package belongs to.
                csv.cloned(),         // The csv file to use for the template.
                description.cloned(), // The description of the package.
                email.cloned(),       // The email address of the package's author.
                keywords.cloned(),    // The keywords that describe the package.
                license.cloned(),     // The license under which the package is released.
                name.cloned(),        // The name of the package.
                output.cloned(),      // The output directory for the generated files.
                repository.cloned(),  // The URL of the package's repository.
                rustversion.cloned(), // The minimum Rust version required to use the package.
                version.cloned(),     // The version number of the package.
                website.cloned(),     // The URL of the package's website.
            )
            .expect("Failed to generate the template files");
        } else if let Some(csv_file) = matches.get_one::<String>("csv") {
            generate_via_csv(csv_file).expect("Failed to generate the template files");
        }
    }
}
