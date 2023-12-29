use anyhow::Result;
use ureq;
use url::Url;
fn client_main() {}
pub fn client(url: &Url) -> Result<String> {
    let agent = ureq::AgentBuilder::new().build();
    let resp = agent.get(&url.as_str()).call()?;
    Ok(resp.into_string()?)
}
