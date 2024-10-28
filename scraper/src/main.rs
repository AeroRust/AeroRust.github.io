use std::{fs, io::ErrorKind, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use log::{error, info, LevelFilter};
use walkdir::{DirEntry, WalkDir};

use scraper::{data::CategoryData, Config, Scraper};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Run the scraper that extracts all the categories and crates from crates.io
/// database dump and matches any custom categories (missing in crates.io) to
/// crates found on crates.io.
pub struct Cli {}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Debug)
        .init();

    let _cli = Cli::parse();

    // todo: make CLI configurable?
    let config_path = env!("CARGO_MANIFEST_DIR")
        .parse::<PathBuf>()?
        .join("config.toml");
    let mut config: Config = toml::from_str(&fs::read_to_string(&config_path)?)?;

    // fill the Category with all the whitelisted crates
    for krate in config.crates.iter() {
        for krate_category in krate.categories.iter() {
            let found_category = config
                .categories
                .iter_mut()
                .find(|category| &category.name == krate_category);

            match found_category {
                Some(category) => {
                    category.filtered.whitelist.insert(krate.crate_data.name.clone());
                    info!("Crate '{}' added to the '{}' category", &krate.crate_data.name, &category.name);
                },
                None => anyhow::bail!("The '{}' crate category '{}' is not configured! Please check or update existing categories.", &krate.crate_data.name, &krate_category)
            }
        }
    }
    info!("Using config at {}", config_path.display());

    let scraper = Scraper::new();

    let categories_data = {
        let loaded_dump = scraper.from_dump(&config.categories)?;

        loaded_dump.to_data()
    };

    let root_dir = env!("CARGO_MANIFEST_DIR").parse::<PathBuf>()?.join("../");

    remove_categories_dirs(&root_dir)?;

    write_to_files(&root_dir, categories_data)?;

    Ok(())
}

/// Removes directories and files for the categories
pub fn remove_categories_dirs(root_dir: &PathBuf) -> anyhow::Result<()> {
    let categories_dir = root_dir.join("content/catalogue/");

    let walkdir = WalkDir::new(&categories_dir)
        // only take the first level of directories
        .max_depth(1);

    let _deleted_dirs = walkdir
        .into_iter()
        .filter_map(|e| match e {
            Ok(dir_entry) => {
                if is_hidden(&dir_entry) {
                    info!("Skipping hidden entry: {}", dir_entry.path().display());

                    return None;
                } else if &categories_dir == dir_entry.path() {
                    info!(
                        "Skipping catalogue's directory: {}",
                        dir_entry.path().display()
                    );

                    return None;
                }

                match dir_entry.metadata() {
                    Ok(meta) if meta.is_dir() => {
                        let remove_result = remove_dir_all::remove_dir_all(dir_entry.path());

                        match remove_result {
                            Ok(_) => {
                                info!(
                                    "Removed directory's content: {}",
                                    dir_entry.path().display()
                                );
                                Some(dir_entry.path().to_path_buf())
                            }
                            Err(err) => {
                                error!(
                                    "Removing directory's content: {}; {err}",
                                    dir_entry.path().display()
                                );
                                None
                            }
                        }
                    }
                    Err(meta_err) => {
                        error!(
                            "Failed to get metadata for path: {}; {meta_err}",
                            dir_entry.path().display()
                        );
                        None
                    }
                    Ok(_meta) => {
                        info!(
                            "Skipping non-directory entry: {}",
                            dir_entry.path().display()
                        );

                        None
                    }
                }
            }
            Err(err) => {
                error!("Error getting entry: {err}");
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(())
}

pub fn write_to_files(root_dir: &PathBuf, categories: Vec<CategoryData>) -> anyhow::Result<()> {
    for category_data in categories {
        let markdown_content = format!(
            r#"+++
title = "{name}"
description = '''
{description}
'''
template = "catalogue/category.html"
paginate_by = 30
sort_by = "title"
[extra]
icon = "{icon}"
+++"#,
            name = category_data.category.name,
            description = category_data.category.description,
            icon = category_data.category.icon,
        );

        let category_dir = root_dir.join(&format!("content/catalogue/{}", category_data.slug));

        // create category dir (section)
        match fs::create_dir(&category_dir) {
            Ok(_) => {}
            Err(err) if err.kind() == ErrorKind::AlreadyExists => {
                info!(
                    "'{}' category dir (with slug '{}') already exist. Skipping creation...",
                    category_data.category.name, category_data.slug
                )
            }
            Err(err) => panic!(
                "Error creating the '{}' category dir ('{}): {}",
                category_data.category.name,
                category_dir.display().to_string(),
                err
            ),
        }

        // create the section `_index.md`
        fs::write(category_dir.join("_index.md"), markdown_content)
            .context("Catalogue markdown file failed to be created")?;

        for krate in category_data.crates {
            let crate_toml = toml::to_string_pretty(&krate)?;
            let crate_markdown = format!(
                r#"+++
title = "{name}"
description = '''
{description}
'''
template = "catalogue/crate.html"
slug = "{name}"
[extra]
{crate_toml}
+++"#,
                name = krate.name,
                description = krate.description,
            );

            fs::write(
                category_dir.join(&format!("{}.md", krate.name)),
                crate_markdown,
            )
            .context("Category json with all the crates failed to be created")?;
        }
    }

    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
