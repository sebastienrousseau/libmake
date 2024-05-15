#[cfg(test)]
mod tests {
    use libmake::generate_file;
    use std::cell::RefCell;

    // Mock generator function
    fn mock_generator_success(_: &str) -> Result<(), String> {
        Ok(())
    }

    // Mock generator function that returns an error
    fn mock_generator_error(_: &str) -> Result<(), String> {
        Err("Mock error".to_string())
    }

    thread_local! {
        static LAST_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
    }

    // Mock eprintln! macro to capture the error messages
    macro_rules! eprintln {
        ($($arg:tt)*) => {
            LAST_ERROR.with(|last_error| {
                *last_error.borrow_mut() = Some(format!($($arg)*));
            });
        };
    }

    #[test]
    fn test_generate_file_success() {
        generate_file!(
            "test",
            "non_empty_value",
            mock_generator_success
        );
        LAST_ERROR.with(|last_error| {
            assert!(last_error.borrow().is_none());
        });
    }

    #[test]
    fn test_generate_file_empty_value() {
        generate_file!("test", "", mock_generator_success);
        LAST_ERROR.with(|last_error| {
            assert!(last_error.borrow().is_none());
        });
    }

    #[test]
    fn test_generate_file_error() {
        generate_file!("test", "non_empty_value", mock_generator_error);
        LAST_ERROR.with(|last_error| {
            assert_eq!(
                *last_error.borrow(),
                Some(
                    "Error generating test file: Mock error"
                        .to_string()
                )
            );
        });
    }
}
