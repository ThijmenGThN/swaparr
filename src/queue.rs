use reqwest::blocking as request;
use serde::Deserialize;

use crate::parser;

// -- Gets Torrents from Radarr or Sonarr.
pub fn get(url: &String) -> Vec<Torrent> {
    // Request active torrents in queue from the Radarr or Sonarr API.
    let res: Response = match request::get(url) {
        Ok(res) => match res.json() {
            Ok(res) => res,
            Err(_) => {
                println!("WARN: Unable to process queue, will attempt again in next run.");
                Response { records: vec![] }
            }
        },
        Err(_) => {
            println!("WARN: Failed to get queue, will attempt again in next run.");
            Response { records: vec![] }
        }
    };

    let mut torrents: Vec<Torrent> = vec![];

    // Iterate over all torrents.
    res.records.iter().for_each(|record| {
        // Obtain HMS from timeleft attribute.
        let timeleft = record.timeleft.clone().unwrap_or_else(|| "0".to_string());

        // Convert timeleft from HMS to milliseconds.
        let timeleft_ms = parser::string_hms_to_ms(&timeleft);

        torrents.push(Torrent {
            id: record.id,
            name: record.movie.title.clone(),
            size: record.size,
            eta: timeleft_ms,
        });
    });

    torrents
}

// -- Deletes Torrent from Radarr or Sonarr.
pub fn delete(url: &String) {
    match request::Client::new().delete(url).send() {
        Ok(_) => (),
        Err(_) => {
            println!("WARN: Failed to remove torrent, will attempt again in next run.");
        }
    }
}

// ----- STRUCTS -----

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
