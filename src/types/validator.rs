use prettytable::{color, Attr, Cell, Row};
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

impl Validator {
    pub fn as_row(&self, row_number: usize, is_special: bool) -> Row {
        let mut row_vec: Vec<Cell> = if !self.is_jailed {
            vec![
                Cell::new(&row_number.to_string()),
                Cell::new(&self.validator),
                Cell::new(&self.name),
                Cell::new(&self.n_recent_blocks.to_string()),
                Cell::new(&self.stake.to_string()),
                Cell::new(&self.is_jailed.to_string()),
            ]
        } else {
            vec![
                Cell::new(&row_number.to_string()).with_style(Attr::ForegroundColor(color::RED)),
                Cell::new(&self.validator).with_style(Attr::ForegroundColor(color::RED)),
                Cell::new(&self.name).with_style(Attr::ForegroundColor(color::RED)),
                Cell::new(&self.n_recent_blocks.to_string())
                    .with_style(Attr::ForegroundColor(color::RED)),
                Cell::new(&self.stake.to_string()).with_style(Attr::ForegroundColor(color::RED)),
                Cell::new(&self.is_jailed.to_string())
                    .with_style(Attr::ForegroundColor(color::RED)),
            ]
        };

        if is_special {
            let mut new_row_vec = vec![];

            for row in row_vec.into_iter() {
                new_row_vec.push(
                    row.with_style(Attr::Italic(true))
                        .with_style(Attr::ForegroundColor(color::BRIGHT_GREEN)),
                );
            }

            row_vec = new_row_vec;
        }

        Row::new(row_vec)
    }
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
