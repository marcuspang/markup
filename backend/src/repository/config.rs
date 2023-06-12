use config::ConfigError;

#[derive(Debug, Default, serde::Deserialize)]
pub struct ServerConfig {
    pub SERVER_HOST: String,
    pub SERVER_PORT: u16,
    pub DATABASE_URL: String,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
