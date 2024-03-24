#[cfg(test)]
mod tests {

    use test_lib::test_lib;

    #[test]
    fn test_test_lib() {
        let test_lib = test_lib::new();
        assert_eq!(test_lib, test_lib::default());
    }
}
