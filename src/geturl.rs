pub fn get_url(api_key: String, publisher_id: String) -> String {
    let url = "https://comicvine.gamespot.com/api/publisher/";
    let params = "&format=json&sort=name:asc";
    let full_url = format!("{}{}/?api_key={}{}", url, publisher_id, api_key, params);
    full_url
}
