use std::sync::OnceLock;

use config::{Config, ConfigError};
use miette::Diagnostic;
use schemars::JsonSchema;
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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Settings {
    #[serde(default)]
    pub inputs: InputsConfig,
    #[serde(default)]
    pub outputs: OutputsConfig,
}

#[derive(Debug, Deserialize, JsonSchema, SmartDefault)]
#[serde(default)]
pub struct InputsConfig {
    #[default(vec!["c".into(), "cpp".into()])]
    /// The extensions which are allowed to be transformed
    pub extensions: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OutputKind {
    CHeader,
}

#[derive(Debug, Deserialize, JsonSchema, SmartDefault)]
#[serde(default)]
pub struct OutputsConfig {
    #[default(vec![OutputKind::CHeader])]
    /// The outputs to enable
    pub enable: Vec<OutputKind>,
    pub c_header: crate::backends::c_header::Config,
}

static SETTINGS: OnceLock<Settings> = OnceLock::new();

impl Settings {
    pub fn global() -> &'static Self {
        SETTINGS.get().unwrap()
    }
}

pub fn load() -> Result<(), Error> {
    let settings = Config::builder()
        .add_source(config::File::with_name("cgen").required(false))
        .build()?;

    match settings.try_deserialize() {
        Err(error) => Err(Error { source: error }),
        Ok(settings) => {
            SETTINGS.set(settings).unwrap();

            Ok(())
        }
    }
}
