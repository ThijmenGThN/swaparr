use reqwest::blocking::get;

use crate::{logger, system};

pub fn check(env: &system::Envs) {
    // Check if the API can be reached.
    match get(&format!(
        "{}/api/v3/health?apikey={}",
        env.baseurl, env.apikey
    )) {
        // Can be reached, continue.
        Ok(res) => {
            // Let's just assume that the APIKEY is
            // invalid if the code returned is not "200".
            if res.status() != 200 {
                logger::alert(
                    "FATAL",
                    format!("The provided \"APIKEY\" is not valid."),
                    format!(
                        "Obtain the {} API key in Settings > General > API Key",
                        env.platform
                    ),
                    None,
                );
                system::exit(1);
            }
        }
        // Could not be reached.
        Err(error) => {
            logger::alert(
                "FATAL",
                format!(
                    "A connection to the {} API could not be established.",
                    env.platform
                ),
                "Ensure that the API is accessible and try again.".to_string(),
                Some(error.to_string()),
            );
            system::exit(1);
        }
    }
}
