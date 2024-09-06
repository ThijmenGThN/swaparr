use std::collections::HashMap;

use reqwest::blocking as request;
use serde::Deserialize;

use crate::{libs, utils};

#[derive(Deserialize)]
struct Response {
    records: Vec<Record>,
}

#[derive(Deserialize)]
pub struct Record {
    id: u32,
    size: f64,
    timeleft: Option<String>,
    pub movie: Option<NestedRecord>,
    pub series: Option<NestedRecord>,
    pub album: Option<NestedRecord>,
    pub book: Option<NestedRecord>,
}

#[derive(Deserialize)]
pub struct NestedRecord {
    pub title: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Download {
    pub id: u32,
    pub name: String,
    pub size: u64,
    pub eta: u64,
}

// Delete Download from Starr.
pub fn delete(url: &String) {
    // Send the request to delete to the API.
    match request::Client::new().delete(url).send() {
        // Should be deleted.
        Ok(_) => (),
        // Attempt to delete did not go through. (This should be attempted again next run)
        Err(error) => {
            utils::logger::alert(
                "WARN",
                "Failed to remove download, will attempt again next run.",
                "The API has refused this request.",
                Some(error.to_string()),
            );
        }
    }
}

// Obtains Sownloads from Starr.
pub fn get(platform: &str, url: &str) -> Vec<Download> {
    // Request active downloads in queue from the Starr API.
    let res: Response = match request::get(url) {
        // API can be reached.
        Ok(res) => match res.json() {
            // Response is valid.
            Ok(res) => res,
            // Did not respond with valid JSON.
            Err(error) => {
                utils::logger::alert(
                    "WARN",
                    "Unable to process queue, will attempt again next run.",
                    "The API has responded with an invalid response.",
                    Some(error.to_string()),
                );
                // Something went wrong, return an empty queue as fallback.
                Response { records: vec![] }
            }
        },
        Err(error) => {
            utils::logger::alert(
                "WARN",
                "Unable to process queue, will attempt again next run.",
                "The connection to the API was unsuccessful.",
                Some(error.to_string()),
            );
            // Something went wrong, return an empty queue as fallback.
            Response { records: vec![] }
        }
    };

    let mut downloads: Vec<Download> = vec![];

    // Iterate over all downloads.
    res.records.iter().for_each(|record| {
        // Convert HMS from record to eta in milliseconds.
        let eta = {
            let timeleft = record.timeleft.clone().unwrap_or_else(|| "0".to_string());
            utils::parse::string_hms_to_ms(&timeleft)
        };

        // Add download to the list.
        downloads.push(Download {
            id: record.id,
            name: utils::parse::recordname(&platform, &record),
            size: record.size as u64,
            eta,
        });
    });

    downloads
}

// Determines if the download is eligible to be striked.
pub fn process(
    env: &utils::system::Envs,
    baseapi: &String,
    queue_items: Vec<Download>,
    strikelist: &mut HashMap<u32, u32>,
) {
    // Table rows that will be pretty-printed to the terminal.
    let mut table_contents: Vec<libs::table::TableContent> = vec![];

    // Loop over all active downloads from the queue.
    for download in queue_items {
        let id = download.id.clone();
        let mut status = String::from("Normal");

        // Add download id to strikes with default "0" if it does not exist yet.
        let mut strikes: u32 = match strikelist.get(&id) {
            Some(strikes) => strikes.clone(),
            None => {
                strikelist.insert(id, 0);
                0
            }
        };

        // -- Bypass Rules -- Rules that define if a download is eligible to be striked.

        let mut bypass: bool = false;

        // Download is larger than set threshold. (Safe to unwrap, gets validated in health-check.)
        let ignore_above_size_bytes =
            utils::parse::string_bytesize_to_bytes(&env.ignore_above_size)
                .unwrap()
                .as_u64();
        if download.size >= ignore_above_size_bytes {
            status = String::from("Ignored");
            bypass = true;
        }

        // -- Strike rules -- Rules that define when to strike a download.

        if !bypass {
            // Extract timestamp from time notation. (Safe to unwrap, gets validated in health-check.)
            let max_download_time_ms =
                utils::parse::string_time_notation_to_ms(&env.max_download_time).unwrap() as u64;

            // Download will take longer than set threshold.
            if download.eta >= max_download_time_ms {
                // Increment strikes if it's below set maximum.
                if strikes < env.max_strikes {
                    strikes += 1;
                    strikelist.insert(id, strikes);
                }
                status = String::from("Striked");
            }

            // Download meets set amount of strikes, a request to delete will be sent.
            if strikes >= env.max_strikes {
                delete(&format!(
                    "{}queue/{}?apikey={}&blocklist={}&removeFromClient={}",
                    baseapi, id, env.apikey, true, env.remove_from_client
                ));
                status = String::from("Removed");
            }
        }

        // -- Logging --

        // Add download to pretty-print table.
        table_contents.push(libs::table::TableContent {
            strikes: format!("{}/{}", strikes, env.max_strikes),
            status,
            name: download.name.chars().take(32).collect::<String>(),
            eta: utils::parse::ms_to_eta_string(&download.eta),
            size: format!("{:.2} GB", (download.size as f64 / 1000000000.0)).to_string(),
        })
    }

    // Print table to terminal.
    libs::table::render(&table_contents);
}
