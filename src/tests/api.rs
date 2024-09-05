use reqwest::blocking::get;

use crate::utils;

pub fn test(platform: &str, baseapi: &str, apikey: &str) {
    // Check if the API can be reached.
    match get(&format!("{baseapi}health?apikey={apikey}")) {
        // Can be reached, continue.
        Ok(res) => {
            // Let's just assume that the APIKEY is
            // invalid if the code returned is not "200".
            if res.status() != 200 {
                utils::logger::alert(
                    "FATAL",
                    "The provided \"APIKEY\" is not valid.",
                    format!("Obtain the {platform} API key in Settings > General > API Key")
                        .as_str(),
                    None,
                );
                utils::system::exit(1);
            }
        }
        // Could not be reached.
        Err(error) => {
            utils::logger::alert(
                "FATAL",
                format!("A connection to the {platform} API could not be established.").as_str(),
                "Ensure that the API is accessible and try again.",
                Some(error.to_string()),
            );
            utils::system::exit(1);
        }
    }
}
