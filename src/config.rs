use crate::{bunny::Bunny, error::*};
use crate::path::Path;
use crate::command::Command;

use serde_derive::*;
use std::{fs::File, io::Read};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	show_welcome: Option<bool>,
    welcome_text: Option<String>,
    startup: Option<Startup>,
    pub path: Option<Path>,
}

impl Config {
    pub fn load() -> Result<Config, Error> {
        let mut file = File::open("config.toml")?;
        let cfg_dat: String;
        {
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            cfg_dat = buf.trim().into();
        }

        let config: Config = toml::from_str(&cfg_dat)?;

        Ok(config)
    }

    pub fn welcome_text(&self) -> String {
        if let Some(text) = self.welcome_text.clone() {
            return text;
        }
        format!("Bunny Line Shell Version {}", crate::VERSION)
    }

    pub fn get_paths(&self) -> Option<Path> {
        self.path.clone()
    }

	pub async fn init(&self, bunny: &Bunny) -> Result<(), Error> {
		if self.show_welcome == None {
			self.welcome_text();
		} else if let Some(true) = self.show_welcome {
			self.welcome_text();
		}

		if let Some(startup) = &self.startup {
			startup.init(bunny).await?;
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Startup(Vec<String>);

impl Startup {
	pub async fn init(&self, bunny: &Bunny) -> Result<(), Error> {
		for full in &self.0 {
			let cmd = Command::new(full, &bunny.path).await?;
			cmd.exec().await?;
		}
		Ok(())
	}
}
