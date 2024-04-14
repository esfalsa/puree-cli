use std::{fs::File, path::PathBuf};

use puree_cli::DumpReader;

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

    while let Some(Ok(region)) = regions_iter.next() {
        if [
            "suspicious",
            "the black hawks",
            "the brotherhood of malice",
            "lily",
            "osiris",
        ]
        .contains(&region.name())
            || !region.delegate_auth().executive()
            || region.embassies().iter().any(|e| e.region() == "Antifa")
            || region.delegate().is_some()
        {
            continue;
        }
    }

    Ok(())
}
