use std::fs::File;
use std::path::Path;

use csv::ReaderBuilder;

/// Reads a CSV file at the given file path and returns the value of
/// the given field.
///
/// # Arguments
///
/// * `file_path` - An optional string slice that holds the file path of the CSV file to read.
/// * `field_name` - A string slice that holds the name of the field to retrieve.
///
pub fn get_csv_field(file_path: Option<&str>, field_name: &str) -> String {
    if let Some(file_path) = file_path {
        println!("file_path = {}", file_path);

        let file = File::open(Path::new(file_path)).unwrap();
        let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
        let record = reader.records().next().unwrap().unwrap();
        let index = record.iter().position(|f| f == field_name).unwrap();
        record.get(index).unwrap().to_string()
    } else {
        "".to_string()
    }
}
