use std::path::Path;

use anyhow::Result;
use regex::Regex;
use serde::Deserialize;

use crate::models::{EmbassyStatus, Region};

#[derive(Deserialize)]
pub struct Config {
    pub exclude: ExcludeConfig,
    pub include: IncludeConfig,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&config)?)
    }
}

const fn default_bool<const V: bool>() -> bool {
    V
}

#[derive(Deserialize)]
pub struct ExcludeConfig {
    pub name: Vec<String>,
    pub embassy: Vec<String>,
    #[serde(default = "default_bool::<true>")]
    pub existing_delegates: bool,
    #[serde(default = "default_bool::<true>")]
    pub nonexecutive_delegates: bool,
}

impl ExcludeConfig {
    pub fn matches(&self, region: &Region) -> bool {
        if self.existing_delegates && region.delegate().is_some() {
            return true;
        }
        if self.nonexecutive_delegates && !region.delegate_auth().executive() {
            return true;
        }
        if self
            .name
            .iter()
            .any(|n| n.to_lowercase() == region.name().to_lowercase())
        {
            return true;
        }
        if self.embassy.iter().any(|cfg_emb| {
            region.embassies().iter().any(|emb| {
                !matches!(
                    emb.status(),
                    &EmbassyStatus::Closing | &EmbassyStatus::Rejected | &EmbassyStatus::Denied
                ) && emb.region().to_lowercase() == cfg_emb.to_lowercase()
            })
        }) {
            return true;
        }

        false
    }
}

#[derive(Deserialize)]
pub struct IncludeConfig {
    pub factbook: Vec<FactbookConfig>,
    pub office: Vec<OfficeConfig>,
    pub appointer: Vec<AppointerConfig>,
    pub embassy: Vec<EmbassyConfig>,
}

#[derive(Deserialize)]
pub struct FactbookConfig {
    pub contains: Vec<String>,
    pub org: Option<String>,
}

impl FactbookConfig {
    pub fn matches(&self, region: &Region) -> bool {
        self.contains.iter().any(|s| region.factbook().contains(s))
    }
}

#[derive(Deserialize)]
pub struct OfficeConfig {
    pub equals: Vec<String>,
    pub org: Option<String>,
}

impl OfficeConfig {
    pub fn matches(&self, region: &Region) -> bool {
        for office in region.officers().iter().map(|o| o.office().to_lowercase()) {
            if self.equals.iter().any(|e| e.to_lowercase() == office) {
                return true;
            }
        }

        false
    }
}

#[derive(Deserialize)]
pub struct AppointerConfig {
    #[serde(with = "serde_regex")]
    regex: Vec<Regex>,
}

impl AppointerConfig {
    pub fn matches(&self, region: &Region) -> bool {
        for appointer in region
            .officers()
            .iter()
            .map(|o| o.appointer().to_lowercase())
        {
            if self.regex.iter().any(|r| r.is_match(&appointer)) {
                return true;
            }
        }

        false
    }
}

#[derive(Deserialize)]
pub struct EmbassyConfig {
    equals: Vec<String>,
    pub org: Option<String>,
}

impl EmbassyConfig {
    pub fn matches(&self, region: &Region) -> bool {
        for embassy in region
            .embassies()
            .iter()
            .filter(|e| {
                !matches!(
                    e.status(),
                    &EmbassyStatus::Closing | &EmbassyStatus::Rejected | &EmbassyStatus::Denied
                )
            })
            .map(|e| e.region().to_lowercase())
        {
            if self.equals.iter().any(|e| e.to_lowercase() == embassy) {
                return true;
            }
        }

        false
    }
}
