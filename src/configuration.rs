// ./src/config.rs

///////////////////////////////////////////////////////////////////////////////
/// CONFIGURATION
/// Get configuration for external toml file
/// https://github.com/mehcode/config-rs

#[derive(serde::Deserialize)]
pub struct Settings {
    pub api_port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.toml",
            config::FileFormat::Toml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
