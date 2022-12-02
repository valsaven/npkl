use crate::libs::get_path_size::get_path_size;

pub fn get_node_item(path: String) -> String {
    let delimiter = "|  ";
    let size = get_path_size(&*path);
    let mut spaces_after_size = 4;

    match size.len() {
        4 => spaces_after_size = 8,
        5 => spaces_after_size = 7,
        6 => spaces_after_size = 6,
        7 => spaces_after_size = 5,
        8 => spaces_after_size = 4,
        9 => spaces_after_size = 3,
        10 => spaces_after_size = 2,
        11 => spaces_after_size = 1,
        _ => println!("Wrong size."),
    }

    let node_item = [
        size,
        " ".repeat(spaces_after_size),
        delimiter.to_string(),
        path.to_string(),
    ]
    .join("");

    return node_item;
}
