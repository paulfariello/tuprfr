use crate::domain::model::SubmissionMode;
use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_max_connections: u32,
    pub ssl_mode: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub submission_mode: SubmissionMode,
}

impl AppConfig {
    /// # Errors
    ///
    /// Returns an error if `database.url` is absent or if the config is malformed.
    pub fn load() -> Result<Self, config::ConfigError> {
        let path = std::env::var("TUPRFR_CONFIG").unwrap_or_else(|_| "tuprfr.toml".to_string());
        Self::load_from(&path)
    }

    fn load_from(path: &str) -> Result<Self, config::ConfigError> {
        Config::builder()
            .set_default("database.pool_max_connections", 5u32)?
            .set_default("database.ssl_mode", "prefer")?
            .set_default("server.bind_address", "0.0.0.0:3000")?
            .set_default("server.submission_mode", "moderated")?
            .add_source(File::with_name(path).required(false))
            .add_source(
                Environment::with_prefix("TUPRFR")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?
            .try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    fn write_temp_toml(name: &str, content: &str) -> String {
        let path = std::env::temp_dir()
            .join(format!("tuprfr_test_{name}.toml"))
            .to_string_lossy()
            .into_owned();
        std::fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn missing_database_url_is_an_error() {
        let result = AppConfig::load_from("/nonexistent/tuprfr.toml");
        assert!(result.is_err());
    }

    #[test]
    fn defaults_apply_when_only_url_is_set() {
        let path = write_temp_toml(
            "defaults",
            "[database]\nurl = \"postgresql://test@localhost/test\"\n",
        );
        let config = AppConfig::load_from(&path).unwrap();
        assert_eq!(config.database.pool_max_connections, 5);
        assert_eq!(config.database.ssl_mode, "prefer");
        assert_eq!(config.server.bind_address, "0.0.0.0:3000");
        assert_eq!(config.server.submission_mode, SubmissionMode::Moderated);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn file_values_override_defaults() {
        let path = write_temp_toml(
            "file_override",
            "[database]\nurl = \"postgresql://test@localhost/test\"\npool_max_connections = 10\n\n[server]\nbind_address = \"127.0.0.1:8080\"\n",
        );
        let config = AppConfig::load_from(&path).unwrap();
        assert_eq!(config.database.pool_max_connections, 10);
        assert_eq!(config.server.bind_address, "127.0.0.1:8080");
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn env_var_overrides_file() {
        let _guard = ENV_MUTEX.lock().unwrap();
        let path = write_temp_toml(
            "env_override",
            "[database]\nurl = \"postgresql://test@localhost/test\"\n\n[server]\nbind_address = \"0.0.0.0:3000\"\n",
        );
        std::env::set_var("TUPRFR_SERVER__BIND_ADDRESS", "127.0.0.1:9999");
        let config = AppConfig::load_from(&path).unwrap();
        std::env::remove_var("TUPRFR_SERVER__BIND_ADDRESS");
        std::fs::remove_file(&path).ok();
        assert_eq!(config.server.bind_address, "127.0.0.1:9999");
    }
}
