use reqwest::blocking::get;

use crate::{logger, parser, system};

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
                    "The provided \"APIKEY\" is not valid.".to_string(),
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

    // Check if variable TIME_THRESHOLD is able to be parsed.
    match parser::string_time_notation_to_ms(&env.time_threshold) {
        // Variable can be parsed, thus valid.
        Ok(_) => (),
        // Variable could not be parsed, throw a fatal.
        Err(_) => {
            logger::alert(
                "FATAL",
                "Environment variable \"TIME_THRESHOLD\" is not valid.".to_string(),
                "Must be a time-notation: \"1d\", \"6h\", \"30m\", etc.. by default: \"2h\""
                    .to_string(),
                None,
            );
            system::exit(1);
        }
    }

    // Check if variable SIZE_THRESHOLD is able to be parsed.
    match parser::string_bytesize_to_bytes(&env.size_threshold) {
        // Variable can be parsed, thus valid.
        Ok(_) => (),
        // Variable could not be parsed, throw a fatal.
        Err(_) => {
            logger::alert(
                "FATAL",
                "Environment variable \"SIZE_THRESHOLD\" is not valid.".to_string(),
                "Must be a bytesize-notation: \"1TB\", \"1GB\", \"1MB\", etc.. by default: \"25GB\""
                    .to_string(),
                None,
            );
            system::exit(1);
        }
    }

    // Check if variable CHECK_INTERVAL is able to be parsed.
    match parser::string_time_notation_to_ms(&env.check_interval) {
        // Variable can be parsed, thus valid.
        Ok(_) => (),
        // Variable could not be parsed, throw a fatal.
        Err(_) => {
            logger::alert(
                "FATAL",
                "Environment variable \"CHECK_INTERVAL\" is not valid.".to_string(),
                "Must be a time-notation: \"1d\", \"6h\", \"30m\", etc.. by default: \"10m\""
                    .to_string(),
                None,
            );
            system::exit(1);
        }
    }
}
