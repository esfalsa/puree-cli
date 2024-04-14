use std::{fs::File, io::Write, path::PathBuf};

use puree_cli::{config::Config, DumpReader};

use anyhow::{bail, Context, Result};
use clap::Parser;

/// Command-line tool to find tagged regions
#[derive(Parser, Debug)]
struct Args {
    /// Path to regions daily dump to search
    dump: PathBuf,

    /// File to write output to
    #[clap(short, long)]
    output: Option<PathBuf>,
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

    let output_writer: Box<dyn Write> = match args.output {
        Some(ref path) => File::create_new(path)
            .map(|f| Box::new(f) as Box<dyn Write>)
            .context("output file already exists")?,
        None => Box::new(std::io::stdout()),
    };

    let mut csv_writer = csv::Writer::from_writer(output_writer);

    while let Some(Ok(region)) = regions_iter.next() {
        if let Some(outcome) = config.matches(&region) {
            csv_writer.serialize(outcome)?;
        }
    }

    Ok(())
}
