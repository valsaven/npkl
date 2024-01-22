use byte_unit::{Byte, UnitType};
use walkdir::WalkDir;

pub fn get_path_size(path: &str) -> String {
    let total_size = WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    let byte = Byte::from_u64(total_size);
    let adjusted_byte = byte.get_appropriate_unit(UnitType::Binary);

    format!("{:.2}", adjusted_byte)
}
