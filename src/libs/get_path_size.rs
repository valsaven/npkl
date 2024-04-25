use byte_unit::{Byte, UnitType};
use walkdir::WalkDir;

pub fn get_path_size(path: &str) -> String {
    let total_size = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .map(|metadata| metadata.len())
        .sum::<u64>();

    format!("{:.2}", Byte::from_u64(total_size).get_appropriate_unit(UnitType::Binary))
}
