use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    /// The XML of the dump could not be parsed.
    #[error("could not parse XML")]
    Xml(#[from] quick_xml::Error),

    /// The dump was missing a field expectee to be present.
    #[error("missing field {field} from model {model}")]
    Builder {
        field: &'static str,
        model: &'static str,
    },

    /// The dump contained a value that could not be parsed.
    #[error("could not parse integer")]
    Parse(#[from] std::num::ParseIntError),

    #[error("invalid UTF-8 input")]
    Utf8(#[from] std::string::FromUtf8Error),
}
