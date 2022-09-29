use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    time::Instant,
};

use db_dump::{
    categories::{self, CategoryId},
    crates, Loader,
};
use log::info;
use slug::slugify;

use crate::{
    config::Category,
    data::{CategoryData, CrateData, CrateId},
};

#[derive(Debug, Clone)]
pub struct Scraper {}

impl Scraper {
    pub fn new() -> Self {
        Self {}
    }

    /// Loads Categories, crates & crates categories from dump
    pub fn from_dump(&self, all_categories: &[Category]) -> anyhow::Result<LoadedData> {
        // if it doesn't have a canonical slug (from crates.io) then we include it in own_categories
        let (own_categories, crates_io_categories): (HashMap<_, _>, HashMap<_, _>) = all_categories
            .to_vec()
            .into_iter()
            .map(|category| (slugify(&category.name), category))
            .partition(|(_slug, category)| category.canonical_slug.is_none());

        let manifest = env!("CARGO_MANIFEST_DIR").parse::<PathBuf>()?;

        let dump_path = manifest.join("../.tmp/db-dump.tar.gz");

        let mut loaded = LoadedData::default();

        let instant = Instant::now();
        // load Crates.io categories first
        Loader::new()
            .categories(|row| {
                let find_category =
                    crates_io_categories
                        .iter()
                        .find(|(_slug, crate_io_category)| {
                            if let Some(slug) = &crate_io_category.canonical_slug {
                                slug == &row.slug
                            } else {
                                false
                            }
                        });

                if let Some((category_name, category)) = find_category {
                    assert!(
                        loaded
                            .categories
                            .insert(slugify(category_name), (category.clone(), row))
                            .is_none(),
                        "Should not have this category in the list twice!"
                    )
                }
            })
            .load(&dump_path)?;

        info!(
            "Loaded categories for: {} secs",
            instant.elapsed().as_secs()
        );

        let instant = Instant::now();
        // Then load the crates_categories filtering out what we don't need
        Loader::new()
            .crates_categories(|crate_category| {
                let find_category = loaded
                    .categories
                    .values()
                    .find(|(_, row)| row.id == crate_category.category_id);

                if find_category.is_some() {
                    assert!(
                        loaded
                            .crates_categories
                            .insert((crate_category.category_id, crate_category.crate_id.into())),
                        "Should not have this crate category in the list twice!"
                    );
                }
            })
            .load(&dump_path)?;
        info!(
            "Loaded crates categories for: {} secs",
            instant.elapsed().as_secs()
        );

        let instant = Instant::now();
        // Then load the crates themselves filtering out what we don't need
        Loader::new()
            .crates(|crate_row| {
                let in_crates_io_category = loaded
                    .crates_categories
                    .iter()
                    .any(|(_category_id, crate_id)| *crate_id == crate_row.id.into());

                let whitelisted = all_categories
                    .iter()
                    .any(|category| category.filtered.whitelist.contains(&crate_row.name));

                // if crate is a category we want or if it's a whitelisted crate
                // even if it's blacklisted for a given category it might be whitelisted in another
                if in_crates_io_category || whitelisted {
                    assert!(
                        loaded
                            .crates
                            .insert(crate_row.id.into(), crate_row)
                            .is_none(),
                        "Should not have this crate in the list twice!"
                    );
                }
            })
            .load(&dump_path)?;
        info!("Loaded crates for: {} secs", instant.elapsed().as_secs());

        loaded.own_categories = own_categories;

        Ok(loaded)
    }
}

#[derive(Debug, Default)]
pub struct LoadedData {
    /// Key - slug
    pub own_categories: HashMap<String, Category>,
    /// Key - slug
    pub categories: HashMap<String, (Category, categories::Row)>,
    pub crates: HashMap<CrateId, crates::Row>,
    pub crates_categories: HashSet<(CategoryId, CrateId)>,
}

impl LoadedData {
    /// collects crates.io categories & crates with the own crates into
    /// the end result of all categories data sorted by [`Category.name`]
    pub fn to_data(&self) -> Vec<CategoryData> {
        let crates_io_categories =
            self.categories
                .iter()
                .map(|(_slug, (category, category_row))| {
                    let crates = self
                        .crates
                        .iter()
                        .filter_map(|(crate_id, crate_row)| {
                            let in_this_category = self
                                .crates_categories
                                .contains(&(category_row.id, *crate_id));

                            let in_blacklist =
                                category.filtered.blacklist.contains(&crate_row.name);
                            let in_whitelist =
                                category.filtered.whitelist.contains(&crate_row.name);

                            match (in_this_category, in_whitelist, in_blacklist) {
                                // both whitelisted and blacklisted
                                (_, true, true) => panic!(
                                "Crate can't be both whitelisted and blacklisted for a category!"
                            ),
                                // blacklisted
                                (_, _, true) => None,
                                // doesn't have the category
                                // nor it's whitelisted by the category
                                (false, false, _) => None,
                                _ => Some(CrateData::from(crate_row)),
                            }
                        })
                        .collect();

                    CategoryData {
                        category: category.clone(),
                        slug: slugify(&category.name),
                        crates,
                    }
                });

        let own_categories = self
            .own_categories
            .clone()
            .into_iter()
            .map(|(slug, category)| {
                let crates = self
                    .crates
                    .iter()
                    .filter_map(|(_crate_id, crate_row)| {
                        let in_blacklist = category.filtered.blacklist.contains(&crate_row.name);
                        let in_whitelist = category.filtered.whitelist.contains(&crate_row.name);

                        match (in_whitelist, in_blacklist) {
                            // both whitelisted and blacklisted
                            (true, true) => panic!(
                                "Crate can't be both whitelisted and blacklisted for a category!"
                            ),
                            // blacklisted
                            (_, true) => None,
                            // it's whitelisted for own categories
                            (false, _) => None,
                            _ => Some(CrateData::from(crate_row)),
                        }
                    })
                    .collect();

                CategoryData {
                    category: category.clone(),
                    slug,
                    crates,
                }
            });

        let mut all_categories: Vec<_> = crates_io_categories.chain(own_categories).collect();
        all_categories.sort_unstable_by(|a, b| a.category.name.cmp(&b.category.name));

        all_categories
    }
}
