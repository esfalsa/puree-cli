use std::collections::{HashMap, HashSet};

use crate::models::{Authority, Embassy, Region};

pub struct Matcher {
    name: fn(&str) -> Option<HashSet<&str>>,
    factbook: fn(&str) -> Option<HashSet<&str>>,
}

impl Matcher {
    pub fn new(
        name_fn: fn(&str) -> Option<HashSet<&str>>,
        factbook_fn: fn(&str) -> Option<HashSet<&str>>,
    ) -> Self {
        Self {
            name: name_fn,
            factbook: factbook_fn,
        }
    }

    pub fn check<'a>(&'a self, region: &'a Region) -> Option<HashSet<&str>> {
        let mut matched = false;
        let mut orgs = HashSet::new();

        if let Some(matched_orgs) = (self.name)(region.name()) {
            orgs.extend(matched_orgs);
            matched = true;
        }
        if let Some(matched_orgs) = (self.factbook)(region.factbook()) {
            orgs.extend(matched_orgs);
            matched = true;
        }

        if matched {
            Some(orgs)
        } else {
            None
        }
    }
}

// pub struct MatchConfig<'a> {
//     name: HashMap<&'a str, &'a str>,
// }

// impl MatchConfig {
//     pub fn new(name: Vec<String>) -> Self {
//         Self { name }
//     }

//     pub fn name(&self) -> &[String] {
//         &self.name
//     }
// }

// /// A function that takes a generic input type and returns `Some` with a set of organizations if the input was a match, or `None` if the input was not a match.
// type MatchFn<I> = fn(I) -> Option<HashSet<&'static str>>;

// pub struct Matcher {
//     pub name: Option<MatchFn<&str>>,
//     pub delegate: Option<MatchFn<&str>>,
//     pub delegate_auth: Option<MatchFn<&Authority>>,
//     pub embassy: Option<MatchFn<&Embassy>>,
// }

// impl Matcher {
//     pub fn check(&self, region: &'a Region) -> Option<HashSet<&'static str>> {
//         let mut matched = false;
//         let mut orgs = HashSet::new();

//         if let Some(name_fn) = self.name {
//             if let Some(matched_orgs) = name_fn(region.name()) {
//                 for org in matched_orgs {
//                     orgs.insert(org);
//                 }
//                 matched = true;
//             }
//         }

//         if matched {
//             Some(orgs)
//         } else {
//             None
//         }
//     }
// }

// pub trait Matcher {
//     /// The type of input that the matcher checks for a match.
//     type Input;

//     /// Checks whether a given input matches this matcher's rules.
//     ///
//     /// This function should return `Some(HashSet<String>)` with a set of organizations matched to the input, or `None` if the input did not match this matcher. Note that an empty set is not equivalent to not finding a match; depending on the matcher, it is possible to match some inputs without being able to associate them with any organization.
//     fn check(&self, input: &Self::Input) -> Option<HashSet<String>>;
// }

// pub struct BlocklistMatcher

// pub struct FactbookMatcher(HashMap<&'static str, Option<&'static str>>);

// impl FactbookMatcher {
//     pub fn new(config: HashMap<&'static str, Option<&'static str>>) -> Self {
//         Self(config)
//     }

//     pub fn check(&self, input: &str) -> Option<HashSet<&'static str>> {
//         let mut orgs = HashSet::new();
//         let mut matched = false;

//         for (key, value) in &self.0 {
//             if input.to_lowercase().contains(key) {
//                 if let Some(org) = *value {
//                     orgs.insert(org);
//                 }
//                 matched = true;
//             }
//         }

//         if matched {
//             Some(orgs)
//         } else {
//             None
//         }
//     }
// }
