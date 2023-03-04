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
macro_rules! assert_generate_from_csv {
    ($csv_path:expr) => {
        assert!(generate_from_csv($csv_path).is_ok());
    };
}
#[macro_export]
/// Asserts generate from JSON.
macro_rules! assert_generate_from_json {
    ($path:expr) => {
        assert!(generate_from_json($path).is_ok());
    };
}
#[macro_export]
/// Asserts generate from YAML.
macro_rules! assert_generate_from_yaml {
    ($path:expr) => {
        assert!(generate_from_yaml($path).is_ok());
    };
}
#[macro_export]
/// Asserts generate from config.
macro_rules! assert_generate_from_config {
    ($path:expr, $file_type:expr) => {
        assert!(generate_from_config($path, $file_type).is_ok());
    };
}
