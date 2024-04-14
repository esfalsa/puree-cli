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

// // impl From<RegionBuilderError> for Error {
// //     fn from(value: RegionBuilderError) -> Self {
// //         match value {
// //             RegionBuilderError::UninitializedField(field) => Error::Builder {
// //                 field,
// //                 model: "region",
// //             },
// //             RegionBuilderError::ValidationError(_) => {
// //                 // there should not be any custom validation for RegionBuilder
// //                 panic!("attempted custom validation for region")
// //             }
// //         }
// //     }
// // }

// // impl From<OfficerBuilderError> for Error {
// //     fn from(value: OfficerBuilderError) -> Self {
// //         match value {
// //             OfficerBuilderError::UninitializedField(field) => Error::Builder {
// //                 field,
// //                 model: "officer",
// //             },
// //             OfficerBuilderError::ValidationError(_) => {
// //                 // there should not be any custom validation for OfficerBuilder
// //                 panic!("attempted custom validation for officer")
// //             }
// //         }
// //     }
// // }

// // impl From<EmbassyBuilderError> for Error {
// //     fn from(value: EmbassyBuilderError) -> Self {
// //         match value {
// //             EmbassyBuilderError::UninitializedField(field) => Error::Builder {
// //                 field,
// //                 model: "embassy",
// //             },
// //             EmbassyBuilderError::ValidationError(_) => {
// //                 // there should not be any custom validation for EmbassyBuilder
// //                 panic!("attempted custom validation for embassy")
// //             }
// //         }
// //     }
// }
