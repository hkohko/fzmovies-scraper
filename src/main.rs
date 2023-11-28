use fzmovies::core::{build_url::build_url, client::client, scraper::scp};
use anyhow::Result;

fn main() -> Result<()>{
    let url = build_url("Action", "downloads", "10")?;
    let resp = client(url)?;
    let _ = scp(resp)?;
	Ok(())
}
