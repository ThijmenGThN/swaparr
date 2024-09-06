use reqwest::blocking::get;

use crate::utils;

pub fn test(platform: &str, baseapi: &str, apikey: &str) {
    match get(&format!("{baseapi}health?apikey={apikey}")) {
        Ok(res) => {
            if res.status() != 200 {
                utils::log::alert(
                    "FATAL",
                    "The provided \"APIKEY\" is not valid.",
                    format!("Obtain the {platform} API key in Settings > General > API Key")
                        .as_str(),
                    None,
                );
                utils::system::exit(1);
            }
        }
        Err(error) => {
            utils::log::alert(
                "FATAL",
                format!("A connection to the {platform} API could not be established.").as_str(),
                "Ensure that the API is accessible and try again.",
                Some(error.to_string()),
            );
            utils::system::exit(1);
        }
    }
}
