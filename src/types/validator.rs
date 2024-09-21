use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Validator {
    pub validator: String,
    pub name: String,
    pub description: String,
    pub n_recent_blocks: usize,
    pub stake: u64,
    pub is_jailed: bool,
}

impl std::fmt::Display for Validator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}, Address: {}, Stake: {}, Recent Blocks: {}, Jailed: {}",
            self.validator, self.name, self.stake, self.n_recent_blocks, self.is_jailed
        )
    }
}
