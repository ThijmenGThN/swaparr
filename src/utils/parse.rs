use std::time::Duration;

use bytesize::ByteSize;
use humantime::format_duration;

use crate::{queue, utils};

// This will pretty-print an ETA from milliseconds.
pub fn ms_to_eta_string(ms: &u64) -> String {
    let eta = format_duration(Duration::from_millis(ms.clone())).to_string();

    if format!("{eta}") == "0s" {
        String::from("Infinite")
    } else {
        eta
    }
}

// Converts human-readable time notation to milliseconds.
pub fn string_time_notation_to_ms(string: &String) -> Result<i64, ms_converter::Error> {
    ms_converter::ms(string)
}

// This will convert for example "1 TB", "512 MB", <"1.5 GB" to 1500000 (bytes)>.
pub fn string_bytesize_to_bytes(string: &String) -> Result<ByteSize, String> {
    string.parse::<ByteSize>()
}

// Converts human-readable string (from Starr API) to milliseconds.
pub fn string_hms_to_ms(string: &String) -> u64 {
    let parts: Vec<&str> = string.split(|c| c == ':' || c == '.').collect();

    // Check if we have at least HH:MM:SS -> hours, minutes, and seconds
    if parts.len() < 3 {
        return 0;
    }

    let mut days: u64 = 0;
    let hours: u64;
    let minutes: u64;
    let seconds: u64;

    match parts.len() {
        // Format-type "12:34:56"
        3 => {
            hours = parts[0].parse().unwrap_or_else(|_| 0);
            minutes = parts[1].parse().unwrap_or_else(|_| 0);
            seconds = parts[2].parse().unwrap_or_else(|_| 0);
        }
        // Format-type "12.34:56:78"
        4 => {
            days = parts[0].parse().unwrap_or_else(|_| 0);
            hours = parts[1].parse().unwrap_or_else(|_| 0);
            minutes = parts[2].parse().unwrap_or_else(|_| 0);
            seconds = parts[3].parse().unwrap_or_else(|_| 0);
        }
        _ => return 0,
    }

    // Calculate total milliseconds and return.
    ((days * 24 + hours) * 3600 + minutes * 60 + seconds) * 1000
}

// Returns the API version based on platform.
pub fn baseapi(platform: &str, baseurl: &str) -> String {
    match platform {
        "radarr" => format!("{baseurl}/api/v3/"),
        "sonarr" => format!("{baseurl}/api/v3/"),
        "lidarr" => format!("{baseurl}/api/v1/"),
        "readarr" => format!("{baseurl}/api/v1/"),
        "whisparr" => format!("{baseurl}/api/v3/"),
        _ => {
            utils::log::alert(
                "FATAL",
                "Unknown \"PLATFORM\" value.",
                "Either set it to \"radarr\", \"sonarr\", \"lidarr\", \"readarr\" or \"whisparr\".",
                None,
            );
            utils::system::exit(1);
        }
    }
}

// Returns the API endpoint based on platform.
pub fn queueapi(platform: &str, baseapi: &str, apikey: &str) -> String {
    let default_page_size = 256;
    match platform {
        "radarr" => format!("{baseapi}queue?includeUnknownMovieItems=true&includeMovie=true&pageSize={default_page_size}&apikey={apikey}"),
        "sonarr" => format!("{baseapi}queue?includeUnknownSeriesItems=true&includeSeries=true&pageSize={default_page_size}&apikey={apikey}"),
        "lidarr" => format!("{baseapi}queue?includeUnknownArtistItems=true&includeArtist=true&includeAlbum=true&pageSize={default_page_size}&apikey={apikey}"),
        "readarr" => format!("{baseapi}queue?includeUnknownAuthorItems=true&includeAuthor=true&includeBook=true&pageSize={default_page_size}&apikey={apikey}"),
        "whisparr" => format!("{baseapi}queue?includeUnknownSeriesItems=true&includeSeries=true&includeEpisode=true&pageSize={default_page_size}&apikey={apikey}"),
        _ => {
            utils::log::alert(
                "FATAL",
                "Unknown \"PLATFORM\" value.",
                "Either set it to \"radarr\", \"sonarr\", \"lidarr\", \"readarr\" or \"whisparr\".",
                None,
            );
            utils::system::exit(1);
        }
    }
}

// This function extracts the name from a record based on the provided platform.
pub fn recordname(platform: &str, record: &queue::Record) -> String {
    let mut title: &str = match platform {
        "radarr" => match record.movie.as_ref() {
            Some(movie) => &movie.title,
            None => "Unknown",
        },
        "sonarr" => match record.series.as_ref() {
            Some(series) => &series.title,
            None => "Unknown",
        },
        "lidarr" => match record.album.as_ref() {
            Some(album) => &album.title,
            None => "Unknown",
        },
        "readarr" => match record.book.as_ref() {
            Some(book) => &book.title,
            None => "Unknown",
        },
        "whisparr" => match record.series.as_ref() {
            Some(series) => &series.title,
            None => "Unknown",
        },
        _ => "Unknown",
    };

    if title.is_empty() {
        title = "Unknown"
    }

    String::from(title)
}

// String to boolean translator.
pub fn string_to_bool(string: String) -> Result<bool, String> {
    match string.to_ascii_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(string),
    }
}
