use std::{collections::HashSet, path::Path};

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::models::{EmbassyStatus, Region};

#[derive(Deserialize)]
pub struct Config {
    pub exclude: ExcludeConfig,
    pub include: IncludeConfig,
}

#[derive(Serialize)]
pub struct RegionMatchOutcome {
    name: String,
    wfe: bool,
    ro: bool,
    embassies: bool,
    orgs: String,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&config)?)
    }

    pub fn matches(&self, region: &Region) -> Option<RegionMatchOutcome> {
        if self.exclude.matches(region) {
            return None;
        }

        let mut orgs = HashSet::new();

        let mut factbook = false;
        let mut ros = false;
        let mut embassies = false;

        for factbook_config in &self.include.factbook {
            if factbook_config.matches(&region) {
                factbook = true;
                if let Some(org) = &factbook_config.org {
                    orgs.insert(org);
                }
            }
        }

        for office_config in &self.include.office {
            if office_config.matches(&region) {
                ros = true;
                if let Some(org) = &office_config.org {
                    orgs.insert(org);
                }
            }
        }

        for appointer_config in &self.include.appointer {
            if appointer_config.matches(&region) {
                ros = true;
            }
        }

        for embassy_config in &self.include.embassy {
            if embassy_config.matches(&region) {
                embassies = true;
                if let Some(org) = &embassy_config.org {
                    orgs.insert(org);
                }
            }
        }

        if factbook || ros || embassies {
            let name = if matches!(
                region.name().chars().next(),
                Some('=') | Some('+') | Some('-') | Some('@')
            ) {
                format!("'{}", region.name())
            } else {
                region.name().to_string()
            };

            Some(RegionMatchOutcome {
                name,
                wfe: factbook,
                ro: ros,
                embassies,
                orgs: orgs.iter().join(", "),
            })
        } else {
            None
        }
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
