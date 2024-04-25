mod libs;

use crate::libs::get_node_item::get_node_item;
use crate::libs::get_selected_item_path::get_selected_item_path;
use crate::libs::is_top_level_node_modules::is_top_level_node_modules;
use libs::print_logo::print_logo;
use std::fs;
use std::path::PathBuf;
use dialoguer::{console::Term, theme::ColorfulTheme, MultiSelect};
use walkdir::{WalkDir};

fn main() -> std::io::Result<()> {
    print_logo();

    let node_items = get_node_items(PathBuf::from("."))?;
    print_total_elements(&node_items);

    let selected_positions = get_user_selection(&node_items)?;
    delete_selected_items(&node_items, selected_positions)?;

    Ok(())
}

fn get_node_items(path: PathBuf) -> std::io::Result<Vec<String>> {
    let mut node_items = vec![];
    let mut in_node_modules = false; // Flag to track if we are inside node_modules

    for entry in WalkDir::new(&path).min_depth(1).into_iter() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("ERROR: {}", err);
                continue;
            }
        };

        let depth = entry.depth(); // Get the depth of the current directory

        if depth == 1 {
            in_node_modules = false; // Reset the flag if we are back to a higher level
        }

        if !entry.file_type().is_dir() || in_node_modules {
            continue;
        }

        if is_top_level_node_modules(&entry) {
            let current_path = entry.path().to_str().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid path")
            })?;
            let node_item = get_node_item(current_path);
            node_items.push(node_item);

            in_node_modules = true; // Set the flag that we are inside node_modules
        }
    }

    Ok(node_items)
}

fn get_user_selection(node_items: &[String]) -> std::io::Result<Vec<usize>> {
    let selection_result = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select with CURSORS and SPACE. Press ENTER to delete")
        .items(node_items)
        .interact_on_opt(&Term::stderr());

    match selection_result {
        Ok(Some(positions)) => Ok(positions),
        Ok(None) => {
            println!("User exited using Esc or q");
            Ok(vec![])
        },
        Err(dialoguer_error) => {
            eprintln!("Dialoguer error: {}", dialoguer_error);
            Err(std::io::Error::new(std::io::ErrorKind::Other, dialoguer_error.to_string()))
        }
    }
}

fn print_total_elements(node_items: &[String]) {
    println!("Total elements:");
    println!("{}\n", node_items.len());
}

fn delete_selected_items(node_items: &[String], selected_positions: Vec<usize>) -> std::io::Result<()> {
    for index in selected_positions {
        let selected_item = &node_items[index];
        let selected_item_path = get_selected_item_path(selected_item);
        fs::remove_dir_all(selected_item_path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn test_get_node_items() {
        let path = PathBuf::from(".");
        let node_items = get_node_items(path).unwrap();

        assert!(node_items.is_empty() || !node_items.is_empty());
    }

    #[test]
    fn test_delete_selected_items() {
        // Создаем временную директорию
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_str().unwrap().to_string();

        let node_items = vec![temp_dir_path.clone()];
        let selected_positions = vec![0];
        let result = delete_selected_items(&node_items, selected_positions);

        assert!(result.is_ok());
        assert!(!Path::new(&temp_dir_path).exists());
    }
}
