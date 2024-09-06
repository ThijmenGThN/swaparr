use std::{collections::HashMap, thread::sleep, time::Duration};

mod libs;
mod queue;
mod tests;
mod utils;

fn main() {
    // Load environment variables.
    let env = utils::system::env();

    // Get the base and queue api url based on the platform.
    let baseapi = utils::parse::baseapi(&env.platform, &env.baseurl);
    let queueapi = utils::parse::queueapi(&env.platform, &baseapi, &env.apikey);

    // Health check the API, verbosely checks if a connection can be established.
    tests::api::test(&env.platform, &baseapi, &env.apikey);

    // Displays initial "banner" with set configurations.
    utils::log::banner(&env);

    // List of striked downloads.
    let mut strikelist: HashMap<u32, u32> = HashMap::new();

    // Main striker-runtime thread.
    loop {
        // Get all active downloads from the queue.
        let queue_items = queue::get(&env.platform, &queueapi);

        // Cleanup downloads that no longer exists in the strikelist.
        strikelist.retain(|&k, _| queue_items.iter().any(|item| item.id == k));

        // Process downloads in the queue, a table with details will also be printed.
        queue::process(&env, &baseapi, queue_items, &mut strikelist);

        println!(" ─ Checking again in {}..\n", &env.scan_interval);

        // SCAN_INTERVAL sleeper for the main thread.
        sleep(Duration::from_millis(
            match utils::parse::string_time_notation_to_ms(&env.scan_interval) {
                Ok(scan_interval_ms) => scan_interval_ms as u64,
                Err(_) => 10 * 60 * 1000, // Using default, 10 minutes
            },
        ));
    }
}
