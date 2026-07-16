use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub name: String,
    pub ext: String,
    pub size_bytes: u64,
    pub creation: DateTime<Utc>
}
