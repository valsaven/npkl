use crate::libs::get_path_size::get_path_size;
use std::path::Path;

// Fixed width for size (right-aligned: spaces on the left)
const MAX_SIZE_WIDTH: usize = 11; // Enough for "1024.00 GiB" or similar

pub fn get_node_item(path: &Path) -> String {
    let delimiter = "|  ";
    let size = get_path_size(path);
    let path_str = path.display();

    // Format the string: size is right-aligned, then delimiter and path
    let node_item = format!(
        "{:>width$}{}{}", // > for right-align, width$ takes MAX_SIZE_WIDTH
        size,
        delimiter,
        path_str,
        width = MAX_SIZE_WIDTH // Set width dynamically
    );

    node_item
}
