use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use super::generator::FileGenerationParams;

/// Replaces placeholders in a template file with values from a set of
/// parameters
///
/// # Arguments
///
/// * `template_file` - A `PathBuf` representing the path to the input template file
/// * `output_file` - A `PathBuf` representing the path to the output file that will be created
/// * `params` - A reference to a `FileGenerationParams` struct containing the values to be
///     substituted into the placeholders
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
            .replace("{build}", params.build.as_ref().unwrap_or(&"".to_string()))
            .replace(
                "{categories}",
                params.categories.as_ref().unwrap_or(&"".to_string()),
            )
            .replace("{csv}", params.csv.as_ref().unwrap_or(&"".to_string()))
            .replace(
                "{description}",
                params.description.as_ref().unwrap_or(&"".to_string()),
            )
            .replace(
                "{documentation}",
                params.documentation.as_ref().unwrap_or(&"".to_string()),
            )
            .replace(
                "{edition}",
                params.edition.as_ref().unwrap_or(&"".to_string()),
            )
            .replace("{email}", params.email.as_ref().unwrap_or(&"".to_string()))
            .replace(
                "{homepage}",
                params.homepage.as_ref().unwrap_or(&"".to_string()),
            )
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
