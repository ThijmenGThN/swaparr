const VERSION: &str = "0.2.0 Pre-Release";

use std::{collections::HashMap, thread::sleep, time::Duration};

mod parser;
mod queue;
mod render;
mod system;

fn main() {
    // Load environment variables.
    let env = system::env();

    // Based on the platform, use a different strategy to approach radarr or sonarr their API.
    let queue_get_url = {
        let method = match env.platform.as_str() {
            "radarr" => "Movie",
            "sonarr" => "Series",
            _ => {
                panic!("Unknown platform, either set it to 'radarr' or 'sonarr'.");
            }
        };
        let params = format!("includeUnknown{method}Items=true&include{method}=true");
        format!(
            "{}/api/v3/queue?{}&apikey={}",
            env.baseurl, params, env.apikey
        )
    };

    // ----- Display Settings -----

    println!("\n ─ Swaparr {}", VERSION);

    println!("╭─╮ Platform: {}", &env.platform);
    println!("│ │ Time threshold: {}", &env.time_threshold);
    println!("│ │ Size threshold: {}", &env.size_threshold);
    println!("│ │ Strike threshold: {}", &env.strike_threshold);
    println!("╰─╯ Aggresive strikes: {}", &env.aggresive_strikes);

    println!(" ─ Checking every: {}\n", env.check_interval);

    // ----- Striker Runtime -----

    let mut strikelist: HashMap<u32, u32> = HashMap::new();

    loop {
        // Table rows that will be pretty-printed to the terminal.
        let mut table_contents: Vec<render::TableContent> = vec![];

        // Get all active torrents from the queue.
        let queue_items = queue::get(&queue_get_url);

        // Cleanup torrents that no longer exists from strikes registry.
        strikelist.retain(|&k, _| queue_items.iter().any(|item| item.id == k));

        // Loop over all active torrents from the queue.
        for torrent in queue_items {
            let id = torrent.id.clone();
            let mut status = String::from("Normal");

            // Add torrent id to strikes with default "0" if it does not exist yet.
            let mut strikes: u32 = match strikelist.get(&id) {
                Some(strikes) => strikes.clone(),
                None => {
                    strikelist.insert(id, 0);
                    0
                }
            };

            // -- Bypass Rules -- Rules that define if a torrent is eligible to be striked.

            let mut bypass: bool = false;

            // Torrent is being processed or the time is infinite.
            if torrent.eta == 0 && !env.aggresive_strikes {
                status = String::from("Pending");
                bypass = true;
            }

            // Torrent is larger than set threshold.
            let size_threshold_bytes = parser::string_bytesize_to_bytes(&env.size_threshold);
            if torrent.size >= size_threshold_bytes {
                status = String::from("Ignored");
                bypass = true;
            }

            // -- Strike rules -- Rules that define when to strike a torrent.

            if !bypass {
                // Torrent will take longer than set threshold.
                let time_threshold_ms = parser::string_hms_to_ms(&env.time_threshold);
                if (torrent.eta >= time_threshold_ms) || (torrent.eta == 0 && env.aggresive_strikes)
                {
                    // Increment strikes if it's below set maximum.
                    if strikes < env.strike_threshold {
                        strikes += 1;
                        strikelist.insert(id, strikes);
                    }
                    status = String::from("Striked");
                }

                // Torrent meets set amount of strikes, a request to delete will be sent.
                if strikes >= env.strike_threshold {
                    let queue_delete_url = format!(
                        "{}/api/v3/queue/{}?removeFromClient=true&blocklist=true&apikey={}",
                        env.baseurl, id, env.apikey
                    );
                    queue::delete(&queue_delete_url);
                    status = String::from("Removed");
                }
            }

            // -- Logging --

            // Add torrent to pretty-print table.
            table_contents.push(render::TableContent {
                strikes: format!("{}/{}", strikes, env.strike_threshold),
                status,
                name: torrent.name.chars().take(32).collect::<String>(),
                eta: parser::ms_to_eta_string(&torrent.eta),
                size: format!("{:.2} GB", (torrent.size as f64 / 1000000000.0)).to_string(),
            })
        }

        // Print table to terminal.
        render::table(&table_contents);
        println!(" ─ Checking again in {}..\n", &env.check_interval);

        sleep(Duration::from_millis(
            match parser::string_to_ms(&env.check_interval) {
                Ok(check_interval_ms) => check_interval_ms as u64,
                Err(_) => 10 * 60 * 1000, // Using default, 10 minutes
            },
        ));
    }
}
