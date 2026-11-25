//! Helper types for UDD columns.

use debversion::Version;
use sqlx::postgres::{PgTypeInfo, PgValueRef};
use sqlx::Postgres;

/// The raw contents of a `debversion` column that may not hold a valid
/// Debian version.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct UnparsedVersion(pub String);

impl UnparsedVersion {
    /// The raw value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Parse the value as a Debian version.
    pub fn parse(&self) -> Result<Version, debversion::ParseError> {
        self.0.parse()
    }
}

impl std::fmt::Display for UnparsedVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl sqlx::Type<Postgres> for UnparsedVersion {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("debversion")
    }
}

impl sqlx::Decode<'_, Postgres> for UnparsedVersion {
    fn decode(value: PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let s: &str = sqlx::Decode::<Postgres>::decode(value)?;
        Ok(UnparsedVersion(s.to_owned()))
    }
}

impl sqlx::Encode<'_, Postgres> for UnparsedVersion {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        sqlx::Encode::<Postgres>::encode_by_ref(&self.0.as_str(), buf)
    }
}
