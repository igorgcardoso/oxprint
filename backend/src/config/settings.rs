use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
}

impl DatabaseSettings {
    pub fn from_env() -> Self {
        Self {
            url: std::env::var("DATABASE_URL").unwrap_or("sqlite:data/oxprint.db".to_string()),
            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub static_dir: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileSettings {
    pub upload_dir: PathBuf,
    pub max_file_size: u64, // bytes
    pub allowed_extensions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PrinterSettings {
    pub default_baud_rate: u32,
    pub connection_timeout: u64, // seconds
    pub command_timeout: u64, // seconds
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
    pub file: FileSettings,
    pub printer: PrinterSettings,
    pub jwt_secret: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            database: DatabaseSettings::from_env(),
            server: ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 3000,
                static_dir: PathBuf::from("static"),
            },
            file: FileSettings {
                upload_dir: PathBuf::from("uploads"),
                max_file_size: 1024 * 1024 * 100, // 100 MB
                allowed_extensions: vec![
                    "gcode".to_string(),
                    "gco".to_string(),
                    "g".to_string()
                ],
            },
            printer: PrinterSettings {
                default_baud_rate: 115200,
                connection_timeout: 10,
                command_timeout: 5,
            },
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "CHANGE_ME_IN_PRODUCTION".to_string()),
        }
    }
}

impl Settings {
    /// Load settings from config file and environment variables
    /// Environment variables take precedence over config file values
    pub fn load() -> Result<Self, config::ConfigError> {
        let settings = Config::builder()
            .add_source(Config::try_from(&Settings::default())?)
            .add_source(File::with_name("config/default").required(false))
            .add_source(Environment::with_prefix("OXPRINT").separator("__"))
            .build()?
            .try_deserialize::<Self>()?;

        if settings.jwt_secret.is_empty() || settings.jwt_secret == "CHANGE_ME_IN_PRODUCTION" {
            return Err(ConfigError::Message(
                "JWT_SECRET environment variable is required. Generate one with: openssl rand -hex 32".to_string()
            ));
        }

        std::fs::create_dir_all(&settings.file.upload_dir)
            .map_err(|e| ConfigError::Message(format!("Failed to create upload directory: {}", e)))?;
        std::fs::create_dir_all(&settings.server.static_dir)
            .map_err(|e| ConfigError::Message(format!("Failed to create static directory: {}", e)))?;

        Ok(settings)
    }
}
