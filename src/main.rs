use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ComicVineResponse {
    number_of_total_results: i32,
    results: ComicResult,
}

#[derive(Debug, Deserialize)]
struct ComicResult {
    aliases: String,
    volumes: Vec<ComicVolume>,
}

#[derive(Debug, Deserialize)]
struct ComicVolume {
    name: String,
    site_detail_url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv_vault::dotenv();
    let url = "https://comicvine.gamespot.com/api/publisher/";
    let publisher_id = "4010-4212";
    let api_key = std::env::var("COMICVINE_KEY").expect("COMICVINE_KEY must be set");
    let query = "&format=json&sort=name:asc";
    let full_url = format!("{}{}/?api_key={}{}", url, publisher_id, api_key, query);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(full_url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0",
        )
        .send()?;

    if response.status().is_success() {
        let json_value = response.json::<ComicVineResponse>();
        match json_value {
            Ok(value) => {
                println!("{:?}", value.number_of_total_results);
                println!("{:?}", value.results.aliases);
                for volume in value.results.volumes {
                    println!("{:?}", volume.name);
                    println!("{:?}", volume.site_detail_url);
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
