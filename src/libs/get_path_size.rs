use byte_unit::Byte;
use walkdir::WalkDir;

pub fn get_path_size(path: &str) -> String {
    let total_size = WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    let byte = Byte::from_bytes(total_size.into());
    let adjusted_byte = byte.get_appropriate_unit(false);

    return adjusted_byte.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_get_path_size() {
        // Создание временной директории
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Создание двух файлов разного размера
        let mut file1 = File::create(dir_path.join("file1.txt")).unwrap();
        file1.write_all(b"Hello, world!").unwrap(); // 13 байт

        let mut file2 = File::create(dir_path.join("file2.txt")).unwrap();
        file2.write_all(b"Rust is awesome!").unwrap(); // 16 байт

        // Общий размер файлов = 29 байт. Ожидаемый результат: "29 B".
        let expected_size = "29 B";
        let actual_size = get_path_size(&dir_path.to_string_lossy());

        assert_eq!(expected_size, actual_size, "Sizes do not match!");
    }
}
