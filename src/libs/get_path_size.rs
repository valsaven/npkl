use byte_unit::{Byte, UnitType};
use rayon::prelude::*;
use walkdir::WalkDir;

pub fn get_path_size(path: &str) -> String {
    let total_size: u64 = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .par_bridge()
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum::<u64>();

    let byte = Byte::from_u64(total_size);
    let adjusted_byte = byte.get_appropriate_unit(UnitType::Binary);

    format!("{:.2}", adjusted_byte)
}
