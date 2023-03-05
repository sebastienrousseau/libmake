#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use libmake::run;

    #[test]
    fn test_main() {
        let mut cmd = Command::cargo_bin("libmake").unwrap();
        let assert = cmd.assert();
        assert.success();
    }

    #[test]
    fn test_main_run() {
        let result = run();
        assert!(result.is_ok());
    }
}
