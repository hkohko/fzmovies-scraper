use crate::core::build_url::build_url;
use crate::core::path::ProjPath;
use crate::data::constant::BASE_URL;
use anyhow::Result;
use std::io::prelude::*;
use ureq;
use url::Url;

pub fn example_main() -> Result<()> {
    request_save()?;
    Ok(())
}

pub fn request_save() -> Result<()> {
    let agent = ureq::AgentBuilder::new().build();
    let url = build_url()?;
    let resp = agent.get(url.as_str()).call()?;
    let f = std::fs::File::create(ProjPath::res_path()?.join("example.html"))?;
    let mut writer = std::io::BufWriter::new(f);
    writer.write_all(resp.into_string()?.as_bytes())?;

    Ok(())
}
