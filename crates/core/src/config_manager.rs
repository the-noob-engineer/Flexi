#[derive(serde::Deserialize, Debug)]
pub struct DatabaseConfig {
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_port: String,
    pub db_host: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct FlexiConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(serde::Deserialize, Debug)]
pub enum ConfigErrors {
    IOError,
    BadConfigFile(String),
}

pub fn build_config_from_file(file: String) -> Result<FlexiConfig, ConfigErrors> {
    let file_content = std::fs::read_to_string(file).map_err(|error| {
        log::info!("Error: {:?}", error);
        ConfigErrors::IOError
    })?;

    let config: FlexiConfig = toml::from_str(&file_content)
        .map_err(|error| ConfigErrors::BadConfigFile(error.to_string()))?;

    Ok(config)
}
