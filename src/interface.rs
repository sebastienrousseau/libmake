use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

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
    author: &Option<String>,
    categories: &Option<String>,
    csv: &Option<String>,
    description: &Option<String>,
    email: &Option<String>,
    keywords: &Option<String>,
    license: &Option<String>,
    name: &Option<String>,
    repository: &Option<String>,
    rustversion: &Option<String>,
    version: &Option<String>,
    website: &Option<String>,
) -> std::io::Result<()> {
    let tpl = File::open(template_file)?;
    let tpl_reader = BufReader::new(tpl);
    let mut output = File::create(output_file)?;
    let tpl_lines = tpl_reader.lines();
    for line in tpl_lines {
        let line = line?;
        let replaced_line = line
            .replace("{author}", author.as_ref().unwrap_or(&"".to_string()))
            .replace(
                "{categories}",
                categories.as_ref().unwrap_or(&"".to_string()),
            )
            .replace("{csv}", csv.as_ref().unwrap_or(&"".to_string()))
            .replace(
                "{description}",
                description.as_ref().unwrap_or(&"".to_string()),
            )
            .replace("{email}", email.as_ref().unwrap_or(&"".to_string()))
            .replace("{keywords}", keywords.as_ref().unwrap_or(&"".to_string()))
            .replace("{license}", license.as_ref().unwrap_or(&"".to_string()))
            .replace("{name}", name.as_ref().unwrap_or(&"".to_string()))
            .replace(
                "{repository}",
                repository.as_ref().unwrap_or(&"".to_string()),
            )
            .replace(
                "{rustversion}",
                rustversion.as_ref().unwrap_or(&"".to_string()),
            )
            .replace("{version}", version.as_ref().unwrap_or(&"".to_string()))
            .replace("{website}", website.as_ref().unwrap_or(&"".to_string()));
        writeln!(output, "{}", replaced_line)?;
    }
    Ok(())
}
