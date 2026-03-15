mod defaults;
#[cfg(test)]
mod tests;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default)]
pub struct RawConfig {
    pub servers: Option<HashMap<String, RawServerConfig>>,
    pub detection: Option<RawDetection>,
    pub general: Option<RawGeneral>,
    pub report: Option<RawReport>,
}

#[derive(Debug, Deserialize)]
pub struct RawServerConfig {
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub libraries: Option<HashMap<String, RawLibraryConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct RawLibraryConfig {
    pub force_rating: Option<String>,
    pub locations: Option<HashMap<String, RawLocationConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct RawLocationConfig {
    pub force_rating: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct RawDetection {
    pub r: Option<RawWordList>,
    pub pg13: Option<RawWordList>,
    pub ignore: Option<RawIgnore>,
    pub g_genres: Option<RawGenres>,
}

#[derive(Debug, Deserialize)]
pub struct RawWordList {
    pub stems: Option<Vec<String>>,
    pub exact: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RawIgnore {
    pub false_positives: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RawGenres {
    pub genres: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RawGeneral {
    pub overwrite: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct RawReport {
    pub output_path: Option<String>,
}

pub fn parse_toml(content: &str) -> Result<RawConfig, toml::de::Error> {
    toml::from_str(content)
}
