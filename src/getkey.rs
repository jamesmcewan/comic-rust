pub fn get_key() -> String {
    let _ = dotenv_vault::dotenv();
    std::env::var("COMICVINE_KEY").expect("COMICVINE_KEY must be set")
}
