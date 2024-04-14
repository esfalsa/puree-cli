use std::{collections::HashSet, fs::File, path::PathBuf};

use puree_cli::{config::Config, DumpReader};

use anyhow::{bail, Result};
use clap::Parser;

/// Command-line tool to find tagged regions
#[derive(Parser, Debug)]
struct Args {
    /// Path to regions daily dump to search
    dump: PathBuf,
}

const DEFAULT_CONFIG: &str = include_str!("default.toml");

fn main() -> Result<()> {
    let args = Args::parse();

    let dump = File::open(args.dump)?;

    let mut regions_iter = DumpReader::new(dump).regions();

    let Some(project_dirs) = directories::ProjectDirs::from("", "esfalsa", "puree-cli") else {
        bail!("could not determine configuration directory")
    };

    std::fs::create_dir_all(project_dirs.config_dir())?;

    let config_path = project_dirs.config_dir().join("config.toml");

    if !config_path.exists() {
        std::fs::write(&config_path, DEFAULT_CONFIG)?;
    };

    let config = Config::load(config_path)?;

    while let Some(Ok(region)) = regions_iter.next() {
        if config.exclude.matches(&region) {
            continue;
        }

        let mut orgs = HashSet::new();

        let mut factbook = false;
        let mut ros = false;
        let mut embassies = false;

        for factbook_config in &config.include.factbook {
            if factbook_config.matches(&region) {
                factbook = true;
                if let Some(org) = &factbook_config.org {
                    orgs.insert(org);
                }
            }
        }

        for office_config in &config.include.office {
            if office_config.matches(&region) {
                ros = true;
                if let Some(org) = &office_config.org {
                    orgs.insert(org);
                }
            }
        }

        for appointer_config in &config.include.appointer {
            if appointer_config.matches(&region) {
                ros = true;
                // if let Some(org) = &appointer_config.org {
                //     orgs.insert(org);
                // }
            }
        }

        for embassy_config in &config.include.embassy {
            if embassy_config.matches(&region) {
                embassies = true;
                if let Some(org) = &embassy_config.org {
                    orgs.insert(org);
                }
            }
        }

        if factbook || ros || embassies {
            println!(
                "{}: factbook={}, ros={}, embassies={}, orgs={:?}",
                region.name(),
                factbook,
                ros,
                embassies,
                orgs
            );
        }
    }

    Ok(())
}
