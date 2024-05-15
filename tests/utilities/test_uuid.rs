#[cfg(test)]
mod tests {
    use libmake::utilities::uuid::generate_unique_string;
    use uuid::Uuid;

    #[test]
    fn test_generate_unique_string_format() {
        let unique_string = generate_unique_string();
        let uuid = Uuid::parse_str(&unique_string);
        assert!(uuid.is_ok(), "Generated string is not a valid UUID");
    }

    #[test]
    fn test_generate_unique_string_uniqueness() {
        let unique_string1 = generate_unique_string();
        let unique_string2 = generate_unique_string();
        assert_ne!(unique_string1, unique_string2, "Generated strings are not unique");
    }
}
