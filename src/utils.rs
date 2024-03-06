use std::{env, time::Duration};

use bytesize::ByteSize;
use humantime::format_duration;
use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
pub struct TableContent {
    pub strikes: String,
    pub status: String,
    pub name: String,
    pub size: String,
    pub eta: String,
}

pub fn print_table(contents: &Vec<TableContent>) {
    if contents.len() > 0 {
        let mut table = Table::new(contents);
        table.with(Style::rounded());
        println!("{}", table.to_string())
    } else {
        println!("╭───────────────────────────────────────────────────────────────────────────╮");
        println!("│                            No torrents found                              │");
        println!("╰───────────────────────────────────────────────────────────────────────────╯");
    }
}

pub fn ms_to_eta(ms: u64) -> String {
    // This will pretty-print the ETA.
    let eta = format_duration(Duration::from_millis(ms)).to_string();

    if format!("{eta}") == "0s" {
        String::from("Infinite")
    } else {
        eta
    }
}

pub fn format_to_bytes(s: String) -> u64 {
    // Converts "1.5 GB" to 1_500_000 for example.
    s.parse::<ByteSize>().unwrap().0
}

pub fn format_to_ms(time: String) -> Option<u64> {
    let parts: Vec<&str> = time.split(|c| c == ':' || c == '.').collect();

    // Check if we have at least hours, minutes, and seconds
    if parts.len() < 3 {
        return None;
    }

    let mut days: u64 = 0;
    let hours: u64;
    let minutes: u64;
    let seconds: u64;

    match parts.len() {
        // For the format "12:34:56"
        3 => {
            hours = parts[0].parse().ok()?;
            minutes = parts[1].parse().ok()?;
            seconds = parts[2].parse().ok()?;
        }
        // For the format "12.34:56:78"
        4 => {
            days = parts[0].parse().ok()?;
            hours = parts[1].parse().ok()?;
            minutes = parts[2].parse().ok()?;
            seconds = parts[3].parse().ok()?;
        }
        _ => return None,
    }

    // Calculate total milliseconds
    let total_ms = ((days * 24 + hours) * 3600 + minutes * 60 + seconds) * 1000;
    Some(total_ms)
}

#[derive(Debug)]
pub struct Args {
    pub baseurl: String,
    pub apikey: String,
    pub platform: String,
    pub time_threshold: String,
    pub size_threshold: String,
    pub check_interval: String,
    pub strike_threshold: u32,
    pub aggresive_strikes: bool,
}

pub fn args() -> Args {
    // Get all arguments from the command line.
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() != 8 {
        panic!("Insufficient arguments, consult the README.md for options.")
    }

    Args {
        baseurl: args.get(0).unwrap().to_string(),
        apikey: args.get(1).unwrap().to_string(),
        platform: args.get(2).unwrap().to_string(),
        time_threshold: args.get(3).unwrap().to_string(),
        size_threshold: args.get(4).unwrap().to_string(),
        check_interval: args.get(5).unwrap().to_string(),
        strike_threshold: args.get(6).unwrap().parse().unwrap(),
        aggresive_strikes: args.get(7).unwrap().parse().unwrap(),
    }
}
