use std::collections::HashMap;

use reqwest::blocking as request;
use serde::Deserialize;

use crate::{logger, parser, render, system};

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
pub struct Torrent {
    pub id: u32,
    pub name: String,
    pub size: u64,
    pub eta: u64,
}

// Delete Torrent from Starr.
pub fn delete(url: &String) {
    // Send the request to delete to the API.
    match request::Client::new().delete(url).send() {
        // Should be deleted.
        Ok(_) => (),
        // Attempt to delete did not go through. (This should be attempted again next run)
        Err(error) => {
            logger::alert(
                "WARN",
                "Failed to remove torrent, will attempt again next run.",
                "The API has refused this request.",
                Some(error.to_string()),
            );
        }
    }
}

// Obtains Torrents from Starr.
pub fn get(platform: &str, url: &str) -> Vec<Torrent> {
    // Request active torrents in queue from the Starr API.
    let res: Response = match request::get(url) {
        // API can be reached.
        Ok(res) => match res.json() {
            // Response is valid.
            Ok(res) => res,
            // Did not respond with valid JSON.
            Err(error) => {
                logger::alert(
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
            logger::alert(
                "WARN",
                "Unable to process queue, will attempt again next run.",
                "The connection to the API was unsuccessful.",
                Some(error.to_string()),
            );
            // Something went wrong, return an empty queue as fallback.
            Response { records: vec![] }
        }
    };

    let mut torrents: Vec<Torrent> = vec![];

    // Iterate over all torrents.
    res.records.iter().for_each(|record| {
        // Convert HMS from record to eta in milliseconds.
        let eta = {
            let timeleft = record.timeleft.clone().unwrap_or_else(|| "0".to_string());
            parser::string_hms_to_ms(&timeleft)
        };

        // Add torrent to the list.
        torrents.push(Torrent {
            id: record.id,
            name: parser::recordname(&platform, &record),
            size: record.size as u64,
            eta,
        });
    });

    torrents
}

// Determines if the torrent is eligible to be striked.
pub fn process(
    env: &system::Envs,
    baseapi: &String,
    queue_items: Vec<Torrent>,
    strikelist: &mut HashMap<u32, u32>,
) {
    // Table rows that will be pretty-printed to the terminal.
    let mut table_contents: Vec<render::TableContent> = vec![];

    // Loop over all active torrents from the queue.
    for torrent in queue_items {
        let id = torrent.id.clone();
        let mut status = String::from("Normal");

        // Add torrent id to strikes with default "0" if it does not exist yet.
        let mut strikes: u32 = match strikelist.get(&id) {
            Some(strikes) => strikes.clone(),
            None => {
                strikelist.insert(id, 0);
                0
            }
        };

        // -- Bypass Rules -- Rules that define if a torrent is eligible to be striked.

        let mut bypass: bool = false;

        // Torrent is being processed or the time is infinite.
        if torrent.eta == 0 && !env.aggresive_strikes {
            status = String::from("Pending");
            bypass = true;
        }

        // Torrent is larger than set threshold. (Safe to unwrap, gets validated in health-check.)
        let size_threshold_bytes = parser::string_bytesize_to_bytes(&env.size_threshold)
            .unwrap()
            .as_u64();
        if torrent.size >= size_threshold_bytes {
            status = String::from("Ignored");
            bypass = true;
        }

        // -- Strike rules -- Rules that define when to strike a torrent.

        if !bypass {
            // Extract timestamp from time notation. (Safe to unwrap, gets validated in health-check.)
            let time_threshold_ms =
                parser::string_time_notation_to_ms(&env.time_threshold).unwrap() as u64;

            // Torrent will take longer than set threshold.
            if (torrent.eta >= time_threshold_ms) || (torrent.eta == 0 && env.aggresive_strikes) {
                // Increment strikes if it's below set maximum.
                if strikes < env.strike_threshold {
                    strikes += 1;
                    strikelist.insert(id, strikes);
                }
                status = String::from("Striked");
            }

            // Torrent meets set amount of strikes, a request to delete will be sent.
            if strikes >= env.strike_threshold {
                delete(&format!(
                    "{}queue/{}?blocklist=true&apikey={}",
                    baseapi, id, env.apikey
                ));
                status = String::from("Removed");
            }
        }

        // -- Logging --

        // Add torrent to pretty-print table.
        table_contents.push(render::TableContent {
            strikes: format!("{}/{}", strikes, env.strike_threshold),
            status,
            name: torrent.name.chars().take(32).collect::<String>(),
            eta: parser::ms_to_eta_string(&torrent.eta),
            size: format!("{:.2} GB", (torrent.size as f64 / 1000000000.0)).to_string(),
        })
    }

    // Print table to terminal.
    render::table(&table_contents);
}
