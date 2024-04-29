use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ComicVineResponse {
    pub results: ComicResult,
}

#[derive(Debug, Deserialize)]
pub struct ComicResult {
    pub aliases: String,
    pub volumes: Vec<ComicVolume>,
}

#[derive(Debug, Deserialize)]
pub struct ComicVolume {
    pub name: String,
    pub site_detail_url: String,
}
