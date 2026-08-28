use config::{Config, ConfigError};
use miette::Diagnostic;
use serde::Deserialize;
use smart_default::SmartDefault;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Failed to load config file")]
#[diagnostic(code(cgen::cfg::config_file_error))]
pub struct Error {
    #[from]
    pub source: ConfigError,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub inputs: InputsConfig,
}

#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct InputsConfig {
    #[default(vec!["c".into(), "cpp".into()])]
    pub extensions: Vec<String>,
}

pub fn load() -> Result<Settings, Error> {
    let settings = Config::builder()
        .add_source(config::File::with_name("cgen").required(false))
        .build()?;

    match settings.try_deserialize() {
        Err(error) => Err(Error { source: error }),
        Ok(settings) => Ok(settings),
    }
}
