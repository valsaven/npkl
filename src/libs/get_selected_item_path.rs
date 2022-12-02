pub fn get_selected_item_path(selected_item: &String) -> String {
    let split = selected_item.split("|  ");

    if let Some(val) = split.last() {
        return val.to_string();
    } else {
        panic!("The path is empty");
    }
}
