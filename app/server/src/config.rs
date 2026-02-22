use std::env;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub log_level: LogLevel,
    pub log_format: LogFormat,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let port = env::var("PORT")
            .map_err(|_| ConfigError::Missing("PORT".into()))?
            .parse::<u16>()
            .map_err(|e| ConfigError::Invalid {
                key: "PORT".into(),
                reason: e.to_string(),
            })?;

        let log_level = env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string())
            .parse::<LogLevel>()
            .map_err(|e| ConfigError::Invalid {
                key: "LOG_LEVEL".into(),
                reason: e,
            })?;

        let log_format = env::var("LOG_FORMAT")
            .unwrap_or_else(|_| "json".to_string())
            .parse::<LogFormat>()
            .map_err(|e| ConfigError::Invalid {
                key: "LOG_FORMAT".into(),
                reason: e,
            })?;

        Ok(Self {
            port,
            log_level,
            log_format,
        })
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Missing(String),
    Invalid { key: String, reason: String },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Missing(key) => write!(f, "missing required env var: {key}"),
            ConfigError::Invalid { key, reason } => {
                write!(f, "invalid value for env var {key}: {reason}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(format!(
                "unknown log level '{s}', expected one of: trace, debug, info, warn, error"
            )),
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
pub enum LogFormat {
    Json,
    Text,
}

impl std::str::FromStr for LogFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(LogFormat::Json),
            "text" => Ok(LogFormat::Text),
            _ => Err(format!(
                "unknown log format '{s}', expected one of: json, text"
            )),
        }
    }
}
