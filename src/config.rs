//! Environment-based configuration with secret-safe error reporting.

use std::env;
use std::path::PathBuf;

/// Validated settings required before the Discord and storage adapters start.
pub struct Config {
    discord_token: String,
    default_prefix: String,
    database_path: PathBuf,
}

impl Config {
    /// Load configuration from the current process environment.
    pub fn from_environment() -> Result<Self, ConfigError> {
        Self::from_lookup(|key| env::var(key).ok())
    }

    fn from_lookup(
        mut lookup: impl FnMut(&str) -> Option<String>,
    ) -> Result<Self, ConfigError> {
        let discord_token = lookup("DISCORD_TOKEN")
            .filter(|value| !value.trim().is_empty())
            .map(|value| value.trim().to_owned())
            .ok_or(ConfigError(
                "DISCORD_TOKEN must be set to a non-empty value",
            ))?;

        let default_prefix = lookup("DEFAULT_PREFIX").unwrap_or_else(|| ".".to_owned());
        if default_prefix.trim().is_empty()
            || default_prefix.chars().count() > 8
            || default_prefix.chars().any(char::is_whitespace)
        {
            return Err(ConfigError(
                "DEFAULT_PREFIX must contain 1 to 8 non-whitespace characters",
            ));
        }

        let database_path =
            lookup("DATABASE_PATH").unwrap_or_else(|| "data/moonbyte.sqlite3".to_owned());
        if database_path.trim().is_empty() {
            return Err(ConfigError(
                "DATABASE_PATH must be a non-empty filesystem path",
            ));
        }

        Ok(Self {
            discord_token,
            default_prefix,
            database_path: PathBuf::from(database_path.trim()),
        })
    }

    /// Return the token for the future Discord adapter. Never log or format it.
    pub fn discord_token(&self) -> &str {
        &self.discord_token
    }

    pub fn default_prefix(&self) -> &str {
        &self.default_prefix
    }

    pub fn database_path(&self) -> &std::path::Path {
        &self.database_path
    }
}

/// A configuration error that names the invalid setting but never its value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigError(&'static str);

impl std::fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn load(values: &[(&str, &str)]) -> Result<Config, ConfigError> {
        Config::from_lookup(|key| {
            values
                .iter()
                .find(|(name, _)| *name == key)
                .map(|(_, value)| (*value).to_owned())
        })
    }

    #[test]
    fn requires_a_non_empty_discord_token() {
        assert_eq!(
            load(&[]).err(),
            Some(ConfigError(
                "DISCORD_TOKEN must be set to a non-empty value"
            ))
        );
        assert!(load(&[("DISCORD_TOKEN", "  ")]).is_err());
    }

    #[test]
    fn applies_documented_defaults() {
        let config = load(&[("DISCORD_TOKEN", "test-token")]).expect("valid config");

        assert_eq!(config.default_prefix(), ".");
        assert_eq!(
            config.database_path(),
            std::path::Path::new("data/moonbyte.sqlite3")
        );
    }

    #[test]
    fn accepts_explicit_prefix_and_database_path() {
        let config = load(&[
            ("DISCORD_TOKEN", "test-token"),
            ("DEFAULT_PREFIX", "mb!"),
            ("DATABASE_PATH", "./state/moonbyte.db"),
        ])
        .expect("valid config");

        assert_eq!(config.default_prefix(), "mb!");
        assert_eq!(
            config.database_path(),
            std::path::Path::new("./state/moonbyte.db")
        );
    }

    #[test]
    fn rejects_invalid_prefixes_and_empty_database_paths() {
        assert!(load(&[
            ("DISCORD_TOKEN", "test-token"),
            ("DEFAULT_PREFIX", "prefix-too-long"),
        ])
        .is_err());
        assert!(load(&[
            ("DISCORD_TOKEN", "test-token"),
            ("DEFAULT_PREFIX", "bad prefix"),
        ])
        .is_err());
        assert!(load(&[
            ("DISCORD_TOKEN", "test-token"),
            ("DATABASE_PATH", "  "),
        ])
        .is_err());
    }

    #[test]
    fn configuration_errors_do_not_include_secret_values() {
        let error = load(&[("DISCORD_TOKEN", " "), ("DEFAULT_PREFIX", "bad prefix")])
            .err()
            .expect("invalid configuration should return an error");
        assert!(!error.to_string().contains("test-token"));
    }
}
