use crate::{error::{Error, ErrorKind}, errorhere};

#[derive(Debug)]
pub struct Function {
    func: IntFunc,
    args: Vec<String>,
}

#[derive(Debug)]
enum IntFunc {
    Cd,
	Exit,
}

impl Function {
    pub fn new(cmd: &str, args: &[&str]) -> Result<Function, Error> {
        let func = IntFunc::new(cmd)?;

        Ok(Function {
            func,
            args: args.iter().map(|x| x.to_string()).collect(),
        })
    }

    pub fn run(&self) -> Result<(), Error> {
        self.func.run(&self.args)
    }
}

impl IntFunc {
    pub fn new(cmd: &str) -> Result<IntFunc, Error> {
        match cmd.to_ascii_lowercase().as_str() {
            "cd" => Ok(IntFunc::Cd),
			"exit" => Ok(IntFunc::Exit),
            _ => errorhere!("Not an internal command"),
        }
    }

	pub fn run(&self, args: &Vec<String>) -> Result<(), Error> {
		match self {
		    IntFunc::Cd => self.cd(args),
			IntFunc::Exit => self.exit(),
			_ => errorhere!(ErrorKind::CmdNotFound, self),
		}
	}

	fn cd(&self, args: &Vec<String>) -> Result<(), Error> {
		Ok(std::env::set_current_dir(&args[0])?)
	}

	fn exit<T>(&self) -> T {
		std::process::exit(0);
	}
}

impl std::fmt::Display for IntFunc {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
		    IntFunc::Cd => write!(f, "cd"),
			_ => write!(f, ""),
		}
	}
}
