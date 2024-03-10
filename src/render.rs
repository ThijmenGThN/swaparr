use tabled::{settings::Style, Table, Tabled};

use crate::logger;

#[derive(Tabled)]
pub struct TableContent {
    pub strikes: String,
    pub status: String,
    pub name: String,
    pub size: String,
    pub eta: String,
}

// -- Either prints a table containg Torrents or a "No torrents found" banner.
pub fn table(contents: &Vec<TableContent>) {
    if contents.len() > 0 {
        let mut table = Table::new(contents);
        table.with(Style::rounded());
        println!("{}", table.to_string())
    } else {
        logger::empty(false);
    }
}
