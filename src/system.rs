use std::{env, process::exit};

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
fn default(which: &str, default: &str) -> String {
    println!("WARN: \"{}\" NOT SET - USING \"{}\"", which, default);
    default.to_string()
}

// -- Returns environment variables from the host.
pub fn env() -> Envs {
    Envs {
        // -- Unrecoverable.
        apikey: env::var("APIKEY").unwrap_or_else(|_| {
            println!("FATAL: \"APIKEY\" NOT SET");
            exit(1);
        }),

        // -- Partially recoverable.
        strike_threshold: env::var("STRIKE_THRESHOLD")
            .unwrap_or_else(|_| default("STRIKE_THRESHOLD", "3"))
            .parse::<u32>()
            .unwrap_or_else(|_| {
                println!("FATAL: \"STRIKE_THRESHOLD\" IS INVALID");
                exit(1);
            }),
        aggresive_strikes: env::var("AGGRESSIVE_STRIKES")
            .unwrap_or_else(|_| default("AGGRESSIVE_STRIKES", "false"))
            .parse::<bool>()
            .unwrap_or_else(|_| {
                println!("FATAL: \"AGGRESSIVE_STRIKES\" IS INVALID");
                exit(1);
            }),

        // -- Recoverable via defaults.
        baseurl: env::var("BASEURL")
            .unwrap_or_else(|_| default("BASEURL", "http://127.0.0.1:7878")),
        platform: env::var("PLATFORM").unwrap_or_else(|_| default("PLATFORM", "radarr")),
        time_threshold: env::var("TIME_THRESHOLD")
            .unwrap_or_else(|_| default("TIME_THRESHOLD", "2h")),
        size_threshold: env::var("SIZE_THRESHOLD")
            .unwrap_or_else(|_| default("SIZE_THRESHOLD", "25 GB")),
        check_interval: env::var("CHECK_INTERVAL")
            .unwrap_or_else(|_| default("CHECK_INTERVAL", "10m")),
    }
}
