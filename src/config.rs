use serde::Deserialize;
use anyhow::Result;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    mqtt: MqttConfig,
    #[serde(default = "Config::default_log_level")]
    log_level: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MqttConfig {
    #[serde(default = "MqttConfig::default_host")]
    host: String,
    #[serde(default = "MqttConfig::default_port")]
    port: u16,
    username: Option<String>,
    password: Option<String>,
}

impl Config {
    fn default_log_level() -> String {
        "info".to_string()
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        let config = toml::from_str(&config_str)?;

        Ok(config)
    }

    pub fn mqtt(&self) -> &MqttConfig {
        &self.mqtt
    }

    pub fn log_level(&self) -> &str {
        &self.log_level
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
