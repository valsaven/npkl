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

    let mut path: Option<PathBuf> = None;
    let mut sort_by_size = true;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--sort" || arg.starts_with("--sort=") {
            let value = arg
                .strip_prefix("--sort=")
                .map(str::to_owned)
                .or_else(|| args.next())
                .unwrap_or_default();
            sort_by_size = match value.as_str() {
                "size" => true,
                "path" => false,
                _ => {
                    eprintln!("ERROR: --sort expects 'size' or 'path'");
                    std::process::exit(2);
                }
            };
        } else if arg.starts_with('-') || path.is_some() {
            eprintln!("Usage: npkl [path] [--sort size|path]");
            std::process::exit(2);
        } else {
            path = Some(PathBuf::from(arg));
        }
    }
    let path = path.unwrap_or_else(|| PathBuf::from("."));
    if !path.is_dir() {
        eprintln!("ERROR: '{}' is not a directory", path.display());
        std::process::exit(2);
    }

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

    println!("Total elements:\n{}\n", node_items.len());

    if sort_by_size {
        node_items.sort_by_key(|item| std::cmp::Reverse(item.size));
    } else {
        node_items.sort_by_key(|item| item.path.clone());
    }

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
