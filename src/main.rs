mod libs;

use crate::libs::node_item::NodeItem;
use dialoguer::{console::Term, theme::ColorfulTheme, MultiSelect};
use indicatif::{ProgressBar, ProgressStyle};
use libs::print_logo::print_logo;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    print_logo();

    let path = PathBuf::from(".");

    // Fast scan: find all node_modules without calculating sizes
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut walk_iter = WalkDir::new(&path).min_depth(1).into_iter();
    while let Some(entry) = walk_iter.next() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("ERROR: {err}");
                continue;
            }
        };

        if entry.file_type().is_dir() && entry.file_name() == "node_modules" {
            paths.push(entry.path().to_path_buf());
            walk_iter.skip_current_dir();
        }
    }

    if paths.is_empty() {
        println!("No node_modules found.");
        return Ok(());
    }

    // Calculate sizes with a progress bar
    let pb = ProgressBar::new(paths.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut node_items = vec![];
    for path in &paths {
        pb.set_message(path.display().to_string());
        node_items.push(NodeItem::from_path(path));
        pb.inc(1);
    }
    pb.finish_and_clear();
    node_items.sort_by_key(|item| std::cmp::Reverse(item.size));

    println!("Total elements:\n{}\n", node_items.len());

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
                println!("{selected_item}");
                fs::remove_dir_all(&selected_item.path)?;
            }
        }
        Ok(None) => println!("User exited using Esc or q"),
        Err(dialoguer_error) => {
            eprintln!("Dialoguer error: {dialoguer_error}");
            return Err(std::io::Error::other(dialoguer_error.to_string()));
        }
    }

    Ok(())
}
