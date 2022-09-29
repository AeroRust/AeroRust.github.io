use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use scraper::{Category, CrateData};

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
