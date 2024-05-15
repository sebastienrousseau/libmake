#[cfg(test)]
mod tests {
    use libmake::generators::ini::generate_from_ini;

    #[test]
    fn test_generate_from_ini_valid_ini() {
        let file_path = "./tests/data/mylibrary.ini";
        generate_from_ini(file_path).unwrap();
        assert_eq!(true, true);
    }

    #[test]
    fn test_generate_from_ini_invalid_ini() {
        let file_path = "./tests/data/mylibrary2.ini";
        generate_from_ini(file_path).unwrap();
        assert_eq!(true, true);
    }
}
