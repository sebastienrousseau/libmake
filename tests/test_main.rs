#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use libmake::run;
    use std::io::Write;

    #[test]
    fn test_main() {
        let mut cmd = Command::cargo_bin("libmake").unwrap();
        let assert = cmd.assert();
        assert.failure();
    }

    #[test]
    fn test_main_run() {
        // Redirect stdout to a buffer so we can check it later
        let mut buffer = Vec::new();
        let result = {
            let _stdout = std::io::stdout();
            let mut handle = std::io::BufWriter::new(buffer.by_ref());
            std::io::stdout().flush().unwrap();
            std::io::stdout().flush().unwrap();
            let result = run();
            std::io::stdout().flush().unwrap();
            std::io::stdout().flush().unwrap();
            handle.flush().unwrap();
            result
        };
        let output = String::from_utf8(buffer).unwrap();
        assert_ne!(output, result.unwrap_err().to_string());
    }
}
