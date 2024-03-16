#[cfg(test)]
mod tests {
    // Import the generate_ascii_art function from the libmake::ascii module
    use libmake::ascii::generate_ascii_art;
    use std::error::Error;

    #[test]
    fn test_generate_ascii_art() -> Result<(), Box<dyn Error>> {
        // Create a test string
        let text = "Hello, world!";

        // Call the generate_ascii_art function with the test string and handle the result
        match generate_ascii_art(text) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()), // Convert the error to a boxed dyn Error
        }
    }
}
