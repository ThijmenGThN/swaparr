use std::{env, process, thread::sleep, time::Duration};

use crate::{logger, parser, system};

#[derive(Debug)]
pub struct Envs {
    pub baseurl: String,
    pub apikey: String,
    pub platform: String,
    pub strike_threshold: u32,
    pub aggresive_strikes: bool,
    pub time_threshold: String,
    pub size_threshold: String,
    pub check_interval: String,
}

// Voids provided vars and returns a default value.
fn default(which: &str, default: &str, invalid: bool) -> String {
    println!(
        " ─ ENV: \"{}\" is {}, using default: \"{}\".",
        which,
        if invalid { "invalid" } else { "undefined" },
        default
    );
    default.to_string()
}

// Delayed process exit, looks better on Windows if running without Docker.
pub fn exit(code: i32) -> ! {
    sleep(Duration::from_secs(2));
    process::exit(code)
}

// Returns environment variables from the host.
pub fn env() -> Envs {
    // Extract environment variables.
    let envs = Envs {
        // ----- Unrecoverable -----
        apikey: env::var("APIKEY").unwrap_or_else(|_| {
            logger::alert(
                "FATAL",
                "ENV: \"APIKEY\" is undefined and required.".to_string(),
                "There is no default value for this field.".to_string(),
                None,
            );
            exit(1);
        }),

        // ----- Recoverable via defaults -----
        strike_threshold: env::var("STRIKE_THRESHOLD")
            .unwrap_or_else(|_| default("STRIKE_THRESHOLD", "3", false))
            .parse::<u32>()
            .unwrap_or_else(|_| {
                default("STRIKE_THRESHOLD", "3", true);
                3 // Needs non-String type as default.
            }),

        aggresive_strikes: env::var("AGGRESSIVE_STRIKES")
            .unwrap_or_else(|_| default("AGGRESSIVE_STRIKES", "false", false))
            .parse::<bool>()
            .unwrap_or_else(|_| {
                default("AGGRESSIVE_STRIKES", "false", true);
                false // Needs non-String type as default.
            }),

        baseurl: env::var("BASEURL")
            .unwrap_or_else(|_| default("BASEURL", "http://127.0.0.1:7878", false)),

        platform: env::var("PLATFORM").unwrap_or_else(|_| default("PLATFORM", "radarr", false)),

        time_threshold: env::var("TIME_THRESHOLD")
            .unwrap_or_else(|_| default("TIME_THRESHOLD", "2h", false)),

        size_threshold: env::var("SIZE_THRESHOLD")
            .unwrap_or_else(|_| default("SIZE_THRESHOLD", "25 GB", false)),

        check_interval: env::var("CHECK_INTERVAL")
            .unwrap_or_else(|_| default("CHECK_INTERVAL", "10m", false)),
    };

    // Check if variable TIME_THRESHOLD is able to be parsed.
    match parser::string_time_notation_to_ms(&envs.time_threshold) {
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
    match parser::string_bytesize_to_bytes(&envs.size_threshold) {
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
    match parser::string_time_notation_to_ms(&envs.check_interval) {
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

    return envs;
}
