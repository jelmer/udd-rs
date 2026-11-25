//! Types for UDD columns that need more than a plain [`Version`].

use std::fmt;
use std::str::FromStr;

use debversion::Version;
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use sqlx::Postgres;

/// Error parsing a [`CiVersion`] or [`HintVersion`].
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError(debversion::ParseError);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for ParseError {}

impl From<debversion::ParseError> for ParseError {
    fn from(e: debversion::ParseError) -> Self {
        ParseError(e)
    }
}

/// The version recorded for a run in the `ci` table.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(try_from = "String", into = "String")
)]
pub enum CiVersion {
    Version(Version),
    /// Stored as `n/a`, for runs where debci could not determine the version.
    NotAvailable,
}

impl FromStr for CiVersion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "n/a" {
            Ok(CiVersion::NotAvailable)
        } else {
            Ok(CiVersion::Version(s.parse()?))
        }
    }
}

impl fmt::Display for CiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CiVersion::Version(v) => v.fmt(f),
            CiVersion::NotAvailable => f.write_str("n/a"),
        }
    }
}

/// The version in the `hints` table, optionally qualified with an
/// architecture as in `1.0-1/arm64`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(try_from = "String", into = "String")
)]
pub struct HintVersion {
    pub version: Version,
    pub architecture: Option<String>,
}

impl FromStr for HintVersion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (version, architecture) = match s.split_once('/') {
            Some((version, arch)) => (version, Some(arch.to_owned())),
            None => (s, None),
        };
        Ok(HintVersion {
            version: version.parse()?,
            architecture,
        })
    }
}

impl fmt::Display for HintVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.version.fmt(f)?;
        if let Some(arch) = &self.architecture {
            write!(f, "/{arch}")?;
        }
        Ok(())
    }
}

macro_rules! debversion_text_type {
    ($ty:ty) => {
        impl TryFrom<String> for $ty {
            type Error = ParseError;

            fn try_from(s: String) -> Result<Self, Self::Error> {
                s.parse()
            }
        }

        impl From<$ty> for String {
            fn from(v: $ty) -> String {
                v.to_string()
            }
        }

        impl sqlx::Type<Postgres> for $ty {
            fn type_info() -> PgTypeInfo {
                PgTypeInfo::with_name("debversion")
            }
        }

        impl sqlx::Decode<'_, Postgres> for $ty {
            fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
                let s: &str = sqlx::Decode::<Postgres>::decode(value)?;
                Ok(s.parse()?)
            }
        }

        impl sqlx::Encode<'_, Postgres> for $ty {
            fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
                sqlx::Encode::<Postgres>::encode_by_ref(&self.to_string().as_str(), buf)
            }
        }
    };
}

debversion_text_type!(CiVersion);
debversion_text_type!(HintVersion);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ci_version() {
        assert_eq!(
            "1.2-1".parse::<CiVersion>().unwrap(),
            CiVersion::Version("1.2-1".parse().unwrap())
        );
        assert_eq!("n/a".parse::<CiVersion>().unwrap(), CiVersion::NotAvailable);
        assert_eq!(CiVersion::NotAvailable.to_string(), "n/a");
        assert!("x/y".parse::<CiVersion>().is_err());
    }

    #[test]
    fn hint_version() {
        let v: HintVersion = "1:21.1.8-14/armhf".parse().unwrap();
        assert_eq!(
            v,
            HintVersion {
                version: "1:21.1.8-14".parse().unwrap(),
                architecture: Some("armhf".to_owned()),
            }
        );
        assert_eq!(v.to_string(), "1:21.1.8-14/armhf");

        let v: HintVersion = "1.0-1".parse().unwrap();
        assert_eq!(v.architecture, None);
        assert_eq!(v.to_string(), "1.0-1");
    }
}
