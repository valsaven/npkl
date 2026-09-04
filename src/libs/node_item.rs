use crate::libs::get_path_size::get_path_size;
use byte_unit::{Byte, UnitType};
use std::fmt;
use std::path::{Path, PathBuf};

const VALUE_WIDTH: usize = 8;
const UNIT_WIDTH: usize = 3;

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
        let size = format!("{size:.2}");
        // ponytail: AdjustedByte has no public value/unit accessors, so split the display string
        let (value, unit) = size.rsplit_once(' ').unwrap_or((size.as_str(), ""));
        write!(
            f,
            "{value:>width$} {unit:<unit_width$}  {}",
            self.path.display(),
            width = VALUE_WIDTH,
            unit_width = UNIT_WIDTH
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(size: u64) -> NodeItem {
        NodeItem {
            path: PathBuf::from("x/node_modules"),
            size,
        }
    }

    #[test]
    fn display_keeps_size_and_path_columns_aligned() {
        assert_eq!(format!("{}", item(512)), "     512 B    x/node_modules");
        assert_eq!(format!("{}", item(2048)), "    2.00 KiB  x/node_modules");
        assert_eq!(
            format!("{}", item(256 * 1024 * 1024)),
            "  256.00 MiB  x/node_modules"
        );
        assert_eq!(
            format!("{}", item(2 * 1024 * 1024 * 1024)),
            "    2.00 GiB  x/node_modules"
        );
    }
}
