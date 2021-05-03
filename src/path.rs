use serde_derive::*;

use crate::config::Config;
use crate::error::{Error, ErrorKind};
use crate::errorhere;
use std::env::var;
use std::path::Path as stdPath;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path(Vec<String>);

impl Path {
    pub fn load() -> Path {
        let var: String = var("PATH").unwrap_or("/bin:/usr/bin".to_string());
        let vec: Vec<String> = var.split(':').map(|x| x.to_string()).collect();
        Path(vec)
    }

    pub fn add(mut self, config: &Config) -> Path {
        if let Some(mut vec) = config.get_paths() {
            self.0.append(&mut vec.0);
        }

        self
    }

    fn to_vec(&self) -> Vec<String> {
        self.0.to_vec()
    }

    pub fn find_command(&self, bin: &str) -> Result<String, Error> {
        let vec = self.to_vec();
        Self::find_current(&vec, bin, 0)
    }

    fn find_current(p: &[String], bin: &str, place: usize) -> Result<String, Error> {
        if p.len() - 1 == place {
            errorhere!(ErrorKind::CmdNotFound, bin)?;
        }
        if stdPath::new(&bin).exists() {
            return Ok(bin.to_string());
        }
        let full = format!("{}/{}", p[place], bin);
        if stdPath::new(&full).exists() {
            return Ok(full);
        }
        Self::find_current(p, bin, place + 1)
    }
}
