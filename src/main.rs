mod libs;

use crate::libs::get_node_item::get_node_item;
use crate::libs::get_selected_item_path::get_selected_item_path;
use libs::print_logo::print_logo;
use std::fs;
use std::path::PathBuf;
use dialoguer::{console::Term, theme::ColorfulTheme, MultiSelect};
use walkdir::{WalkDir};

fn main() -> std::io::Result<()> {
    print_logo();

    let path = PathBuf::from(".");
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

        if entry.file_name().to_str() == Some("node_modules") {
            let current_path = entry.path().to_str().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid path")
            })?;
            let node_item = get_node_item(current_path);
            node_items.push(node_item);

            in_node_modules = true; // Set the flag that we are inside node_modules
        }
    }

    println!("Total elements:");
    println!("{}\n", node_items.len());

    let selection_result = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select with CURSORS and SPACE. Press ENTER to delete\n")
        .items(&node_items)
        .report(false)
        .clear(true)
        .interact_on_opt(&Term::stderr());

    match selection_result {
        Ok(Some(positions)) => {
            println!("Selected items:");
            for index in &positions {
                let selected_item = &node_items[*index];
                println!("{}", selected_item);
                let selected_item_path = get_selected_item_path(selected_item);
                fs::remove_dir_all(selected_item_path)?;
            }
        },
        Ok(None) => println!("User exited using Esc or q"),
        Err(dialoguer_error) => {
            eprintln!("Dialoguer error: {}", dialoguer_error);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, dialoguer_error.to_string()));
        }
    }

    Ok(())
}
