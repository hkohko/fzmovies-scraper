use std::env;
use anyhow::Result;
pub struct ProjPath {}
impl ProjPath {
    fn cur_dir() -> Result<std::path::PathBuf>{
        Ok(env::current_dir()?)
    }
    pub fn res_path() -> Result<std::path::PathBuf> {
        Ok(ProjPath::cur_dir()?.join("res"))
    }
}