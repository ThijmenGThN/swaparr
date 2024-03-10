use std::{env, process, thread::sleep, time::Duration};

use crate::logger;

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

// -- Use defaults if unset.
fn default(which: &str, default: &str, invalid: bool) -> String {
    println!(
        " ─ ENV: \"{}\" is {}, using default: \"{}\".",
        which,
        if invalid { "invalid" } else { "undefined" },
        default
    );
    default.to_string()
}

// -- Delayed process exit.
pub fn exit(code: i32) -> ! {
    sleep(Duration::from_secs(2));
    process::exit(code)
}

// -- Returns environment variables from the host.
pub fn env() -> Envs {
    Envs {
        // ----- Unrecoverable -----
        apikey: env::var("APIKEY").unwrap_or_else(|_| {
            logger::alert(
                "FATAL",
                "ENV: \"APIKEY\" is undefined and required.".to_string(),
                "There is no default value for this field.".to_string(),
                true,
            );
            // Delayed exit for UX on Windows.
            sleep(Duration::from_secs(2));
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
    }
}
