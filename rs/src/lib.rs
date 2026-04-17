//! A Rust implementation of Human Versions.
//!
//! Provides a semantic human versioning system, representing human age at
//! different time scales. Major versions represent the number of decades
//! lived, incrementing every 10 years. Minor versions represent the number
//! of years lived within the current decade, and patch versions represent
//! the number of months lived within the current year.
//!
//! ```
//! use chrono::NaiveDate;
//! use humver::Version;
//!
//! fn main() {
//!     let birth = NaiveDate::from_ymd_opt(2003, 4, 17).unwrap();
//!     let as_of = NaiveDate::from_ymd_opt(2026, 4, 17).unwrap();
//!
//!     let version = Version::try_from_dates(birth, as_of).unwrap();
//!
//!     assert_eq!(version.to_string(), "2.3.0");
//! }
//! ```
mod errors;

use chrono::Datelike;
use std::fmt;

use crate::errors::HumVerErr;

/// Human Version number representing age in a `major.minor.patch` format.
///
/// Age as a semantic human version, where the components represent different
/// time scales:
/// - `major`: Number of decades lived (increments every 10 years).
/// - `minor`: Years within the current decade (0-9).
/// - `patch`: Months within the current year (0-11).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Version {
    major: u64,
    minor: u64,
    patch: u64,
}

impl Version {
    /// Create a Human Version from a birth and as-of date.
    pub fn try_from_dates(
        birth: impl Datelike,
        as_of: impl Datelike,
    ) -> Result<Self, HumVerErr> {
        if as_of.year() < birth.year()
            || (as_of.year() == birth.year()
                && as_of.ordinal() < birth.ordinal())
        {
            return Err(HumVerErr::NegativeAge);
        }

        let total_months = ((as_of.year() - birth.year()) * 12
            + as_of.month() as i32
            - birth.month() as i32
            - if as_of.day() < birth.day() { 1 } else { 0 })
        .max(0) as u64;

        Ok(Self {
            major: total_months / 120,
            minor: (total_months % 120) / 12,
            patch: total_months % 12,
        })
    }

    /// Returns an age in years from the Human Version.
    pub fn to_years(&self) -> u64 {
        self.major * 10 + self.minor
    }

    /// Returns an age in months from the Human Version.
    pub fn to_months(&self) -> u64 {
        self.major * 120 + self.minor * 12 + self.patch
    }

    /// Returns the human major version.
    pub fn major(&self) -> u64 {
        self.major
    }

    /// Returns the human minor version.
    ///
    /// The value ranges from 0 to 9.
    pub fn minor(&self) -> u64 {
        self.minor
    }

    /// Returns the human patch version.
    ///
    /// The value ranges from 0 to 11.
    pub fn patch(&self) -> u64 {
        self.patch
    }
}

impl<B: Datelike, A: Datelike> TryFrom<(B, A)> for Version {
    type Error = HumVerErr;

