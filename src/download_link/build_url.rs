use crate::data::constant::BASE_URL;
use crate::Data;
use anyhow::Result;
use url;

pub fn build_url(vec: Vec<Data>) -> Result<url::Url> {
    let base = url::Url::parse(BASE_URL)?;
    let mut link: url::Url = base.clone();
    for url in vec.iter() {
        link = base.join(url.path.as_str())?;
        break;
    }
    Ok(link)
}
