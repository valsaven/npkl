pub fn get_selected_item_path(selected_item: &String) -> String {
    let split = selected_item.split("|  ");

    if let Some(val) = split.last() {
        return val.to_string();
    } else {
        panic!("The path is empty");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_selected_item_path_valid_input() {
        let input = String::from("item1|  item2");
        let result = get_selected_item_path(&input);
        assert_eq!(result, "item2");
    }
    //
    // #[test]
    // #[should_panic(expected = "The path is empty")]
    // fn test_get_selected_item_path_no_delimiter() {
    //     let input = String::from("item1 item2 item3");
    //     get_selected_item_path(&input);
    // }
    //
    // #[test]
    // #[should_panic(expected = "The path is empty")]
    // fn test_get_selected_item_path_empty_string() {
    //     let input = String::from("");
    //     get_selected_item_path(&input);
    // }
}
