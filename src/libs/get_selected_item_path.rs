pub fn get_selected_item_path(selected_item: &str) -> Option<&str> {
    selected_item.rsplit("|  ").next()
}
