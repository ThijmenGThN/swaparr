use reqwest::blocking as request;
use serde::Deserialize;

use crate::logger;
use crate::parser;

// -- Gets Torrents from Radarr or Sonarr.
pub fn get(url: &String, platform: &String) -> Vec<Torrent> {
    // Request active torrents in queue from the Radarr or Sonarr API.
    let res: Response = match request::get(url) {
        Ok(res) => match res.json() {
            Ok(res) => res,
            Err(error) => {
                logger::alert(
                    "WARN",
                    "Unable to process queue, will attempt again next run.".to_string(),
                    "The API has responded with an invalid response.".to_string(),
                    Some(error.to_string()),
                );

                Response { records: vec![] }
            }
        },
        Err(error) => {
            logger::alert(
                "WARN",
                "Unable to process queue, will attempt again next run.".to_string(),
                "The connection to the API was unsuccessful.".to_string(),
                Some(error.to_string()),
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

        // Extract name from API record.
        let name: String = match platform.as_str() {
            "radarr" => record.movie.as_ref().unwrap().title.clone(),
            "sonarr" => record.series.as_ref().unwrap().title.clone(),
            _ => String::from("Unknown"),
        };

        torrents.push(Torrent {
            id: record.id,
            name,
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

#[derive(Deserialize)]
struct Response {
    records: Vec<Record>,
}

#[derive(Deserialize)]
struct Record {
    id: u32,
    size: u64,
    movie: Option<NestedRecord>,
    series: Option<NestedRecord>,
    timeleft: Option<String>,
}

#[derive(Deserialize)]
struct NestedRecord {
    title: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Torrent {
    pub id: u32,
    pub name: String,
    pub size: u64,
    pub eta: u64,
}
