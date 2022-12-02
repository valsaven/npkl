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
