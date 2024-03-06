use reqwest::blocking as request;
use serde::Deserialize;

use crate::utils;

#[derive(Debug, Deserialize, Clone)]
pub struct Torrent {
    pub id: u32,
    pub name: String,
    pub size: u64,
    pub eta: u64,
}

#[derive(Deserialize)]
struct RecordMovie {
    title: String,
}

#[derive(Deserialize)]
struct Record {
    id: u32,
    size: u64,
    movie: RecordMovie,
    timeleft: Option<String>,
}

#[derive(Deserialize)]
struct Response {
    records: Vec<Record>,
}

pub fn get(url: &String) -> Vec<Torrent> {
    // Request active torrents in queue from the Radarr or Sonarr API.
    let res: Response = request::get(url).unwrap().json().unwrap();

    let mut torrents: Vec<Torrent> = vec![];

    // Iterate over all torrents.
    res.records.iter().for_each(|record| {

        // Obtain HMS from timeleft attribute.
        let timeleft = record.timeleft.clone().unwrap_or_else(|| "0".to_string());

        // Convert timeleft from HMS to milliseconds.
        let timeleft_ms = utils::format_to_ms(timeleft).unwrap_or_else(|| 0);

        torrents.push(Torrent {
            id: record.id,
            name: record.movie.title.clone(),
            size: record.size,
            eta: timeleft_ms,
        });
    });

    torrents
}

pub fn delete(url: &String) {
    request::Client::new().delete(url).send().unwrap();
}
