// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2023-2024 LibMake. All rights reserved.

use std::{
    error::Error,
    fs::{self},
    path::{Path, PathBuf},
};
/// Ensures a directory exists, creating it if necessary.
///
/// This function takes a reference to a `Path` object for a directory and a
/// human-readable name for the directory, and creates the directory if it
/// does not already exist.
///
/// # Arguments
///
/// * `dir` - A reference to a `Path` object for the directory.
/// * `name` - A human-readable name for the directory, used in error messages.
///
/// # Returns
///
/// * `Result<(), String>` - A result indicating success or failure.
///     - `Ok(())` if the directory exists or was created successfully.
///     - `Err(String)` if the directory does not exist and could not be created.
///
/// # Example
///
/// ```
/// use libmake::utilities::directory::directory;
/// use std::path::Path;
/// use std::fs;
///
/// // Create a "logs" directory if it doesn't exist
/// let dir = Path::new("logs");
/// directory(dir, "logs").expect("Could not create logs directory");
/// fs::remove_dir_all(dir).expect("Could not remove logs directory");
/// ```
///
pub fn directory(dir: &Path, name: &str) -> Result<String, String> {
    if dir.exists() {
        if !dir.is_dir() {
            return Err(format!(
                "❌ Error: {} is not a directory.",
                name
            ));
        }
    } else {
        match fs::create_dir_all(dir) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!(
                    "❌ Error: Cannot create {} directory: {}",
                    name, e
                ))
            }
        }
    }
    Ok(String::new())
}

/// Moves the output directory to the public directory.
///
/// This function takes a reference to a `Path` object for the output directory
/// and a string for the site name, and moves the output directory to the
/// public directory.
///
/// # Arguments
///
/// * `site_name` - A string for the site name.
/// * `out_dir` - A reference to a `Path` object for the output directory.
///
/// # Returns
///
/// * `Result<(), std::io::Error>` - A result indicating success or failure.
///     - `Ok(())` if the output directory was moved successfully.
///     - `Err(std::io::Error)` if the output directory could not be moved.
///
pub fn move_output_directory(
    site_name: &str,
    out_dir: &Path,
) -> std::io::Result<()> {
    println!("❯ Moving output directory...");

    let public_dir = Path::new("public");

    if public_dir.exists() {
        fs::remove_dir_all(public_dir)?;
    }

    fs::create_dir(public_dir)?;

    let site_name = site_name.replace(' ', "_");
    let new_project_dir = public_dir.join(site_name);
    fs::create_dir_all(&new_project_dir)?;

    fs::rename(out_dir, &new_project_dir)?;

    println!("  Done.\n");

    Ok(())
}

/// Cleans up the directory at the given path.
///
/// If the directory does not exist, this function does nothing.
///
/// # Arguments
///
/// * `directories` - An array of references to `Path` objects representing the
///    directories to be cleaned up.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - A result indicating success or failure.
///     - `Ok(())` if the directories were cleaned up successfully.
///     - `Err(Box<dyn Error>)` if an error occurred during the cleanup process.
///
pub fn cleanup_directory(
    directories: &[&Path],
) -> Result<(), Box<dyn Error>> {
    for directory in directories {
        if !directory.exists() {
            continue;
        }

        println!("\n❯ Cleaning up directories");

        fs::remove_dir_all(directory)?;

        println!("  Done.\n");
    }

    Ok(())
}

/// Creates a new directory at the given path.
///
/// If the directory already exists, this function does nothing.
///
/// # Arguments
///
/// * `directories` - An array of references to `Path` objects representing the
///    directories to be created.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - A result indicating success or failure.
///     - `Ok(())` if the directories were created successfully.
///     - `Err(Box<dyn Error>)` if an error occurred during the creation process.
///
pub fn create_directory(
    directories: &[&Path],
) -> Result<(), Box<dyn Error>> {
    for directory in directories {
        if directory.exists() {
            continue;
        }

        fs::create_dir(directory)?;
    }

    Ok(())
}

/// Truncates a path to only have a set number of path components.
///
/// Will truncate a path to only show the last `length` components in a path.
/// If a length of `0` is provided, the path will not be truncated.
/// A value will only be returned if the path has been truncated.
///
/// # Arguments
///
/// * `path` - The path to truncate.
/// * `length` - The number of path components to keep.
///
/// # Returns
///
/// * An `Option` of the truncated path as a string. If the path was not truncated, `None` is returned.
pub fn truncate(path: &Path, length: usize) -> Option<String> {
    // Checks if the length is 0. If it is, returns `None`.
    if length == 0 {
        return None;
    }

    // Creates a new PathBuf object to store the truncated path.
    let mut truncated = PathBuf::new();

    // Iterates over the components of the path in reverse order.
    let mut count = 0;
    while let Some(component) = path.components().next_back() {
        // Adds the component to the truncated path.
        truncated.push(component);
        count += 1;

        // If the count reaches the desired length, breaks out of the loop.
        if count == length {
            break;
        }
    }

    // If the count is equal to the desired length, returns the truncated path as a string.
    if count == length {
        Some(truncated.to_string_lossy().to_string())
    } else {
        // Otherwise, returns `None`.
        None
    }
}
