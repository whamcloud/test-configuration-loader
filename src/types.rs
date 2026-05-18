use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug)]
pub struct ParseLogLevelError(String);

impl Display for ParseLogLevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseLogLevelError {}

impl FromStr for LogLevel {
    type Err = ParseLogLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "error" | "err" => Ok(LogLevel::Error),
            "warn" | "warning" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            other => Err(ParseLogLevelError(format!("unknown log level '{}'", other))),
        }
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}
