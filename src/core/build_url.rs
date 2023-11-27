use url::Url;
use anyhow::Result;
use crate::data::constant::BASE_URL;

pub fn build_url() -> Result<Url> {
    let genre = "Action";
    let sort = "downloads";
    let page = "1";

    let base = Url::parse(BASE_URL)?;
    let url = base.join("genre.php")?;
    let params = Url::parse_with_params(
        url.as_str(),
        &[
            ("catID", "2"),
            ("genre", genre),
            ("by", sort),
            ("pg", page),
        ],
    )?;

    Ok(params)
}