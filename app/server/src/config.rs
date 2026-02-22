use std::env;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
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

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let port = env::var("PORT")
            .map_err(|_| ConfigError::Missing("PORT".into()))?
            .parse::<u16>()
            .map_err(|e| ConfigError::Invalid {
                key: "PORT".into(),
                reason: e.to_string(),
            })?;

        Ok(Self { port })
    }
}
