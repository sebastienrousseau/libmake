#[macro_export]
/// Asserts create directory.
macro_rules! assert_create_directory {
    ($path:expr) => {
        assert!(create_directory(Path::new($path)).is_ok());
        assert!(create_directory(Path::new($path)).is_ok());
    };
}
#[macro_export]
/// Asserts generate files.
macro_rules! assert_generate_files {
    ($params:expr) => {
        assert!(generate_files($params).is_ok());
    };
}
#[macro_export]
/// Asserts generate files from CSV.
macro_rules! assert_generate_files_from_csv {
    ($csv_path:expr) => {
        assert!(generate_files_from_csv($csv_path).is_ok());
    };
}
#[macro_export]
/// Asserts generate via JSON.
macro_rules! assert_generate_via_json {
    ($path:expr) => {
        assert!(generate_via_json($path).is_ok());
    };
}
#[macro_export]
/// Asserts generate via YAML.
macro_rules! assert_generate_via_yaml {
    ($path:expr) => {
        assert!(generate_via_yaml($path).is_ok());
    };
}
