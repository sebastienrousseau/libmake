use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use super::generator::FileGenerationParams;

/// Replaces placeholders in a template file with given values
///
/// # Arguments
///
/// * template_file - A PathBuf pointing to the input template file
/// * output_file - A PathBuf pointing to the output file to be created
/// * author - An Option<String> containing the author name
/// * categories - An Option<String> containing the categories
/// * csv - An Option<String> containing the CSV file
/// * description - An Option<String> containing the description
/// * email - An Option<String> containing the email address
/// * keywords - An Option<String> containing the keywords
/// * license - An Option<String> containing the license
/// * name - An Option<String> containing the name
/// * repository - An Option<String> containing the repository URL
/// * rustversion - An Option<String> containing the Rust version
/// * version - An Option<String> containing the version
/// * website - An Option<String> containing the website URL
///
pub fn replace_placeholders(
    template_file: &PathBuf,
    output_file: &PathBuf,
    params: &FileGenerationParams,
) -> std::io::Result<()> {
    let tpl = File::open(template_file)?;
    let tpl_reader = BufReader::new(tpl);
    let mut output = File::create(output_file)?;
    let tpl_lines = tpl_reader.lines();
    for line in tpl_lines {
        let line = line?;
        let replaced_line = line
            .replace(
                "{author}",
                params.author.as_ref().unwrap_or(&"".to_string()),
            )
            .replace(
                "{categories}",
                params.categories.as_ref().unwrap_or(&"".to_string()),
            )
            .replace("{csv}", params.csv.as_ref().unwrap_or(&"".to_string()))
            .replace(
                "{description}",
                params.description.as_ref().unwrap_or(&"".to_string()),
            )
            .replace("{email}", params.email.as_ref().unwrap_or(&"".to_string()))
            .replace(
                "{keywords}",
                params.keywords.as_ref().unwrap_or(&"".to_string()),
            )
            .replace(
                "{license}",
                params.license.as_ref().unwrap_or(&"".to_string()),
            )
            .replace("{name}", params.name.as_ref().unwrap_or(&"".to_string()))
            .replace(
                "{repository}",
                params.repository.as_ref().unwrap_or(&"".to_string()),
            )
            .replace(
                "{rustversion}",
                params.rustversion.as_ref().unwrap_or(&"".to_string()),
            )
            .replace(
                "{version}",
                params.version.as_ref().unwrap_or(&"".to_string()),
            )
            .replace(
                "{website}",
                params.website.as_ref().unwrap_or(&"".to_string()),
            );
        writeln!(output, "{}", replaced_line)?;
    }
    Ok(())
}
