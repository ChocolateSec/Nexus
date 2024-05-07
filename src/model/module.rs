use semver::Version;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Module {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub version: Version,
    pub url: String,
    pub checksum: String,
}
