use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    #[serde(rename = "type")]
    pub t: String,
}

