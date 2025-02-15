use std::{env, process, thread::sleep, time::Duration};

use crate::utils;

#[derive(Debug)]
pub struct Envs {
    pub baseurl: String,
    pub apikey: String,
    pub platform: String,
    pub max_strikes: u32,
    pub scan_interval: String,
    pub max_download_time: String,
    pub ignore_above_size: String,
    pub remove_from_client: String,
    pub dry_run: String,
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
    let envs = Envs {
        // ----- Unrecoverable -----
        apikey: env::var("APIKEY").unwrap_or_else(|_| {
            utils::log::alert(
                "FATAL",
                "ENV: \"APIKEY\" is undefined and required.",
                "There is no default value for this field.",
                None,
            );
            exit(1);
        }),

        // ----- Recoverable via defaults -----
        max_strikes: env::var("MAX_STRIKES")
            // Allow falling back onto STRIKE_THRESHOLD for backwards compatibility.
            .or_else(|_| env::var("STRIKE_THRESHOLD"))
            .unwrap_or_else(|_| default("MAX_STRIKES", "3", false))
            // Convert to u32, if it fails, use default u32.
            .parse::<u32>()
            .unwrap_or_else(|_| {
                default("MAX_STRIKES", "3", true);
                3 // default
            }),

        baseurl: env::var("BASEURL")
            .unwrap_or_else(|_| default("BASEURL", "http://127.0.0.1:7878", false)),

        platform: env::var("PLATFORM").unwrap_or_else(|_| default("PLATFORM", "radarr", false)),

        max_download_time: env::var("MAX_DOWNLOAD_TIME")
            // Allow falling back onto TIME_THRESHOLD for backwards compatibility.
            .or_else(|_| env::var("TIME_THRESHOLD"))
            .unwrap_or_else(|_| default("MAX_DOWNLOAD_TIME", "2h", false)),

        ignore_above_size: env::var("IGNORE_ABOVE_SIZE")
            // Allow falling back onto SIZE_THRESHOLD for backwards compatibility.
            .or_else(|_| env::var("SIZE_THRESHOLD"))
            .unwrap_or_else(|_| default("IGNORE_ABOVE_SIZE", "25 GB", false)),

        scan_interval: env::var("SCAN_INTERVAL")
            // Allow falling back onto CHECK_INTERVAL for backwards compatibility.
            .or_else(|_| env::var("CHECK_INTERVAL"))
            .unwrap_or_else(|_| default("SCAN_INTERVAL", "10m", false)),

        remove_from_client: match utils::parse::string_to_bool(
            env::var("REMOVE_FROM_CLIENT")
                .unwrap_or_else(|_| default("REMOVE_FROM_CLIENT", "true", false)),
        ) {
            Ok(value) => value.to_string(),
            Err(_) => default("REMOVE_FROM_CLIENT", "true", true).to_string(),
        },

        dry_run: match utils::parse::string_to_bool(
            env::var("DRY_RUN")
                .unwrap_or_else(|_| default("DRY_RUN", "false", false)),
        ) {
            Ok(value) => value.to_string(),
            Err(_) => default("DRY_RUN", "false", true).to_string(),
        },
    };

    // Check if variable MAX_DOWNLOAD_TIME is able to be parsed.
    match utils::parse::string_time_notation_to_ms(&envs.max_download_time) {
        Ok(_) => (),
        Err(_) => {
            utils::log::alert(
                "FATAL",
                "Environment variable \"MAX_DOWNLOAD_TIME\" is not valid.",
                "Must be a time-notation: \"1d\", \"6h\", \"30m\", etc.. by default: \"2h\"",
                None,
            );
            utils::system::exit(1);
        }
    }

    // Check if variable IGNORE_ABOVE_SIZE is able to be parsed.
    match utils::parse::string_bytesize_to_bytes(&envs.ignore_above_size) {
        Ok(_) => (),
        Err(_) => {
            utils::log::alert(
                "FATAL",
                "Environment variable \"IGNORE_ABOVE_SIZE\" is not valid.",
                "Must be a bytesize-notation: \"1TB\", \"1GB\", \"1MB\", etc.. by default: \"25GB\"",
                None,
            );
            utils::system::exit(1);
        }
    }

    // Check if variable SCAN_INTERVAL is able to be parsed.
    match utils::parse::string_time_notation_to_ms(&envs.scan_interval) {
        Ok(_) => (),
        Err(_) => {
            utils::log::alert(
                "FATAL",
                "Environment variable \"SCAN_INTERVAL\" is not valid.",
                "Must be a time-notation: \"1d\", \"6h\", \"30m\", etc.. by default: \"10m\"",
                None,
            );
            utils::system::exit(1);
        }
    }

    return envs;
}
