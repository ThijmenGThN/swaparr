use reqwest::blocking as request;
use serde::Deserialize;

use crate::logger;
use crate::parser;

// -- Gets Torrents from Radarr or Sonarr.
pub fn get(url: &String) -> Vec<Torrent> {
    // Request active torrents in queue from the Radarr or Sonarr API.
    let res: Response = match request::get(url) {
        Ok(res) => match res.json() {
            Ok(res) => res,
            Err(error) => {
                logger::alert(
                    "WARN",
                    "Unable to process queue, will attempt again next run.".to_string(),
                    "The API has responded with an invalid response.".to_string(),
                    Some(error.to_string())
                );

                Response { records: vec![] }
            }
        },
        Err(error) => {
            logger::alert(
                "WARN",
                "Unable to process queue, will attempt again next run.".to_string(),
                "The connection to the API was unsuccessful.".to_string(),
                Some(error.to_string())
            );
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
        Err(error) => {
            logger::alert(
                "WARN",
                "Failed to remove torrent, will attempt again next run.".to_string(),
                "The API has refused this request.".to_string(),
                Some(error.to_string()),
            );
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
