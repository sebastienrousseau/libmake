#[cfg(test)]
mod tests {
    use libmake::utilities::directory::directory;
    use std::fs;
    use std::io::Error;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_directory_exists() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("logs");

        fs::create_dir(&path).unwrap();
        let result = directory(&path, "logs");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_directory_create() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("logs");

        let result = directory(&path, "logs");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
        assert!(path.exists());
    }

    #[test]
    fn test_directory_not_a_directory() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("not_a_directory");

        fs::File::create(&file_path).unwrap();
        let result = directory(&file_path, "not_a_directory");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "❌ Error: not_a_directory is not a directory.");
    }

    #[test]
    fn test_move_output_directory() {
        let dir = tempdir().unwrap();
        let out_dir = dir.path().join("out");

        fs::create_dir(&out_dir).unwrap();
        let result = move_output_directory("site_name", &out_dir);
        assert!(result.is_ok());

        let public_dir = dir.path().join("public");
        let new_project_dir = public_dir.join("site_name");
        assert!(new_project_dir.exists());
    }

    #[test]
    fn test_cleanup_directory() {
        let dir = tempdir().unwrap();
        let cleanup_dir = dir.path().join("cleanup");

        fs::create_dir(&cleanup_dir).unwrap();
        let result = cleanup_directory(&[cleanup_dir.as_path()]);
        assert!(result.is_ok());
        assert!(!cleanup_dir.exists());
    }

    #[test]
    fn test_create_directory() {
        let dir = tempdir().unwrap();
        let create_dir = dir.path().join("create");

        let result = create_directory(&[create_dir.as_path()]);
        assert!(result.is_ok());
        assert!(create_dir.exists());
    }

    #[test]
    fn test_truncate_path() {
        let path = Path::new("/a/b/c/d/e");

        let result = truncate(&path, 3);
        assert_eq!(result, Some("c/d/e".to_string()));

        let result = truncate(&path, 0);
        assert_eq!(result, None);

        let result = truncate(&path, 10);
        assert_eq!(result, None);
    }
}
