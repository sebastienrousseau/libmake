use libmake::generator::{
    create_directory, generate_files, generate_from_args,
};
use libmake::{
    assert_generate_files,
    generator::{
        generate_from_config, generate_from_yaml, FileGenerationParams,
    },
    utils::{get_csv_field, get_json_field, get_yaml_field},
};
use tempfile::tempdir;

use std::path::Path;
use std::{env, io};

/// Tests the `get_csv_field` function by passing a CSV file path and a
/// field index and checking if the function returns the expected value.
#[test]
fn test_get_csv_field() {
    let file_path = "./tests/data/mylibrary.csv";
    let field_author_index = 0;
    let value = get_csv_field(Some(file_path), field_author_index);
    assert_eq!(value, Some(vec!["Me".to_string()]));
}

/// Tests the `get_json_field` function by passing a JSON file path and
/// a field name and checking if the function returns the expected value.
#[test]
fn test_get_json_field() {
    let file_path = "./tests/data/mylibrary.json";
    let field_name = "mylibrary";
    let value = if Path::new(file_path).exists() {
        get_json_field(Some(file_path), field_name)
    } else {
        Ok(String::new()) // Wrap the String in Ok
    };
    assert_eq!(value.unwrap(), "null".to_string()); // Unwrap the value and compare with "null"
}

/// Tests the `get_yaml_field` function by passing a YAML file path and
/// a field name and checking if the function returns the expected value.
#[test]
fn test_get_yaml_field() {
    let file_path = "./tests/data/mylibrary.yaml";
    let field_name = "mylibrary";
    let value = if Path::new(file_path).exists() {
        get_yaml_field(Some(file_path), field_name)
    } else {
        Ok(String::new()) // Wrapping the String in Ok to match the expected type
    };
    assert_eq!(value.unwrap(), "null".to_string()); // Unwrap the value and compare with "null"
}

/// Tests the `generate_from_config` function by passing a YAML file
/// path and a file type, and checking if the function runs without errors.
#[test]
fn test_generate_from_config() {
    let file_path = "./tests/data/mylibrary.yaml";
    let file_type = "yaml";
    generate_from_config(file_path, file_type).unwrap();
    assert_eq!(true, true); // If we get here without panicking, the test has passed
}

/// Tests the `generate_from_config` function by passing a CSV file path
/// and a file type, and checking if the function runs without errors.
#[test]
fn test_generate_from_csv() {
    let file_path = "./tests/data/mylibrary.csv";
    generate_from_config(file_path, "csv").unwrap();
    assert_eq!(true, true); // If we get here without panicking, the test has passed
}

/// Tests the `generate_from_config` function by passing a JSON file
/// path and a file type, and checking if the function runs without errors.
#[test]
fn test_generate_from_json() {
    let file_path = "./tests/data/mylibrary.json";
    generate_from_config(file_path, "json").unwrap();
    assert_eq!(true, true); // If we get here without panicking, the test has passed
}

/// Tests the `generate_from_yaml` function by passing a YAML file path
/// and checking if the function runs without errors.
#[test]
fn test_generate_from_yaml() {
    let file_path = "./tests/data/mylibrary.yaml";
    generate_from_yaml(file_path).unwrap();
    assert_eq!(true, true); // If we get here without panicking, the test has passed
}

/// Tests the `generate_from_toml` function by passing a TOML file path
/// and checking if the function runs without errors.
#[test]
fn generate_from_toml() {
    let file_path = "./tests/data/mylibrary.toml";
    generate_from_config(file_path, "toml").unwrap();
    assert_eq!(true, true); // If we get here without panicking, the test has passed
}

/// Tests the `generate_from_args` function by passing a string of
/// arguments and checking if the function runs without errors.
#[test]
fn test_generate_from_args() {
    let args = "--author=Me --output=my_library"
        .split(' ')
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    let args_str = args[1..].join(" ");
    let result = generate_from_args(&args_str);

    assert!(result.is_ok());
}

/// Tests the `from_args` function by passing a string of arguments and
/// checking if the function runs without errors.
#[test]
fn test_from_args() {
    let args = "--author=Me \
                                --build=build.rs \
                                --categories=[cat1,cat2] \
                                --description='test' \
                                --documentation= \
                                --edition=2018 \
                                --email='test@test.com' \
                                --homepage= \
                                --keywords= \
                                --license=MIT \
                                --output=my_library \
                                --readme= \
                                --repository= \
                                --rustversion= \
                                --version= \
                                --website="
        .split(' ')
        .map(ToString::to_string) // Replaced the closure with the method directly
        .collect::<Vec<String>>();

    let args_str = args[1..].join(" ");
    let result = generate_from_args(&args_str);

    assert!(result.is_ok());
}

/// Tests the `generate_files` function by passing a
/// `FileGenerationParams` struct and checking if the function runs
/// without errors.
#[test]
fn test_assert_generate_files() {
    let temp_dir = env::temp_dir().join("my_library");
    let mut params = FileGenerationParams::new();
    params.output =
        Some(temp_dir.as_path().to_str().unwrap().to_owned());
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_generate_files() {
    // Create a temporary directory
    let temp_directory = tempdir();
    let temp_path = temp_directory.unwrap().path().to_owned();

    // Set up the parameters for file generation
    let mut params = FileGenerationParams::new();
    params.output = Some(temp_path.to_str().unwrap().to_owned());

    // Call the function you want to test
    assert_generate_files!(params.clone());

    // Assert that the temporary directory exists
    assert!(temp_path.exists());

    // Clean up: Remove the temporary directory and its contents
    std::fs::remove_dir_all(temp_path).unwrap();
}

/// Tests the `create_directory` function by passing an invalid path and
/// checking if the function returns an error.
#[test]
fn test_create_directory_error() -> io::Result<()> {
    // Create a temporary directory for testing
    let temp_dir = env::current_dir()?.join("valid_path");
    std::fs::create_dir(&temp_dir)?;

    // Attempt to create the directory again using create_directory with an invalid path
    let result = create_directory(&temp_dir.join("invalid_path/test"));

    // Verify that the function returns an error
    assert!(result.is_err());

    // Clean up the temporary directory
    std::fs::remove_dir(&temp_dir)?;

    Ok(())
}
