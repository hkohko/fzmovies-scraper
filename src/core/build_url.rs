use crate::data::constant::BASE_URL;
use anyhow::Result;
use url::Url;

pub fn build_url(genre: &str, sort: &str, page: &str) -> Result<Url> {
    let base = Url::parse(BASE_URL)?;
    let url = base.join("genre.php")?;
    let params = Url::parse_with_params(
        url.as_str(),
        &[("catID", "2"), ("genre", genre), ("by", sort), ("pg", page)],
    )?;

    Ok(params)
}
