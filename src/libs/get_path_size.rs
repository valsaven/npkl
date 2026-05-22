use byte_unit::{Byte, UnitType};
use std::path::Path;
use walkdir::WalkDir;

pub fn get_path_size(path: &Path) -> String {
    let total_size: u64 = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum();

    let adjusted = Byte::from_u64(total_size).get_appropriate_unit(UnitType::Binary);
    format!("{adjusted:.2}")
}
