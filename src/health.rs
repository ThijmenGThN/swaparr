use reqwest::blocking::get;

use crate::{logger, system};

pub fn api(platform: &str, baseapi: &str, apikey: &str) {
    // Check if the API can be reached.
    match get(&format!("{baseapi}health?apikey={apikey}")) {
        // Can be reached, continue.
        Ok(res) => {
            // Let's just assume that the APIKEY is
            // invalid if the code returned is not "200".
            if res.status() != 200 {
                logger::alert(
                    "FATAL",
                    "The provided \"APIKEY\" is not valid.".to_string(),
                    format!("Obtain the {platform} API key in Settings > General > API Key"),
                    None,
                );
                system::exit(1);
            }
        }
        // Could not be reached.
        Err(error) => {
            logger::alert(
                "FATAL",
                format!("A connection to the {platform} API could not be established."),
                "Ensure that the API is accessible and try again.".to_string(),
                Some(error.to_string()),
            );
            system::exit(1);
        }
    }
}
