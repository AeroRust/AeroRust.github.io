use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::data::CrateData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub categories: Vec<Category>,
    /// The whitelist of Category crates
    pub crates: Vec<Crate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// TODO: implement crates overrides, like:
// - repository
// - wiki?!
pub struct Crate {
    #[serde(flatten)]
    pub crate_data: CrateData,
    pub categories: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    /// The Category name
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub icon: String,
    /// The crates.io canonical category slug
    /// if such category exist on crates.io
    pub canonical_slug: Option<String>,
    /// The crates that should be included
    /// or excluded in the category.
    #[serde(flatten)]
    pub filtered: FilteredCrates,
}

/// The list of whitelisted and blacklisted crates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilteredCrates {
    /// Included in the category, because they don't have the
    /// set category.
    /// This is populated after loading the categories from config.
    #[serde(default, skip_deserializing)]
    pub whitelist: HashSet<String>,
    /// Excluded from the category, because they are placed in
    /// the category wrongly.
    #[serde(default)]
    pub blacklist: HashSet<String>,
}
