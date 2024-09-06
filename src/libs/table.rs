use tabled::{settings::Style, Table, Tabled};

use crate::utils;

#[derive(Tabled)]
pub struct TableContent {
    pub strikes: String,
    pub state: String,
    pub name: String,
    pub size: String,
    pub eta: String,
}

// Either prints a table containing Download or a "No downloads found" banner.
pub fn render(contents: &Vec<TableContent>) {
    if contents.len() > 0 {
        let mut table = Table::new(contents);
        table.with(Style::rounded());
        println!("{}", table.to_string())
    } else {
        utils::log::empty();
    }
}
