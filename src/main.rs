use std::{collections::HashSet, fs::File, path::PathBuf};

use puree_cli::{config::Config, models::EmbassyStatus, DumpReader};

use anyhow::Result;
use clap::Parser;

/// Command-line tool to find tagged regions
#[derive(Parser, Debug)]
struct Args {
    /// Path to regions daily dump to search
    dump: PathBuf,
}

// const factbook_criteria

fn main() -> Result<()> {
    let args = Args::parse();

    let dump = File::open(args.dump)?;

    let mut regions_iter = DumpReader::new(dump).regions();

    let config = Config::load("default.toml")?;

    while let Some(Ok(region)) = regions_iter.next() {
        if config.exclude.existing_delegates && region.delegate().is_some() {
            continue;
        }

        if config.exclude.nonexecutive_delegates && !region.delegate_auth().executive() {
            continue;
        }

        if config
            .exclude
            .name
            .iter()
            .any(|n| n.to_lowercase() == region.name().to_lowercase())
        {
            continue;
        }

        if config.exclude.embassy.iter().any(|e| {
            region.embassies().iter().any(|emb| {
                emb.status() != &EmbassyStatus::Closing
                    && emb.status() != &EmbassyStatus::Rejected
                    && emb.status() != &EmbassyStatus::Denied
                    && emb.region().to_lowercase() == e.to_lowercase()
            })
        }) {
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
