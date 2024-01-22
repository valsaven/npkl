pub fn get_selected_item_path(selected_item: &str) -> &str {
    selected_item
        .rsplit("|  ")
        .next()
        .expect("The path is empty")
}
