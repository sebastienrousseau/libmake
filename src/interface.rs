// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use super::generator::FileGenerationParams;

/// Replaces placeholders in a template file with values from the provided parameters
/// and writes the result to an output file.
///
/// # Arguments
///
/// * `template_file` - Path to the template file.
/// * `output_file` - Path to the output file where the result should be written.
/// * `params` - Parameters containing values to replace placeholders in the template.
///
/// # Errors
///
/// Returns an `std::io::Result` error if:
///
/// - The template file cannot be read.
/// - The output file cannot be created or written to.
/// - There are issues parsing the template or replacing placeholders.
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
                params.author.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{build}",
                params.build.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{categories}",
                params.categories.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{description}",
                params.description.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{documentation}",
                params.documentation.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{edition}",
                params.edition.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{email}",
                params.email.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{homepage}",
                params.homepage.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{keywords}",
                params.keywords.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{license}",
                params.license.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{name}",
                params.name.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{repository}",
                params.repository.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{rustversion}",
                params.rustversion.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{version}",
                params.version.as_ref().unwrap_or(&String::new()),
            )
            .replace(
                "{website}",
                params.website.as_ref().unwrap_or(&String::new()),
            );
        writeln!(output, "{replaced_line}")?;
    }
    Ok(())
}
