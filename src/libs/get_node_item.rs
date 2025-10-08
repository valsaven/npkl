use crate::libs::get_path_size::get_path_size;

pub fn get_node_item(path: &str) -> String {
    let delimiter = "|  ";
    let size = get_path_size(path);

    // Fixed width for size (right-aligned: spaces on the left)
    let max_size_width = 11; // Enough for "1024.00 GiB" or similar

    // Format the string: size is right-aligned, then delimiter and path
    let node_item = format!(
        "{:>width$}{}{}", // > for right-align, width$ takes max_size_width
        size,
        delimiter,
        path,
        width = max_size_width // Set width dynamically
    );

    node_item
}