    fn try_from((birth, as_of): (B, A)) -> Result<Self, Self::Error> {
        Self::try_from_dates(birth, as_of)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl std::str::FromStr for Version {
    type Err = HumVerErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(".").collect();

        if parts.is_empty() || parts.len() > 3 {
            return Err(HumVerErr::ParseVersion(s.to_string()));
        }

        let major = parts[0]
            .parse::<u64>()
            .map_err(|_| HumVerErr::ParseVersion(s.to_string()))?;

        let minor = if parts.len() > 1 {
            parts[1]
                .parse::<u64>()
                .map_err(|_| HumVerErr::ParseVersion(s.to_string()))?
        } else {
            0
        };

        let patch = if parts.len() > 2 {
            parts[2]
                .parse::<u64>()
                .map_err(|_| HumVerErr::ParseVersion(s.to_string()))?
        } else {
            0
        };

        if minor >= 10 {
            return Err(HumVerErr::InvalidMinor);
        }
        if patch >= 12 {
            return Err(HumVerErr::InvalidPatch);
        }

        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn try_from_dates() {
        let birth = NaiveDate::from_ymd_opt(2003, 4, 17).unwrap();
        let as_of = NaiveDate::from_ymd_opt(2026, 4, 17).unwrap();

        let version = Version::try_from_dates(birth, as_of).unwrap();
        assert_eq!(version.major(), 2);
        assert_eq!(version.minor(), 3);
        assert_eq!(version.patch(), 0);
    }

    #[test]
    fn try_from_negative_age() {
        let birth = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let as_of = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

        let result = Version::try_from_dates(birth, as_of);
        assert_eq!(result, Err(HumVerErr::NegativeAge));
    }

    #[test]
    fn try_from_same_year_negative() {
        let birth = NaiveDate::from_ymd_opt(2000, 12, 12).unwrap();
        let as_of = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();

        let result = Version::try_from_dates(birth, as_of);
        assert_eq!(result, Err(HumVerErr::NegativeAge));
    }

    #[test]
    fn to_years() {
        let birth = NaiveDate::from_ymd_opt(2003, 4, 17).unwrap();
        let as_of = NaiveDate::from_ymd_opt(2026, 4, 17).unwrap();

        let version = Version::try_from_dates(birth, as_of).unwrap();
        assert_eq!(version.to_years(), 23);
    }

    #[test]
    fn to_months() {
        let birth = NaiveDate::from_ymd_opt(2003, 4, 17).unwrap();
        let as_of = NaiveDate::from_ymd_opt(2026, 4, 17).unwrap();

        let version = Version::try_from_dates(birth, as_of).unwrap();
        assert_eq!(version.to_months(), 276);
    }

    #[test]
    fn try_from_tuple() {
        let birth = NaiveDate::from_ymd_opt(2003, 4, 17).unwrap();
        let as_of = NaiveDate::from_ymd_opt(2026, 4, 17).unwrap();

        let version = Version::try_from((birth, as_of)).unwrap();
        assert_eq!(version.major(), 2);
        assert_eq!(version.minor(), 3);
        assert_eq!(version.patch(), 0);
    }

    #[test]
    fn display() {
        let birth = NaiveDate::from_ymd_opt(2003, 4, 17).unwrap();
        let as_of = NaiveDate::from_ymd_opt(2026, 4, 17).unwrap();

        let version = Version::try_from_dates(birth, as_of).unwrap();
        assert_eq!(version.to_string(), "2.3.0");
    }

    #[test]
    fn from_str() {
        let v: Version = "1.2.3".parse().unwrap();
        assert_eq!((v.major(), v.minor(), v.patch()), (1, 2, 3));
    }

    #[test]
    fn from_str_no_patch() {
        let v: Version = "1.2".parse().unwrap();
        assert_eq!((v.major(), v.minor(), v.patch()), (1, 2, 0));
    }

    #[test]
    fn from_str_major_only() {
        let v: Version = "1".parse().unwrap();
        assert_eq!((v.major(), v.minor(), v.patch()), (1, 0, 0));
    }

    #[test]
    fn from_str_empty() {
        assert!(matches!(
            "".parse::<Version>(),
            Err(HumVerErr::ParseVersion(_))
        ));
    }

    #[test]
    fn from_str_too_many() {
        assert!(matches!(
            "1.2.3.4".parse::<Version>(),
            Err(HumVerErr::ParseVersion(_))
        ));
    }

    #[test]
    fn from_str_alpha() {
        assert!(matches!(
            "a.b.c".parse::<Version>(),
            Err(HumVerErr::ParseVersion(_))
        ));
    }

    #[test]
    fn from_str_invalid_minor() {
        assert!(matches!(
            "1.10.0".parse::<Version>(),
            Err(HumVerErr::InvalidMinor)
        ));
    }

    #[test]
    fn from_str_invalid_patch() {
        assert!(matches!(
            "1.0.12".parse::<Version>(),
            Err(HumVerErr::InvalidPatch)
        ));
    }
}
