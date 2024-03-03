use reqwest;

enum PlatformType { "Series", "Movie" }

fn fetch(type: PlatformType) {

    let params = format!("includeUnknown{type}Items=true&include{type}=true");
    let url = format!("{domain}/api/v3/queue");

    reqwest::Client::get(&self, url)

}

pub fn get(domain: &String, api_key: &String) {

    println!("Hello, world!");

}
