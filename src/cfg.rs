use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use miette::Diagnostic;
use schemars::JsonSchema;
use serde::Deserialize;
use smart_default::SmartDefault;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Failed to load config file")]
pub enum Error {
    #[error("Config file not found")]
    #[diagnostic(code(cgen::cfg::not_found))]
    NotFound,
    #[error("Config file is in invalid format")]
    #[diagnostic(code(cgen::cfg::invalid_format))]
    InvalidFormat,
    #[error("Config file path has an invalid extension")]
    #[diagnostic(
        code(cgen::cfg::invalid_extension),
        help = "Change the config file extension to be json, yaml, or toml."
    )]
    InvalidExtension,
    #[error(transparent)]
    #[diagnostic(code(cgen::cfg::io_error))]
    Io(#[from] std::io::Error),
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
    pub clang: crate::frontends::ClangConfig,
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
    pub c_header: crate::backends::CHeaderConfig,
}

static SETTINGS: OnceLock<Settings> = OnceLock::new();

impl Settings {
    pub fn global() -> &'static Self {
        SETTINGS.get().unwrap()
    }
}

struct ConfigFile {
    content: String,
    format: ConfigFormat,
}

enum ConfigFormat {
    Json,
    Yaml,
    Toml,
}

fn search_upwards<'a>(
    dir: &Path,
    files: impl IntoIterator<Item = &'a str> + Clone,
) -> Option<(PathBuf, String)> {
    for file in files.clone() {
        let path = dir.join(file);

        if let Ok(content) = fs::read_to_string(&path) {
            return Some((path, content));
        }
    }

    search_upwards(dir.parent()?, files)
}

fn format_from_extension(extension: &str) -> Option<ConfigFormat> {
    match extension {
        "json" => Some(ConfigFormat::Json),
        "yml" | "yaml" => Some(ConfigFormat::Yaml),
        "toml" => Some(ConfigFormat::Toml),
        _ => None,
    }
}

fn search_for_configs<'a>(
    configs: impl IntoIterator<Item = &'a str> + Clone,
) -> Option<ConfigFile> {
    let (path, content) = search_upwards(&env::current_dir().ok()?, configs)?;

    let extension = path.extension()?.to_str()?;

    Some(ConfigFile {
        content,
        format: format_from_extension(extension)?,
    })
}

fn parse_config_file(file: &ConfigFile) -> Result<(), Error> {
    let settings = match file.format {
        ConfigFormat::Json => serde_json::from_str::<Settings>(&file.content).ok(),
        ConfigFormat::Yaml => noyalib::from_str::<Settings>(&file.content).ok(),
        ConfigFormat::Toml => toml::from_str::<Settings>(&file.content).ok(),
    }
    .ok_or(Error::InvalidFormat)?;

    SETTINGS.set(settings).unwrap();

    Ok(())
}

pub fn load() -> Result<(), Error> {
    if let Some(path) = &crate::cli::Args::global().config {
        let content = fs::read_to_string(path)?;

        let extension = path
            .extension()
            .ok_or(Error::InvalidExtension)?
            .to_str()
            .ok_or(Error::InvalidExtension)?;

        parse_config_file(&ConfigFile {
            content,
            format: format_from_extension(extension).ok_or(Error::InvalidExtension)?,
        })?;

        return Ok(());
    }

    let config_file = search_for_configs(["cgen.json", "cgen.yml", "cgen.yaml", "cgen.toml"])
        .ok_or(Error::NotFound)?;

    parse_config_file(&config_file)?;

    Ok(())
}
