use crate::{
    error::Result,
    models::{EmbassyBuilder, EmbassyStatus, OfficerBuilder, Region, RegionBuilder},
};
use flate2::read::GzDecoder;
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use std::io::{BufReader, Read};

pub struct DumpReader<R: Read>(Reader<BufReader<GzDecoder<R>>>);

impl<R: Read> DumpReader<R> {
    /// Creates a new dump reader from the given reader.
    ///
    /// This function expects a reader that sources its data from a compressed regions daily dump. A DumpReader handles gzip decompression and XML parsing.
    pub fn new(source: R) -> Self {
        Self(Reader::from_reader(BufReader::new(GzDecoder::new(source))))
    }

    /// Returns an interator over the regions in a dump.
    pub fn regions(self) -> RegionsIter<R> {
        RegionsIter {
            source: self.0,
            buf: Vec::new(),
            current_tag: None,
        }
    }
}

/// An iterator over the regions in a dump.
pub struct RegionsIter<R: Read> {
    source: Reader<BufReader<GzDecoder<R>>>,
    buf: Vec<u8>,
    current_tag: Option<BytesStart<'static>>,
}

impl<R: Read> Iterator for RegionsIter<R> {
    type Item = Result<Region>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut region_builder = RegionBuilder::default();
        let mut officer_builder = OfficerBuilder::default();
        let mut embassy_builder = EmbassyBuilder::default();

        loop {
            match self.source.read_event_into(&mut self.buf) {
                Ok(Event::Start(e)) => self.current_tag = Some(e.to_owned()),
                Ok(Event::End(e)) => {
                    if let Some(tag) = self.current_tag.as_deref() {
                        if e.name().as_ref() == tag {
                            self.current_tag = None;
                        }
                    }

                    match e.name().as_ref() {
                        b"REGION" => return Some(region_builder.build().map_err(|e| e.into())),
                        b"OFFICER" => match officer_builder.build() {
                            Ok(officer) => region_builder.officer(officer),
                            Err(e) => return Some(Err(e.into())),
                        },
                        b"EMBASSY" => match embassy_builder.build() {
                            Ok(embassy) => region_builder.embassy(embassy),
                            Err(e) => return Some(Err(e.into())),
                        },
                        _ => (),
                    }
                }
                Ok(Event::Text(event)) => {
                    if let Some(ref tag) = self.current_tag {
                        let text = match event.unescape() {
                            Ok(content) => content,
                            Err(e) => return Some(Err(e.into())),
                        };

                        match tag.name().as_ref() {
                            b"NAME" => region_builder.name(text),
                            b"NUMNATIONS" => match text.parse::<i32>() {
                                Ok(population) => region_builder.population(population),
                                Err(e) => return Some(Err(e.into())),
                            },
                            // Some(b"NATIONS") => unimplemented!(),
                            b"DELEGATE" if text != "0" => region_builder.delegate(text),
                            b"DELEGATEVOTES" => match text.parse::<i32>() {
                                Ok(votes) => region_builder.delegate_votes(votes),
                                Err(e) => return Some(Err(e.into())),
                            },
                            b"DELEGATEAUTH" => region_builder.delegate_auth(text.as_ref()),
                            b"FRONTIER" => match text.as_ref() {
                                "1" => region_builder.frontier(true),
                                "0" => region_builder.frontier(false),
                                _ => (),
                            },
                            b"GOVERNOR" if text != "0" => {
                                region_builder.governor(text);
                            }
                            b"LASTMAJORUPDATE" => match text.parse::<i64>() {
                                Ok(time) => region_builder.last_major(time),
                                Err(e) => return Some(Err(e.into())),
                            },
                            b"LASTMINORUPDATE" => match text.parse::<i64>() {
                                Ok(time) => region_builder.last_minor(time),
                                Err(e) => return Some(Err(e.into())),
                            },

                            b"NATION" => officer_builder.name(text),
                            b"OFFICE" => officer_builder.office(text),
                            b"AUTHORITY" => officer_builder.authority(text.as_ref()),
                            b"TIME" => match text.parse::<i64>() {
                                Ok(time) => officer_builder.time(time),
                                Err(e) => return Some(Err(e.into())),
                            },
                            b"BY" => officer_builder.appointer(text),

                            b"EMBASSY" => {
                                // println!("setting embassy region to {}", text);
                                embassy_builder.region(text);

                                if let Some(ref tag) = self.current_tag {
                                    match tag.try_get_attribute("type") {
                                        Ok(Some(attr)) => match attr.unescape_value() {
                                            Ok(value) => embassy_builder.status(value.as_ref()),
                                            Err(e) => return Some(Err(e.into())),
                                        },
                                        Ok(None) => embassy_builder.status(EmbassyStatus::Open),
                                        Err(e) => return Some(Err(e.into())),
                                    }
                                }
                            }

                            _ => (),
                        }
                    }
                }
                Ok(Event::CData(e)) => {
                    if self
                        .current_tag
                        .as_ref()
                        .is_some_and(|t| t.name().as_ref() == b"FACTBOOK")
                    {
                        region_builder.factbook(String::from_utf8_lossy(&e))
                    }
                }
                Ok(Event::Eof) => return None,
                Err(e) => return Some(Err(e.into())),
                _ => (),
            }

            self.buf.clear();
        }
    }
}
