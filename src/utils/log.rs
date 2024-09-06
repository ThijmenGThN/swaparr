use crate::utils;

pub fn empty() {
    println!(
        "\n╭──────────────────────────────────────────────────────────────────────────╮\n│                            No downloads found                            │\n╰──────────────────────────────────────────────────────────────────────────╯\n",
    );
}

pub fn alert(method: &str, title: &str, message: &str, error: Option<String>) {
    println!("\n ─ {}", method);
    println!("╭─╮ {}", title);
    println!("╰─╯ {}", message);

    if let Some(error) = error {
        println!("{}", error);
    }

    println!("\n");
}

pub fn banner(env: &utils::system::Envs) {
    println!("\n ── Swaparr ───── \n");
    println!("╭─╮ Platform: {}", &env.platform);
    println!("│ │ Max strikes: {}", &env.max_strikes);
    println!("│ │ Scan interval: {}", env.scan_interval);
    println!("│ │ Max download time: {}", &env.max_download_time);
    println!("│ │ Ignore above size: {}", &env.ignore_above_size);
    println!("╰─╯ Remove from client: {}\n", &env.remove_from_client);

    // Open-Source = ❤️
    println!("╭─╮ Has Swaparr been useful and do you like open-source projects?");
    println!("│ │ Then please do consider to star the repository on GitHub.");
    println!("╰─╯ Your gesture means a lot and will help improve Swaparr!");
    println!("\n ──────────────── \n")
}
