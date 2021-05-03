use crate::command::Command;
use crate::config::Config;
use crate::{error::*, path::Path};

use std::io::{stdin, Read};

#[derive(Debug)]
pub struct Bunny {
    config: Config,
    last_command: Option<Command>,
    pub path: Path,
}

impl Bunny {
    pub async fn new(config: Config, mut path: Path) -> Bunny {
        path = path.add(&config);
        Bunny {
            config,
            last_command: None,
            path,
        }
    }

    pub async fn run(mut self) -> Result<(), Error> {
		self.config.init(&self).await?;
        loop {
            if let Err(e) = self.inner().await {
                eprintln!("{}", e);
            }
        }
        Ok(())
    }

    async fn inner(&mut self) -> Result<(), Error> {
        let mut stdin = stdin();
        loop {
            let mut buf = String::new();
            stdin.read_line(&mut buf)?;
            let cmd = Command::new(buf.trim(), &self.path).await?;
            self.last_command = Some(cmd.exec().await?);
        }
    }
}
