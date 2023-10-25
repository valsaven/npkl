mod libs;

use crate::libs::get_node_item::get_node_item;
use crate::libs::get_selected_item_path::get_selected_item_path;
use crate::libs::is_top_level_node_modules::is_top_level_node_modules;
use libs::print_logo::print_logo;
use std::fs;

use dialoguer::{console::Term, theme::ColorfulTheme, MultiSelect};
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    print_logo();

    let path = ".";
    let mut node_items = vec![];

    let mut it = WalkDir::new(path).into_iter();
    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(entry)) => entry,
        };

        if !entry.file_type().is_dir() {
            continue;
        }

        if is_top_level_node_modules(&entry) {
            if entry.file_type().is_dir() {
                let current_path = entry.path().to_str().unwrap().parse().unwrap();
                let node_item = get_node_item(current_path);

                node_items.push(node_item);

                // We don't need nested node_modules, so we skip them
                it.skip_current_dir();
            }
            continue;
        }
    }

    println!("Total elements:");
    println!("{}\n", node_items.len());

    // TODO: Show total size
    // println!("Total size:");
    // println!("{}\n", total_size);

    let selection_result = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select with CURSORS and SPACE. Press ENTER to delete")
        .items(&node_items)
        .interact_on_opt(&Term::stderr());

    let selection = match selection_result {
        Ok(val) => val,
        Err(dialoguer_error) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, dialoguer_error.to_string()));
        }
    };

    match selection {
        Some(positions) => {
            for index in &positions {
                let selected_item = &node_items[*index];
                let selected_item_path = get_selected_item_path(selected_item);

                fs::remove_dir_all(selected_item_path)?;
            }
        },
        None => println!("User exited using Esc or q"),
    }

    Ok(())
}
