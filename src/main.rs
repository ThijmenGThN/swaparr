use std::{collections::HashMap, thread::sleep, time::Duration};

mod libs;
mod queue;
mod tests;
mod utils;

fn main() {
    let env = utils::system::env();
    let baseapi = utils::parse::baseapi(&env.platform, &env.baseurl);
    let queueapi = utils::parse::queueapi(&env.platform, &baseapi, &env.apikey);

    // Test: Health-checks
    tests::api::test(&env.platform, &baseapi, &env.apikey);

    // Displays initial "banner" with set configurations.
    utils::log::banner(&env);

    let mut strikelist: HashMap<u32, u32> = HashMap::new();

    loop {
        let queue_items = queue::get(&env.platform, &queueapi);

        // Cleanup downloads tracker
        strikelist.retain(|&k, _| queue_items.iter().any(|item| item.id == k));

        // Process downloads - Also prints table to logs
        queue::process(&env, &baseapi, queue_items, &mut strikelist);

        println!(" ─ Checking again in {}..\n", &env.scan_interval);

        // Thread sleeper
        sleep(Duration::from_millis(
            match utils::parse::string_time_notation_to_ms(&env.scan_interval) {
                Ok(scan_interval_ms) => scan_interval_ms as u64,
                Err(_) => 10 * 60 * 1000,
            },
        ));
    }
}
