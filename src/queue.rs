use std::collections::HashMap;

use reqwest::blocking as request;
use serde::Deserialize;

use crate::{libs, utils};

#[derive(Deserialize)]
struct Response {
    records: Vec<Record>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Record {
    id: u32,
    size: f64,
    timeleft: Option<String>,
    status: String,
    errorMessage: Option<String>,
    pub movie: Option<NestedRecord>,
    pub series: Option<NestedRecord>,
    pub album: Option<NestedRecord>,
    pub book: Option<NestedRecord>,
}

#[derive(Deserialize, Debug)]
pub struct NestedRecord {
    pub title: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Download {
    pub id: u32,
    pub name: String,
    pub size: u64,
    pub status: String,
    pub eta: u64,
}

// Delete Download from Starr.
pub fn delete(url: &String) {
    match request::Client::new().delete(url).send() {
        Ok(_) => (),
        Err(error) => {
            utils::log::alert(
                "WARN",
                "Failed to remove download, will attempt again next run.",
                "The API has refused this request.",
                Some(error.to_string()),
            );
        }
    }
}

// Obtains Downloads from Starr.
pub fn get(platform: &str, url: &str) -> Vec<Download> {
    let res: Response = match request::get(url) {
        Ok(res) => match res.json() {
            Ok(res) => res,
            Err(error) => {
                utils::log::alert(
                    "WARN",
                    "Unable to process queue, will attempt again next run.",
                    "The API has responded with an invalid response.",
                    Some(error.to_string()),
                );
                Response { records: vec![] }
            }
        },
        Err(error) => {
            utils::log::alert(
                "WARN",
                "Unable to process queue, will attempt again next run.",
                "The connection to the API was unsuccessful.",
                Some(error.to_string()),
            );
            Response { records: vec![] }
        }
    };

    let mut downloads: Vec<Download> = vec![];

    res.records.iter().for_each(|record| {
        let eta = {
            let timeleft = record.timeleft.clone().unwrap_or_else(|| "0".to_string());
            utils::parse::string_hms_to_ms(&timeleft)
        };

        // Determine status of download.
        // - Please inform me; if you have a different method
        //   on how to identify a download that is fetching metadata.
        let status = if let Some(error_message) = &record.errorMessage {
            if error_message.to_ascii_lowercase().contains("metadata") {
                "metadata".to_string()
            } else {
                record.status.clone()
            }
        } else {
            record.status.clone()
        };

        downloads.push(Download {
            id: record.id,
            name: utils::parse::recordname(&platform, &record),
            size: record.size as u64,
            status,
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
    let mut table_contents: Vec<libs::table::TableContent> = vec![];

    for download in queue_items {
        let id = download.id.clone();
        let mut state = String::from("Normal");

        // Add non-existing download to strikelist.
        let mut strikes: u32 = match strikelist.get(&id) {
            Some(strikes) => strikes.clone(),
            None => {
                strikelist.insert(id, 0);
                0
            }
        };

        // -- Bypass Section: Rules that define if a download is eligible to be striked.

        let mut bypass: bool = false;

        if download.size
            >= utils::parse::string_bytesize_to_bytes(&env.ignore_above_size)
                .unwrap()
                .as_u64()
        {
            state = String::from("Ignored");
            bypass = true;
        }

        if download.status == "queued" {
            state = String::from("Queued");
            bypass = true;
        }

        // -- Strike Section: Rules that define when to strike a download.

        if !bypass {
            let max_download_time_ms =
                utils::parse::string_time_notation_to_ms(&env.max_download_time).unwrap() as u64;

            if download.status == "metadata" || download.eta >= max_download_time_ms || (download.eta == 0 && download.status != "queued") {
                if strikes < env.max_strikes {
                    strikes += 1;
                    strikelist.insert(id, strikes);
                }
                state = String::from("Striked");
            }

            if strikes >= env.max_strikes {
                delete(&format!(
                    "{}queue/{}?apikey={}&blocklist={}&removeFromClient={}",
                    baseapi, id, env.apikey, true, env.remove_from_client
                ));
                state = String::from("Removed");
            }
        }

        // -- Logging Section

        table_contents.push(libs::table::TableContent {
            strikes: format!("{}/{}", strikes, env.max_strikes),
            name: download.name.chars().take(32).collect::<String>(),
            eta: utils::parse::ms_to_eta_string(&download.eta),
            size: format!("{:.2} GB", (download.size as f64 / 1000000000.0)).to_string(),
            state,
        })
    }

    libs::table::render(&table_contents);
}
