use std::{collections::HashSet, fmt::Display};

use crate::error::{Error, Result};

#[derive(Debug)]
// #[builder(setter(into))]
pub struct Region {
    name: String,
    factbook: String,
    population: i32,

    // #[builder(setter(strip_option), default)]
    delegate: Option<String>,
    delegate_votes: i32,
    delegate_auth: Authority,
    frontier: bool,

    // #[builder(setter(strip_option), default)]
    governor: Option<String>,
    last_major: i64,
    last_minor: i64,

    // #[builder(setter(each(name = "officer", into)), default)]
    officers: Vec<Officer>,

    // #[builder(setter(each(name = "embassy", into)), default)]
    embassies: Vec<Embassy>,
}

impl Region {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn factbook(&self) -> &str {
        &self.factbook
    }
    pub fn population(&self) -> i32 {
        self.population
    }
}

/// A builder for a `Region`.
///
/// Unlike typical builders, the methods of a `RegexBuilder` do not return a reference to the builder itself, and cannot be chained.
#[derive(Default, Debug)]
pub struct RegionBuilder {
    name: Option<String>,
    factbook: Option<String>,
    population: Option<i32>,
    delegate: Option<String>,
    delegate_votes: Option<i32>,
    delegate_auth: Option<Authority>,
    frontier: Option<bool>,
    governor: Option<String>,
    last_major: Option<i64>,
    last_minor: Option<i64>,
    officers: Vec<Officer>,
    embassies: Vec<Embassy>,
}

impl RegionBuilder {
    pub fn name<S: Into<String>>(&mut self, name: S) {
        self.name = Some(name.into());
    }
    pub fn factbook<S: Into<String>>(&mut self, entry: S) {
        self.factbook = Some(entry.into());
    }
    pub fn population<I: Into<i32>>(&mut self, population: I) {
        self.population = Some(population.into());
    }
    pub fn delegate<S: Into<String>>(&mut self, delegate: S) {
        self.delegate = Some(delegate.into());
    }
    pub fn delegate_votes<I: Into<i32>>(&mut self, votes: I) {
        self.delegate_votes = Some(votes.into());
    }
    pub fn delegate_auth<A: Into<Authority>>(&mut self, authority: A) {
        self.delegate_auth = Some(authority.into());
    }
    pub fn frontier(&mut self, frontier: bool) {
        self.frontier = Some(frontier);
    }
    pub fn governor<S: Into<String>>(&mut self, governor: S) {
        self.governor = Some(governor.into());
    }
    pub fn last_major<I: Into<i64>>(&mut self, last_major: I) {
        self.last_major = Some(last_major.into());
    }
    pub fn last_minor<I: Into<i64>>(&mut self, last_minor: I) {
        self.last_minor = Some(last_minor.into());
    }
    pub fn officer(&mut self, officer: Officer) {
        self.officers.push(officer);
    }
    pub fn embassy(&mut self, embassy: Embassy) {
        self.embassies.push(embassy);
    }

    pub fn build(&mut self) -> Result<Region> {
        Ok(Region {
            name: match self.name.take() {
                Some(value) => value,
                None => {
                    return Err(Error::Builder {
                        field: "name",
                        model: "region",
                    });
                }
            },
            factbook: match self.factbook.take() {
                Some(value) => value,
                None => {
                    return Err(Error::Builder {
                        field: "factbook",
                        model: "region",
                    });
                }
            },
            population: match self.population.take() {
                Some(value) => value,
                None => {
                    return Err(Error::Builder {
                        field: "population",
                        model: "region",
                    });
                }
            },
            delegate: self.delegate.take(),
            delegate_votes: match self.delegate_votes.take() {
                Some(value) => value,
                None => {
                    return Err(Error::Builder {
                        field: "delegate_votes",
                        model: "region",
                    });
                }
            },
            delegate_auth: match self.delegate_auth.take() {
                Some(value) => value,
                None => {
                    return Err(Error::Builder {
                        field: "delegate_auth",
                        model: "region",
                    });
                }
            },
            frontier: match self.frontier.take() {
                Some(value) => value,
                None => {
                    return Err(Error::Builder {
                        field: "frontier",
                        model: "region",
                    });
                }
            },
            governor: self.governor.take(),
            last_major: match self.last_major.take() {
                Some(value) => value,
                None => {
                    return Err(Error::Builder {
                        field: "last_major",
                        model: "region",
                    });
                }
            },
            last_minor: match self.last_minor.take() {
                Some(value) => value,
                None => {
                    return Err(Error::Builder {
                        field: "last_minor",
                        model: "region",
                    });
                }
            },
            officers: std::mem::take(&mut self.officers),
            embassies: std::mem::take(&mut self.embassies),
        })
    }
}

#[derive(Debug)]
// #[builder(setter(into))]
pub struct Officer {
    name: String,
    office: String,
    authority: Authority,
    time: i64,
    appointer: String,
}

