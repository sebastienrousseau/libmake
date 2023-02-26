#[cfg(test)]
mod tests {
    use libmake::ascii::generate_ascii_art;

    #[test]
    fn test_generate_ascii_art() {
        let text = "Hello, world!";
        let font_file = "./resources/standard.flf";
        assert!(std::panic::catch_unwind(|| {
            generate_ascii_art(text, font_file);
        })
        .is_ok());
    }
}
