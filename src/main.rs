use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ComicVineResponse {
    status: String,
    error: String,
    limit: i32,
    offset: i32,
    number_of_page_results: i32,
    number_of_total_results: i32,
    results: Vec<Publisher>,
}

#[derive(Debug, Deserialize)]
struct Publisher {
    id: i32,
    api_detail_url: String,
    name: String,
    site_detail_url: String,
    count_of_issues: i32,
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
        println!("Request successful");
        println!("Headers:\n{:#?}", response.headers());
        println!("Body:\n{}", response.text()?);
        // Deserialize the JSON response into a Post struct
        // let comic: ComicVineResponse = response.json()?;
        // println!("{:?}", comic);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
