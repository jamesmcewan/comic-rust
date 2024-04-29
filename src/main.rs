use reqwest::header::USER_AGENT;
mod comicstructs;
mod geturl;
use crate::comicstructs::ComicVineResponse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let _ = dotenv_vault::dotenv();
    let publisher_id = &args[1].to_string();
    let api_key = std::env::var("COMICVINE_KEY").expect("COMICVINE_KEY must be set");
    let full_url = crate::geturl::get_url(api_key, publisher_id.to_string());

    println!("looking for: {}\n", publisher_id);

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
                print!("Result found\n\n");
                print!("--------------------------------\n\n");
                println!("Name: {:?}\n", value.results.aliases);
                print!("--------------------------------\n\n");
                for volume in &value.results.volumes {
                    println!("{:?}\n", volume.name);
                    println!("{:?}\n", volume.site_detail_url);
                    print!("--------------------------------\n\n");
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
