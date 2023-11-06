#[cfg(test)]
mod tests {
    // Import the generate_ascii_art function from the libmake::ascii module
    use libmake::ascii::generate_ascii_art;

    // Define a unit test named test_generate_ascii_art
    #[test]
    fn test_generate_ascii_art() {
        // Create a test string
        let text = "Hello, world!";

        // Call the generate_ascii_art function with the test string and assert that no panic occurs
        assert!(std::panic::catch_unwind(|| {
            generate_ascii_art(text).unwrap();
        })
        .is_ok());
    }
}
