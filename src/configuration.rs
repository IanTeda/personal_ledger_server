// -- ./src/configuration.rs

//! Application configuration settings
//!
//! # Application Configuration Crate
//!
//! Get API configuration from the `./configuration/base.yaml` file and
//! overwrite with runtime environment configuration `./config/production.yaml`
//! and environmental runtime variables.
//!
//! # References
//!
//! * [config.rs Repository](https://github.com/mehcode/config-rs)
//! * [Configuration management in Rust web services](https://blog.logrocket.com/configuration-management-in-rust-web-services/)

// #![allow(unused)] // For development only

use crate::prelude::*;

use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use strum::{AsRefStr, Display};
use std::path::PathBuf;

/// Directory from binary base folder to look in for configuration files
const CONFIGURATION_DIRECTORY_PREFIX: &str = "./configuration/";

/// Configuration for the API
#[derive(serde::Deserialize, Clone, Debug)]
pub struct Configuration {
	pub database: DatabaseSettings,
	pub application: ApplicationSettings,
	pub email_client: EmailClientSettings,
}

/// Define log levels the system will recognise
#[derive( serde::Deserialize, Debug, Clone, AsRefStr, Display, Copy)]
pub enum LogLevels {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Configuration for running the API application
#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    // The host address the api should bind to
    pub address: String,
    /// The port that the api should bind to
	#[serde(deserialize_with = "deserialize_number_from_string")]
	pub port: u16,
    /// Application log level has a default set in builder
    pub log_level: LogLevels,
    /// Application runtime environment is set to default in the builder
    pub runtime_environment: Environment
}

/// Configuration for connecting to the database server
#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    /// Database host address
    pub host: String,
    /// Database host port
	#[serde(deserialize_with = "deserialize_number_from_string")]
	pub port: u16,
    /// Database username for login
	pub username: String,
    /// Database password for login
    pub password: Secret<String>,
    /// Database name to use
    pub database_name: String,
    /// Should ssl be used to connect to the database
	pub require_ssl: bool,
}

impl DatabaseSettings {
    // TODO: Should be able to delete without as we are using slqx::test
    pub fn without_database_name(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_database_name(&self) -> PgConnectOptions {
        self.without_database_name().database(&self.database_name)
    }    
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password.expose_secret(), self.host, self.port, self.database_name
        )
    }
}

/// Configuration for connecting to the email provider
#[derive(serde::Deserialize, Clone, Debug)]
pub struct EmailClientSettings {
    /// URL for connecting to the email client
	pub base_url: String,
    /// Sender email address
	pub sender_email: String,
    /// Authorisation token for connecting the email provider
	pub authorisation_token: Secret<String>,
    /// How long should be try to connect to the email provider
	pub timeout_milliseconds: u64,
}

/// The possible runtime environment for our application.
#[derive(Clone, Debug, serde::Deserialize, PartialEq, Copy, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Environment {
	Development,
    Testing,
	Production,
}

impl Environment {
	pub fn as_str(&self) -> &'static str {
		match self {
			Environment::Development => "development",
            Environment::Testing => "testing",
			Environment::Production => "production",
		}
	}
}

impl TryFrom<String> for Environment {
	type Error = String;

	fn try_from(s: String) -> core::result::Result<Self, Self::Error> {
		match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "testing" => Ok(Self::Testing),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `development`, `testing` or `production`.",
                other
            )),
        }
	}
}

/// Returns the runtime environment enum used to start the application
/// 
/// This function parse the runtime environmental variables for "APP_ENVIRONMENT".
/// If the variable is not set, then default to development
pub fn get_runtime_environment() -> Result<Environment> {
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    Ok(environment)
}

impl Configuration {
    /// Parse the application configuration from yaml files, returning a 
    /// `Configuration` result.
    pub fn parse() -> Result<Configuration> {

        // Define the configuration directory within the base application directory
        let base_dir_path: PathBuf = std::env::current_dir()
            .expect("Failed to parse current directory.")
            .join(CONFIGURATION_DIRECTORY_PREFIX);
        // dbg!(base_dir_path);

        let environment_filename = format!(
            "{}.yaml", 
            get_runtime_environment()?.as_str()
        );
        // dbg!(environment_filename);

        // Build our configuration reader
        // 
        // # Setting
        // 
        // Configuration files are added in this order, with subsequent files
        // overwriting previous configurations if present:
        // 
        //  1. `base.yaml` in user configuration folder
        //  2. `runtime_environment.yaml` in user configuration folder
        //  5. `PL__` environment variables
        let configuration = config::Config::builder()
            .set_default(
                "application.runtime_environment", 
                "development"
            )?
            .set_default(
                "application.log_level", 
                "info"
            )?
            .add_source(config::File::from(
                base_dir_path.join("base.yaml"),
            ))
            .add_source(config::File::from(
                base_dir_path.join(environment_filename),
            ))

            // -- Environmental variables
            // Add in settings from environment variables (with a prefix of PL and '__' as separator)
            // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
            .add_source(
                config::Environment::with_prefix("PL")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;

        // Convert the configuration values into Settings type
        Ok(configuration.try_deserialize::<Configuration>()?)
    }
}

//-- Unit Tests
#[cfg(test)]
pub mod tests {

    // Override with more flexible error
    pub type Result<T> = core::result::Result<T, Error>;
	pub type Error = Box<dyn std::error::Error>;

    // Bring module functions into test scope
    use super::*;

    // Test creating a new Thing without a description
    #[test]
    fn default_config() -> Result<()> {
        let configuration: Configuration = Configuration::parse()?;
        // dbg!(_configuration);
        assert!(configuration.application.address.is_empty());
        // assert_eq!(configuration.get("application.address").ok(), "127.0.0.1");
        Ok(())
    }

}
