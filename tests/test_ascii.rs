#[cfg(test)]
mod tests {
    use libmake::ascii::generate_ascii_art;

    #[test]
    fn test_generate_ascii_art() {
        let text = "Hello, world!";
        assert!(std::panic::catch_unwind(|| {
            generate_ascii_art(text);
        })
        .is_ok());
    }
}
