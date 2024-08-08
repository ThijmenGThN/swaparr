use std::{collections::HashMap, thread::sleep, time::Duration};

mod health;
mod logger;
mod parser;
mod queue;
mod render;
mod system;

fn main() {
    // Load environment variables.
    let env = system::env();

    // Get the base and queue api url based on the platform.
    let baseapi = parser::baseapi(&env.platform, &env.baseurl);
    let queueapi = parser::queueapi(&env.platform, &baseapi, &env.apikey);

    // Health check the API, verbosely checks if a connection can be established.
    health::api(&env.platform, &baseapi, &env.apikey);

    // Displays initial "banner" with set configurations.
    logger::banner(&env);

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

        println!(" ─ Checking again in {}..\n", &env.check_interval);

        // CHECK_INTERVAL sleeper for the main thread.
        sleep(Duration::from_millis(
            match parser::string_time_notation_to_ms(&env.check_interval) {
                Ok(check_interval_ms) => check_interval_ms as u64,
                Err(_) => 10 * 60 * 1000, // Using default, 10 minutes
            },
        ));
    }
}
