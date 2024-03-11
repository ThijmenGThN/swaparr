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

    // Health check the API, verbosely checks if a connection can be established.
    health::check(&env);

    // Displays initial "banner" with set configurations.
    logger::banner(&env);

    // Get the queue get url based on the platform.
    let queue_get_url = parser::env_to_queue_get(&env);

    // List of striked torrents.
    let mut strikelist: HashMap<u32, u32> = HashMap::new();

    // Main striker-runtime thread.
    loop {
        // Get all active torrents from the queue.
        let queue_items = queue::get(&queue_get_url, &env.platform);

        // Cleanup torrents that no longer exists in the strikelist.
        strikelist.retain(|&k, _| queue_items.iter().any(|item| item.id == k));

        // Process torrents in the queue, a table with details will also be printed.
        queue::process(queue_items, &mut strikelist, &env);

        println!(" ─ Checking again in {}..\n", &env.check_interval);

        // CHECK_INTERVAL sleeper for the main thread.
        sleep(Duration::from_millis(
            match parser::string_to_ms(&env.check_interval) {
                Ok(check_interval_ms) => check_interval_ms as u64,
                Err(_) => 10 * 60 * 1000, // Using default, 10 minutes
            },
        ));
    }
}
