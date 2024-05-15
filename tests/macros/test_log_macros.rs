#[cfg(test)]
mod tests {
    use libmake::macro_log_info;

    struct DateTime {
        iso_8601: String,
    }

    impl DateTime {
        fn new() -> Self {
            DateTime {
                iso_8601: "2024-05-15T12:34:56Z".to_string(),
            }
        }
    }

    struct Random;

    impl Random {
        fn default() -> Self {
            Random
        }

        fn rand(&mut self) -> u64 {
            42 // Mock random number generator returns a fixed value
        }
    }

    struct Log {
        session_id: String,
        iso_8601: String,
        level: String,
        component: String,
        description: String,
        format: String,
    }

    impl Log {
        fn new(
            session_id: &str,
            iso_8601: &str,
            level: &str,
            component: &str,
            description: &str,
            format: &str,
        ) -> Self {
            Log {
                session_id: session_id.to_string(),
                iso_8601: iso_8601.to_string(),
                level: level.to_string(),
                component: component.to_string(),
                description: description.to_string(),
                format: format.to_string(),
            }
        }
    }

    #[test]
    fn test_macro_log_info() {
        let log = macro_log_info!(
            "INFO",
            "TestComponent",
            "This is a test log",
            "TestFormat"
        );

        assert_eq!(log.session_id, "42");
        assert_eq!(log.iso_8601, "2024-05-15T12:34:56Z");
        assert_eq!(log.level, "INFO");
        assert_eq!(log.component, "TestComponent");
        assert_eq!(log.description, "This is a test log");
        assert_eq!(log.format, "TestFormat");
    }
}
