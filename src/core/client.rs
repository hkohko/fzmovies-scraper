use crate::ResPath;
use anyhow::Result;
use native_tls;
use std::fs;
use std::io::{self, prelude::*};
use std::sync::Arc;
use ureq;
use url::Url;

pub fn client(url: &Url, save: bool, filename: Option<&str>) -> Result<String> {
    let agent = ureq::AgentBuilder::new()
        .tls_connector(Arc::new(native_tls::TlsConnector::new()?))
        .build();
    let resp = agent.get(&url.as_str()).call()?;
    let page = resp.into_string()?;
    if save {
        let filename = ResPath::new(filename.unwrap_or("None"));
        let f = fs::File::create(filename.create_path()?)?;
        let mut writer = io::BufWriter::new(f);
        writer
            .write(page.as_bytes())
            .expect("Error writing file.\n\n");
    }
    Ok(page)
}
