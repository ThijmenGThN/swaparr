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
    // Yes, a lot of printlines, but it looks better like this.
    println!("\n ── Swaparr ───── \n");
    println!("╭─╮ Platform: {}", &env.platform);
    println!("│ │ Max strikes: {}", &env.max_strikes);
    println!("│ │ Scan interval: {}", env.scan_interval);
    println!("│ │ Max download time: {}", &env.max_download_time);
    println!("│ │ Ignore above size: {}", &env.ignore_above_size);
    println!("╰─╯ Remove from client: {}\n", &env.remove_from_client);

    if &env.dry_run == "true" {
        println!("╭─╮ Dry-run: true");
        println!("╰─╯ All destructive actions are negated.\n");
    }

    // Open-Source = ❤️
    println!("╭─╮ Be part of Swaparr's journey ⭐ Star us on GitHub!");
    println!("╰─╯ Your support strengthens the open-source community.");
    println!("\n ──────────────── \n")
}
