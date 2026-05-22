use crate::libs::get_path_size::get_path_size;
use std::fmt;
use std::path::{Path, PathBuf};

const MAX_SIZE_WIDTH: usize = 11;

pub struct NodeItem {
    pub path: PathBuf,
    pub size: String,
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
        write!(
            f,
            "{:>width$}|  {}",
            self.size,
            self.path.display(),
            width = MAX_SIZE_WIDTH
        )
    }
}
