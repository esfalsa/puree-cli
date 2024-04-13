use std::{collections::HashSet, fs::File, path::PathBuf};

use puree_cli::{matchers::Matcher, DumpReader};

use anyhow::Result;
use clap::Parser;

/// Command-line tool to find tagged regions
#[derive(Parser, Debug)]
struct Args {
    /// Path to regions daily dump to search
    dump: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let dump = File::open(args.dump)?;

    let regions_iter = DumpReader::new(dump).regions();

    let name_fn = |name: &str| {
        if [
            "Suspicious",
            "The Black Hawks",
            "The Brotherhood of Malice",
            "Lily",
            "Osiris",
        ]
        .iter()
        .any(|s| s.to_lowercase() == name.to_lowercase())
        {
            Some(HashSet::new())
        } else {
            None
        }
    };

    let factbook_fn = |wfe: &str| {
        if ["[region]the brotherhood of malice[/region]"]
            .iter()
            .any(|s| wfe.to_lowercase().contains(&s.to_lowercase()))
        {
            Some(HashSet::new())
        } else {
            None
        }
    };

    let matcher = Matcher::new(name_fn, factbook_fn);

    // let match_config = MatchConfig::new(vec![
    //     "The Black Hawks".to_string(),
    //     "The Brotherhood of Malice".to_string(),
    //     "Lily".to_string(),
    //     "Osiris".to_string(),
    // ]);

    // let block_matcher = Matcher {
    //     name: Some(|n| {
    //         if [
    //             "Suspicious",
    //             "The Black Hawks",
    //             "The Brotherhood of Malice",
    //             "Lily",
    //             "Osiris",
    //         ]
    //         .iter()
    //         .any(|s| s.to_lowercase() == n.to_lowercase())
    //         {
    //             Some(HashSet::new())
    //         } else {
    //             None
    //         }
    //     }),
    //     delegate: None,
    //     delegate_auth: None,
    //     embassy: None,
    // };

    // let mut factbook_config = HashMap::new();
    // factbook_config.insert("[region]the brotherhood of malice[/region]", Some("BoM"));
    // factbook_config.insert("[region]the black hawks[/region]", Some("TBH"));
    // factbook_config.insert("[region]valle de arena[/region]", Some("Osiris"));

    // let factbook_matcher = FactbookMatcher::new(factbook_config);

    for region in regions_iter {
        let region = region?;

        // dbg!(matcher.check(&region));

        if let Some(orgs) = matcher.check(&region) {
            println!("Matched region {} with orgs {:?}", region.name(), orgs);
        }

        // if let Some(orgs) = region.check(&match_config) {
        //     println!("Matched region {} with orgs {:?}", region.name(), orgs);
        // }

        // if let Some(orgs) = block_matcher.check(&region) {
        //     println!("Matched region {} with orgs {:?}", region.name(), orgs);
        // }

        // match a {
        //     Some(orgs) => println!("Matched region {} with orgs {:?}", region.name(), orgs),
        //     None => (),
        // }

        // blocklist
        // if [
        //     "Suspicious",
        //     "The Black Hawks",
        //     "The Brotherhood of Malice",
        //     "Lily",
        //     "Osiris",
        // ]
        // .iter()
        // .any(|s| s.to_lowercase() == region.name().to_lowercase())
        // {
        //     continue;
        // }

        // match factbook_matcher.check(region.factbook()) {
        //     Some(orgs) => println!("Matched region {} with orgs {:?}", region.name(), orgs),
        //     None => (),
        // }

        // if ["[region]the brotherhood of malice[/region]"]
        //     .iter()
        //     .any(|s| region.factbook().to_lowercase().contains(&s.to_lowercase()))
        // {
        //     println!("{}", region.name());
        // }
    }

    // for region in regions_iter {
    //     // println!("{:?}", region.unwrap());
    //     let region = region?;

    //     println!("region: {}; pop {}", region.name(), region.population());
    // }

    // println!("{}", args.dump.to_string_lossy());

    Ok(())
}
