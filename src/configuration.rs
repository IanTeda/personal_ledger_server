//! ./src/configuration.rs
//!
//! # APPLICATION CONFIGURATION
//!
//! A layered configuration system.
//!
//! Get API configuration for the external `./config/default.yaml` file and
//! overwrite with runtime environment configuration `./config/production.yaml`.
//!
//! #### REFERENCES
//! * [config.rs Repository](https://github.com/mehcode/config-rs)
//! * [Configuration management in Rust web services](https://blog.logrocket.com/configuration-management-in-rust-web-services/)

use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use serde_aux::field_attributes::deserialize_number_from_string;
use strum::{AsRefStr, Display};

/// # DEFAULT_CONFIG_FILE_PATH
///
/// Default configuration file
const DEFAULT_CONFIG_FILE_PATH: &str = "./configuration/default.yaml";
/// # CONFIG_FILE_PREFIX
///
/// Configuration folder to look in for runtime configurations
const CONFIG_FILE_PREFIX: &str = "./configuration/";

/// # ENV
///
/// Server runtime environment
///
/// Strum crate is used to derive Display trait and serialise Display output into
/// snake case
///
/// ## ATTRIBUTES
///
/// * `Development`: Development run time environment
/// * `Testing`: Testing run time environment
/// * `Production`: Production environment
#[derive(Clone, Debug, serde::Deserialize, PartialEq, Copy, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Env {
    // #[strum(serialize = "development")]
    Development,
    // #[strum(serialize = "testing")]
    Testing,
    // #[strum(serialize = "production")]
    Production,
}

/// # LOG LEVELS
///
/// Define log levels available.
///
/// ## ATTRIBUTES
///
/// * `Error`: Error conditions within an application that hinder the execution
///    of a specific operation. The application can continue functioning at a
///    reduced level of functionality or performance
/// * `Warn`: Warn that something unexpected has occurred, but the application
///    can continue to function normally for the time being. It is also used to
///    signify conditions that should be promptly addressed before they escalate
///    into problems for the application.
/// * `Info`: Info captures events in the system that are significant to the
///    application's business purpose. Such events are logged to show that the
///    system is operating normally. Production systems typically default to
///    logging at this level
/// * `Debug`: Debug is used for logging messages that aid developers in
///    identifying issues during a debugging session.
/// * `Trace`: Trace is designed specifically for tracing the path of code execution
///    within a program. It is primarily used to provide a detailed breakdown of
///    the events leading up to a crash, error, or other logged events at higher
///    levels.
///
///  #### REFERENCES
///
/// * [Log Levels Explained and How to Use Them](https://betterstack.com/community/guides/logging/log-levels-explained/)
#[derive(Clone, Debug, serde::Deserialize, Display, AsRefStr, Copy)]
#[strum(serialize_all = "snake_case")]
pub enum LogLevels {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// # FROM TRAIT
///
/// Add logic you might want to enable or disable based on the runtime
/// environment
impl From<&str> for Env {
    fn from(env: &str) -> Self {
        match env {
            "Testing" => Env::Testing,
            "Production" => Env::Production,
            _ => Env::Development,
        }
    }
}

/// # Server
///
/// Server settings
///
/// ## Attributes
///
/// * `port`: Port that the api server will run on
/// * `address`: The API server address without http/https
/// * `env`: Server runtime environment
/// * `log_level`: Log level to use in the application
#[derive(Debug, serde::Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub address: String,
    pub env: Env,
    pub log_level: LogLevels,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Database {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl Database {
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    pub fn without_database_name(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
    pub fn with_database_name(&self) -> PgConnectOptions {
        self.without_database_name().database(&self.database_name)
    }
}

/// # Settings
///
/// Root level configuration
///
/// ## Attributes
///
/// * `server`: Server setting struct
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
}

/// # Settings
///
/// Implementation of the settings trait for the configuration module
impl Settings {
    /// # new
    ///
    /// A new setting constructor
    ///
    /// ## RETURNS
    ///
    /// This function will return a Result<Self, config::ConfigError> since we are
    /// doing some file reading and need to manage errors.
    ///
    /// TODO: Check for a correct run time environment else panic
    /// TODO: Check for a correct log level else panic
    pub fn new() -> Result<Self, config::ConfigError> {
        // If RUN_ENV is not set then default to `Development`
        // TODO: Should we throw an error here instead of defaulting it to "Development"
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());

        let settings = config::Config::builder()
            .set_default("server.env", env.clone())?
            .add_source(config::File::with_name(DEFAULT_CONFIG_FILE_PATH))
            .add_source(config::File::with_name(&format!(
                "{}{}",
                CONFIG_FILE_PREFIX, env
            )))
            .build()
            .unwrap();

        settings.try_deserialize::<Settings>()
    }
}
