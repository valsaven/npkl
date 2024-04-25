use crate::libs::get_path_size::get_path_size;

pub fn get_node_item(path: &str) -> String {
    let size = get_path_size(path);
    format!("{:<11}|  {}", size, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_temp_dir_with_file(dir: &tempfile::TempDir, file_name: &str, size: usize) -> String {
        let dir_path = dir.path().join("test_dir");
        std::fs::create_dir_all(&dir_path).unwrap();
        let file_path = dir_path.join(file_name);
        let mut file = File::create(file_path).unwrap();
        for _ in 0..size {
            file.write_all(b"0").unwrap();
        }
        dir_path.to_str().unwrap().to_string()
    }

    #[test]
    fn get_node_item_returns_correct_format_for_zero_size() {
        let dir = tempdir().unwrap();
        let path = create_temp_dir_with_file(&dir, "empty_file.txt", 0);
        let result = get_node_item(&path);
        assert_eq!(result, "0 B        |  ".to_string() + &path);
    }

    #[test]
    fn get_node_item_returns_correct_format_for_small_size() {
        let dir = tempdir().unwrap();
        let path = create_temp_dir_with_file(&dir, "small_file.txt", 10 * 1024); // 10 KB
        let result = get_node_item(&path);
        assert_eq!(result, "10.00 KiB  |  ".to_string() + &path);
    }

    #[test]
    fn get_node_item_returns_correct_format_for_medium_size() {
        let dir = tempdir().unwrap();
        let path = create_temp_dir_with_file(&dir, "medium_file.txt", 1024 * 1024); // 1 MB
        let result = get_node_item(&path);
        assert_eq!(result, "1.00 MiB   |  ".to_string() + &path);
    }

    #[test]
    fn get_node_item_returns_correct_format_for_large_size() {
        let dir = tempdir().unwrap();
        let path = create_temp_dir_with_file(&dir, "large_file.txt", 10 * 1024 * 1024); // 10 MB
        let result = get_node_item(&path);
        assert_eq!(result, "10.00 MiB  |  ".to_string() + &path);
    }
}
