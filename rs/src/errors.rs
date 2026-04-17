use core::fmt::{Display, Formatter, Result};
use std::error::Error;

/// An enum representing all errors for Human Versions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HumVerErr {
    /// As-of date is before birth date.
    NegativeAge,

    /// Version string format is invalid.
    ParseVersion(String),

    /// Minor version exceeds maximum value of 9.
    InvalidMinor,

    /// Patch version exceeds maximum value of 11.
    InvalidPatch,
}

impl Display for HumVerErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::NegativeAge => "age cannot be negative",
            Self::ParseVersion(string) => {
                return write!(f, "invalid version format: {string}");
            }
            Self::InvalidMinor => "minor version must be 0-9",
            Self::InvalidPatch => "patch version must be 0-11",
        })
    }
}

impl Error for HumVerErr {}

#[test]
fn test_display() {
    assert_eq!(HumVerErr::NegativeAge.to_string(), "age cannot be negative");

    assert_eq!(
        HumVerErr::ParseVersion("1.0.1.0".to_string()).to_string(),
        "invalid version format: 1.0.1.0"
    );

    assert_eq!(
        HumVerErr::InvalidMinor.to_string(),
        "minor version must be 0-9"
    );

    assert_eq!(
        HumVerErr::InvalidPatch.to_string(),
        "patch version must be 0-11"
    );
}