#[derive(Default, Debug)]
pub struct OfficerBuilder {
    name: Option<String>,
    office: Option<String>,
    authority: Option<Authority>,
    time: Option<i64>,
    appointer: Option<String>,
}

impl OfficerBuilder {
    pub fn name<S: Into<String>>(&mut self, name: S) {
        self.name = Some(name.into());
    }
    pub fn office<S: Into<String>>(&mut self, office: S) {
        self.office = Some(office.into());
    }
    pub fn authority<A: Into<Authority>>(&mut self, authority: A) {
        self.authority = Some(authority.into());
    }
    pub fn time<I: Into<i64>>(&mut self, time: I) {
        self.time = Some(time.into());
    }
    pub fn appointer<S: Into<String>>(&mut self, appointer: S) {
        self.appointer = Some(appointer.into())
    }

    pub fn build(&mut self) -> Result<Officer> {
        let Some(name) = self.name.take() else {
            return Err(Error::Builder {
                field: "name",
                model: "officer",
            });
        };
        let Some(office) = self.office.take() else {
            return Err(Error::Builder {
                field: "office",
                model: "officer",
            });
        };
        let Some(authority) = self.authority.take() else {
            return Err(Error::Builder {
                field: "authority",
                model: "officer",
            });
        };
        let Some(time) = self.time.take() else {
            return Err(Error::Builder {
                field: "authority",
                model: "officer",
            });
        };
        let Some(appointer) = self.appointer.take() else {
            return Err(Error::Builder {
                field: "appointer",
                model: "officer",
            });
        };

        Ok(Officer {
            name,
            office,
            authority,
            time,
            appointer,
        })
    }
}

#[derive(Debug)]
pub enum EmbassyStatus {
    Open,
    Pending,
    Closing,
    Invited,
    Requested,
    Rejected,
    Denied,
}

impl From<&str> for EmbassyStatus {
    fn from(value: &str) -> Self {
        match value {
            "pending" => Self::Pending,
            "closing" => Self::Closing,
            "invited" => Self::Invited,
            "requested" => Self::Requested,
            "rejected" => Self::Rejected,
            "denied" => Self::Denied,
            _ => Self::Open,
        }
    }
}

#[derive(Debug)]
// #[builder(setter(into))]
pub struct Embassy {
    region: String,
    status: EmbassyStatus,
}

#[derive(Default, Debug)]
pub struct EmbassyBuilder {
    region: Option<String>,
    status: Option<EmbassyStatus>,
}

impl EmbassyBuilder {
    pub fn region<S: Into<String>>(&mut self, value: S) {
        self.region = Some(value.into());
    }
    pub fn status<E: Into<EmbassyStatus>>(&mut self, value: E) {
        self.status = Some(value.into());
    }

    pub fn build(&mut self) -> Result<Embassy> {
        let Some(region) = self.region.take() else {
            return Err(Error::Builder {
                field: "region",
                model: "embassy",
            });
        };
        let Some(status) = self.status.take() else {
            return Err(Error::Builder {
                field: "status",
                model: "embassy",
            });
        };

        Ok(Embassy { region, status })
    }
}

#[derive(Clone, Debug)]
pub struct Authority {
    executive: bool,
    world_assembly: bool,
    succession: bool,
    appearance: bool,
    border_control: bool,
    communications: bool,
    embassies: bool,
    polls: bool,
}

impl Authority {
    pub fn executive(&self) -> bool {
        self.executive
    }
    pub fn succession(&self) -> bool {
        self.succession
    }
    pub fn appearance(&self) -> bool {
        self.appearance
    }
    pub fn border_control(&self) -> bool {
        self.border_control
    }
    pub fn communications(&self) -> bool {
        self.communications
    }
    pub fn embassies(&self) -> bool {
        self.embassies
    }
    pub fn polls(&self) -> bool {
        self.polls
    }
}

impl From<&str> for Authority {
    fn from(value: &str) -> Self {
        Self {
            executive: value.contains('X'),
            world_assembly: value.contains('W'),
            succession: value.contains('S'),
            appearance: value.contains('A'),
            border_control: value.contains('B'),
            communications: value.contains('C'),
            embassies: value.contains('E'),
            polls: value.contains('P'),
        }
    }
}

impl Display for Authority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.executive {
            write!(f, "X")?;
        }
        if self.world_assembly {
            write!(f, "W")?;
        }
        if self.succession {
            write!(f, "S")?;
        }
        if self.appearance {
            write!(f, "A")?;
        }
        if self.border_control {
            write!(f, "B")?;
        }
        if self.communications {
            write!(f, "C")?;
        }
        if self.embassies {
            write!(f, "E")?;
        }
        if self.polls {
            write!(f, "P")?;
        }

        Ok(())
    }
}
