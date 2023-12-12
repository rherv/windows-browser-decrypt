use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ExporterError {
    CannotFindCache(String),
    FailedToReadMasterKey(String),
    IO(String),
}

impl Error for ExporterError {}

impl Display for ExporterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::IO(msg) => format!("{}", msg),
            Self::CannotFindCache(msg) => format!("{}", msg),
            Self::FailedToReadMasterKey(msg) => format!("{}", msg),
        };

        write!(f, "Error: {message}")
    }
}