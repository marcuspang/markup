use config::ConfigError;

#[derive(Debug, Default, serde::Deserialize)]
pub struct ServerConfig {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
