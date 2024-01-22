use crate::libs::get_path_size::get_path_size;

pub fn get_node_item(path: &str) -> String {
    let delimiter = "|  ";
    let size = get_path_size(path);
    let size_length = size.len();

    // Set the maximum expected length of the size string
    let max_size_length = 11;
    // Calculate the number of spaces based on the length of the string with size
    let spaces_after_size = std::cmp::max(0, max_size_length - size_length);

    let node_item = format!(
        "{}{}{}{}",
        size,
        " ".repeat(spaces_after_size),
        delimiter,
        path
    );

    node_item
}
