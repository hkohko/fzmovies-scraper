use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;

pub mod core {
    pub mod build_url;
    pub mod client;
    pub mod db;
    pub mod path;
    pub mod scraper;
}
pub mod data {
    pub mod constant;
}
pub mod opts {
    //    pub mod example;
}
pub struct DBPath<'a> {
    pub name: &'a str,
}
impl<'a> DBPath<'a> {
    pub fn new(&self) -> Result<PathBuf> {
        let cur_dir = env::current_dir()?;
        let db_folder = cur_dir.join("db");
        if !db_folder.exists() {
            fs::create_dir(&db_folder)?;
        }
        let name = format!("{}.db", self.name);
        Ok(db_folder.join(name.as_str()))
    }
}
