use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;

pub mod core {
    pub mod client;
}
pub mod file_quality {
    pub mod build_url;
    pub mod db;
    pub mod scraper;
}
pub mod movie_page {
    pub mod build_url;
    pub mod db;
    pub mod scraper;
}
pub mod data {
    pub mod constant;
}
pub struct DBPath<'a> {
    name: &'a str,
}
impl<'a> DBPath<'a> {
    pub fn new(n: &str) -> DBPath {
        DBPath { name: n }
    }
    pub fn create_path(&self) -> Result<PathBuf> {
        let cur_dir = env::current_dir()?;
        let db_folder = cur_dir.join("db");
        if !db_folder.exists() {
            fs::create_dir(&db_folder)?;
        }
        let name = format!("{}.db", self.name);
        Ok(db_folder.join(name.as_str()))
    }
}
#[derive(Debug)]
pub struct Data {
    path: String,
}
pub struct ResPath<'a> {
    name: &'a str,
}
impl<'a> ResPath<'a> {
    fn cur_dir() -> Result<std::path::PathBuf> {
        Ok(env::current_dir()?)
    }
    pub fn new(filename: &str) -> ResPath {
        ResPath { name: filename }
    }
    pub fn create_path(&self) -> Result<std::path::PathBuf> {
        let res_path = ResPath::cur_dir()?.join("res");
        if !res_path.exists() {
            std::fs::create_dir(&res_path)
                .expect(format!("Failed to create dir: {:?}\n\n", &res_path).as_str());
        }
        Ok(res_path.join(self.name))
    }
}
