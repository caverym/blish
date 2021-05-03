#[macro_use]

pub struct Error {
    kind: ErrorKind,
    error: String,
}

pub enum ErrorKind {
    CmdNotFound,
    CmdFailed,
    PipeFailed,
    Other,
}

#[macro_export]
macro_rules! errorhere {
    ($e:expr) => {
        Err(crate::error::Error::new(crate::error::ErrorKind::Other, $e))
    };

    ($ek:expr, $e:expr) => {
        Err(crate::error::Error::new($ek, $e))
    };
}

impl Error {
    pub fn new<T: ToString>(kind: ErrorKind, error: T) -> Error {
        Error {
            kind,
            error: error.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.error)
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ErrorKind::CmdFailed => write!(f, "Command Failure"),
            ErrorKind::CmdNotFound => write!(f, "Command Not Found"),
            ErrorKind::PipeFailed => write!(f, "Pipe Failure"),
            ErrorKind::Other => write!(f, "Other"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        let kind: ErrorKind;
        let error: String;

        kind = match e.kind() {
            std::io::ErrorKind::NotFound => ErrorKind::CmdNotFound,
            std::io::ErrorKind::BrokenPipe => ErrorKind::PipeFailed,
            _ => ErrorKind::Other,
        };

        error = e.to_string();

        Error { kind, error }
    }
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        let kind = match e.kind {
            ErrorKind::CmdNotFound => std::io::ErrorKind::NotFound,
            ErrorKind::PipeFailed => std::io::ErrorKind::BrokenPipe,
            _ => std::io::ErrorKind::Other,
        };
        std::io::Error::new(kind, e.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        let str = format!("{}", e);
        Error {
            kind: ErrorKind::Other,
            error: str,
        }
    }
}
