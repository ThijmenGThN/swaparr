use crate::system;

pub fn empty() {
    println!(
        "\n╭───────────────────────────────────────────────────────────────────────────╮\n│                            No torrents found                              │\n╰───────────────────────────────────────────────────────────────────────────╯\n",
    );
}

pub fn alert(method: &str, title: String, message: String, error: Option<String>) {
    println!("\n ─ {}", method);
    println!("╭─╮ {}", title);
    println!("╰─╯ {}", message);

    if let Some(error) = error {
        println!("{}", error);
    }

    println!("\n");
}

pub fn banner(env: &system::Envs) {
    println!("\n ─ Swaparr");
    println!("╭─╮ Platform: {}", &env.platform);
    println!("│ │ Time threshold: {}", &env.time_threshold);
    println!("│ │ Size threshold: {}", &env.size_threshold);
    println!("│ │ Strike threshold: {}", &env.strike_threshold);
    println!("╰─╯ Aggresive strikes: {}", &env.aggresive_strikes);
    println!(" ─ Checking every: {}\n", env.check_interval);

    // Open-Source = <3
    println!("╭─╮ Has Swaparr been useful and do you like open-source projects?");
    println!("│ │ Then please do consider to star the repository on GitHub.");
    println!("╰─╯ Your gesture means a lot and will help improve Swaparr!\n");
}
