use crate::system;

pub fn alert(method: &str, title: String, message: String, isolated: bool) {
    print!("{}", if isolated { "\n" } else { "" });
    println!(" ─ {}", method);
    println!("╭─╮ {}", title);
    println!("╰─╯ {}", message);
    print!("{}", if isolated { "\n" } else { "" });
}

pub fn banner(env: &system::Envs, isolated: bool) {
    print!("{}", if isolated { "\n" } else { "" });
    println!(" ─ Swaparr");
    println!("╭─╮ Platform: {}", &env.platform);
    println!("│ │ Time threshold: {}", &env.time_threshold);
    println!("│ │ Size threshold: {}", &env.size_threshold);
    println!("│ │ Strike threshold: {}", &env.strike_threshold);
    println!("╰─╯ Aggresive strikes: {}", &env.aggresive_strikes);
    println!(" ─ Checking every: {}", env.check_interval);
    print!("{}", if isolated { "\n" } else { "" });
}

pub fn empty(isolated: bool) {
    print!("{}", if isolated { "\n" } else { "" });
    println!("╭───────────────────────────────────────────────────────────────────────────╮");
    println!("│                            No torrents found                              │");
    println!("╰───────────────────────────────────────────────────────────────────────────╯");
    print!("{}", if isolated { "\n" } else { "" });
}
