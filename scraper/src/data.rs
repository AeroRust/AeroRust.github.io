use std::hash::Hash;

use db_dump::crates;
use serde::{Deserialize, Serialize};

use crate::config::Category;

#[derive(Debug, Clone)]
pub struct CategoryData {
    pub category: Category,
    /// UNIQUE
    /// and taken from [`categories::Row`].
    pub slug: String,
    pub crates: Vec<CrateData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Should be unique by name!
pub struct CrateData {
    pub name: String,
    #[serde(default)]
    pub downloads: u64,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub documentation: Option<String>,
    #[serde(default)]
    pub readme: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
}

impl Hash for CrateData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl PartialEq for CrateData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for CrateData {}

#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[serde(transparent)]
#[repr(transparent)]
pub struct CrateId(pub u32);

impl From<db_dump::crates::CrateId> for CrateId {
    fn from(crate_id: db_dump::crates::CrateId) -> Self {
        Self(crate_id.0)
    }
}

impl From<&crates::Row> for CrateData {
    fn from(row: &crates::Row) -> Self {
        Self {
            name: row.name.clone(),
            // todo:fix
            downloads: 0,
            description: row.description.clone(),
            homepage: row.homepage.clone(),
            documentation: row.documentation.clone(),
            readme: row.documentation.clone(),
            repository: row.repository.clone(),
        }
    }
}
