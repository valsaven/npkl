use crate::libs::get_path_size::get_path_size;
use byte_unit::{Byte, UnitType};
use std::fmt;
use std::path::{Path, PathBuf};

const MAX_SIZE_WIDTH: usize = 11;

pub struct NodeItem {
    pub path: PathBuf,
    pub size: u64,
}

impl NodeItem {
    pub fn from_path(path: &Path) -> Self {
        NodeItem {
            path: path.to_path_buf(),
            size: get_path_size(path),
        }
    }
}

impl fmt::Display for NodeItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = Byte::from_u64(self.size).get_appropriate_unit(UnitType::Binary);
        write!(
            f,
            "{:>width$}|  {}",
            format!("{size:.2}"),
            self.path.display(),
            width = MAX_SIZE_WIDTH
        )
    }
}
