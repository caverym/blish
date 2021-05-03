use crate::path::Path;
use crate::{error::Error, functions::Function};

use std::process::{Child, Command as stdCommand};

#[derive(Debug)]
pub struct Command(CommandKind);

#[derive(Debug)]
pub enum CommandKind {
    Internal(Function),
    External(CmdAspects),
}

#[derive(Debug)]
pub struct CmdAspects {
    bin: String,
    args: Vec<String>,
    wait: bool,
}

impl Command {
    pub async fn new(input: &str, path: &Path) -> Result<Command, Error> {
        let mut args: Vec<&str> = input.split(' ').collect();
        let cmd = args.remove(0);
        if let Ok(c) = Function::new(cmd, &args) {
            return Ok(Command(CommandKind::Internal(c)));
        }

        let bin = path.find_command(cmd)?;

        Ok(Command(CommandKind::External(CmdAspects {
            bin,
            args: args.iter().map(|x| x.to_string()).collect(),
            wait: true,
        })))
    }

    pub async fn exec(self) -> Result<Command, Error> {
        match &self.0 {
            CommandKind::Internal(x) => x.run()?,
            CommandKind::External(x) => x.run()?,
        }

        Ok(self)
    }
}

impl CmdAspects {
    pub fn new(bin: String, args: Vec<String>, wait: bool) -> CmdAspects {
        CmdAspects {
            bin,
            wait,
            args: args.iter().map(|x| x.to_string()).collect(),
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        let mut child = stdCommand::new(&self.bin).args(&self.args).spawn()?;
        if self.wait {
            child.wait()?;
        }
        Ok(())
    }
}
