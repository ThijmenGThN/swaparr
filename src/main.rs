const VERSION: &str = "0.1.0 Pre-Release";

use std::{collections::HashMap, thread::sleep, time::Duration};

use ms_converter::ms as format_to_ms;

use utils::TableContent;

mod queue;
mod utils;

fn main() {
    // ----- Arguments & Config parsing -----

    let args = utils::args();
    let time_threshold_ms = format_to_ms(&args.time_threshold).unwrap() as u64;
    let size_threshold_bytes = utils::format_to_bytes(args.size_threshold.clone());
    let check_interval_ms = format_to_ms(&args.check_interval).unwrap() as u64;

    // Based on the platform, use a different strategy to approach radarr or sonarr their API.
    let queue_get_url = {
        let method = match args.platform.as_str() {
            "radarr" => "Movie",
            "sonarr" => "Series",
            _ => {
                panic!("Unknown platform, either set it to 'radarr' or 'sonarr'.");
            }
        };
        let params = format!("includeUnknown{method}Items=true&include{method}=true");
        format!(
            "{}/api/v3/queue?{}&apikey={}",
            args.baseurl, params, args.apikey
        )
    };

    // ----- Display Settings -----

    println!("\n ─ Swappar {}", VERSION);

    println!("╭─╮ Platform: {}", &args.platform);
    println!("│ │ Time threshold: {}", &args.time_threshold);
    println!("│ │ Size threshold: {}", &args.size_threshold);
    println!("│ │ Strike threshold: {}", &args.strike_threshold);
    println!("╰─╯ Aggresive strikes: {}", &args.aggresive_strikes);

    println!(" ─ Checking every: {}\n", args.check_interval);

    // ----- Striker Runtime -----

    let mut strikes: HashMap<u32, u32> = HashMap::new();

    loop {
        // Table rows that will be pretty-printed to the terminal.
        let mut table_contents: Vec<TableContent> = vec![];

        // Get all active torrents from the queue.
        let queue_items = queue::get(&queue_get_url);

        // Cleanup torrents that no longer exists from strikes registry.
        strikes.retain(|&k, _| queue_items.iter().any(|item| item.id == k));

        // Loop over all active torrents from the queue.
        for torrent in queue_items {
            let id = torrent.id.clone();
            let mut status = String::from("Normal");

            // Add torrent id to strikes with default "0" if it does not exist yet.
            if !strikes.contains_key(&id) {
                strikes.insert(id, 0);
            }

            // -- Bypass Rules -- Rules that define if a torrent is eligible to be striked.

            let mut bypass: bool = false;

            // Torrent is being processed or the time is infinite.
            if torrent.eta == 0 && !args.aggresive_strikes {
                status = String::from("Pending");
                bypass = true;
            }

            // Torrent is larger than set threshold.
            if torrent.size >= size_threshold_bytes {
                status = String::from("Ignored");
                bypass = true;
            }

            // -- Strike rules -- Rules that define when to strike a torrent.

            if !bypass {
                // Torrent will take longer than set threshold.
                if (torrent.eta >= time_threshold_ms)
                    || (torrent.eta == 0 && args.aggresive_strikes)
                {
                    // Increment strikes if it's below set maximum.
                    if strikes.get(&id).unwrap() < &args.strike_threshold {
                        strikes.insert(id, strikes.get(&id).unwrap() + 1);
                    }
                    status = String::from("Striked");
                }

                // Torrent meets set amount of strikes, a request to delete will be sent.
                if strikes.get(&id).unwrap() >= &args.strike_threshold {
                    let queue_delete_url = format!(
                        "{}/api/v3/queue/{}?removeFromClient=true&blocklist=true&apikey={}",
                        args.baseurl, id, args.apikey
                    );
                    queue::delete(&queue_delete_url);
                    status = String::from("Removed");
                }
            }

            // -- Logging --

            // Add torrent to pretty-print table.
            table_contents.push(TableContent {
                strikes: format!(
                    "{}/{}",
                    strikes.get(&id).unwrap_or_else(|| &0),
                    args.strike_threshold
                ),
                status,
                name: torrent.name.chars().take(32).collect::<String>(),
                eta: utils::ms_to_eta(torrent.eta),
                size: format!("{:.2} GB", (torrent.size as f64 / 1000000000.0)).to_string(),
            })
        }

        // Print table to terminal.
        utils::print_table(&table_contents);
        println!(" ─ Checking again in {}..\n", &args.check_interval);

        sleep(Duration::from_millis(check_interval_ms.clone()));
    }
}
