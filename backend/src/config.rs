use config::ConfigError;

#[derive(Debug, Default, serde::Deserialize)]
pub struct ServerConfig {
    pub server_host: String,
    pub server_port: u16,
    pub pg: deadpool_postgres::Config,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}
