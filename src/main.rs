use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
struct Publisher {
    id: i32,
    api_detail_url: String,
    name: String,
    site_detail_url: String,
    count_of_issues: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv_vault::dotenv();
    let url = "https://comicvine.gamespot.com/api/publisher/";
    let publisher_id = "4010-4212";
    let api_key = std::env::var("COMICVINE_KEY").expect("COMICVINE_KEY must be set");
    let query = "&format=json&sort=name:asc";
    let full_url = format!("{}{}/?api_key={}{}", url, publisher_id, api_key, query);

    let response = reqwest::blocking::get(full_url)?;

    if response.status().is_success() {
        // Deserialize the JSON response into a Post struct
        let post: ComicVineResponse = response.json()?;
        println!("{:?}", post);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
