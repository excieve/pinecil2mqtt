use std::path::PathBuf;
use serde::Deserialize;
use anyhow::Result;
use config::{Config, ConfigError, Environment, File};

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Settings {
    mqtt: MqttConfig,
    #[serde(default = "Settings::default_log_level")]
    loglevel: String
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct MqttConfig {
    #[serde(default = "MqttConfig::default_host")]
    host: String,
    #[serde(default = "MqttConfig::default_port")]
    port: u16,
    username: Option<String>,
    password: Option<String>,
}

impl Settings {
    pub fn new(file_path: PathBuf) -> Result<Self, ConfigError> {
        let cfg = Config::builder()
            .add_source(File::from(file_path).required(false))
            .add_source(Environment::with_prefix("P2M").separator("_"))
            .build()?;

        cfg.try_deserialize()
    }

    fn default_log_level() -> String {
        "info".to_string()
    }

    pub fn mqtt(&self) -> &MqttConfig {
        &self.mqtt
    }

    pub fn log_level(&self) -> &str {
        &self.loglevel
    }

    pub fn set_log_level(&mut self, level: String) {
        self.loglevel = level;
    }
}

impl MqttConfig {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    pub fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }

    fn default_port() -> u16 {
        1883
    }

    fn default_host() -> String {
        "localhost".to_string()
    }
}
