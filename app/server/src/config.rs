use std::env;

use url::Url;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub log_level: LogLevel,
    pub log_format: LogFormat,
    pub cors_allowed_origins: Vec<String>,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in_secs: u64,
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

        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .map(|s| {
                parse_origin(&s).map_err(|reason| ConfigError::Invalid {
                    key: "CORS_ALLOWED_ORIGINS".into(),
                    reason,
                })?;
                Ok(s)
            })
            .collect::<Result<Vec<_>, ConfigError>>()?;

        let database_url =
            env::var("DATABASE_URL").map_err(|_| ConfigError::Missing("DATABASE_URL".into()))?;

        let jwt_secret =
            env::var("JWT_SECRET").map_err(|_| ConfigError::Missing("JWT_SECRET".into()))?;

        let jwt_expires_in_secs = env::var("JWT_EXPIRES_IN_SECS")
            .unwrap_or_else(|_| "3600".to_string())
            .parse::<u64>()
            .map_err(|e| ConfigError::Invalid {
                key: "JWT_EXPIRES_IN_SECS".into(),
                reason: e.to_string(),
            })?;

        Ok(Self {
            port,
            log_level,
            log_format,
            cors_allowed_origins,
            database_url,
            jwt_secret,
            jwt_expires_in_secs,
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

fn parse_origin(s: &str) -> Result<(), String> {
    let url = Url::parse(s).map_err(|e| format!("'{s}' is not a valid URL: {e}"))?;

    if !matches!(url.scheme(), "http" | "https") {
        return Err(format!("'{s}': scheme must be http or https"));
    }

    if url.host().is_none() {
        return Err(format!("'{s}': missing host"));
    }

    // An origin must not include a path, query, or fragment
    if url.path() != "/" {
        return Err(format!("'{s}': must not contain a path"));
    }
    if url.query().is_some() {
        return Err(format!("'{s}': must not contain a query string"));
    }
    if url.fragment().is_some() {
        return Err(format!("'{s}': must not contain a fragment"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_origin_valid() {
        assert!(parse_origin("http://localhost:3000").is_ok());
        assert!(parse_origin("https://example.com").is_ok());
        assert!(parse_origin("http://localhost").is_ok());
        assert!(parse_origin("https://sub.example.com:8443").is_ok());
    }

    #[test]
    fn test_parse_origin_invalid_scheme() {
        assert!(parse_origin("ftp://example.com").is_err());
        assert!(parse_origin("ws://example.com").is_err());
    }

    #[test]
    fn test_parse_origin_not_a_url() {
        assert!(parse_origin("not-a-url").is_err());
        assert!(parse_origin("").is_err());
    }

    #[test]
    fn test_parse_origin_missing_host() {
        assert!(parse_origin("http://").is_err());
        assert!(parse_origin("https://").is_err());
    }

    #[test]
    fn test_parse_origin_with_path() {
        assert!(parse_origin("http://example.com/path").is_err());
        assert!(parse_origin("https://example.com/api/v1").is_err());
    }

    #[test]
    fn test_parse_origin_with_query() {
        assert!(parse_origin("http://example.com?foo=bar").is_err());
    }

    #[test]
    fn test_parse_origin_with_fragment() {
        assert!(parse_origin("http://example.com#section").is_err());
    }
}
