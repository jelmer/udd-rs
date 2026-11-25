//! Rust counterparts of the enum types defined in UDD.

/// The `bugs_severity` type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Type)]
#[sqlx(type_name = "bugs_severity", rename_all = "lowercase")]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum BugSeverity {
    Fixed,
    Wishlist,
    Minor,
    Normal,
    Important,
    Serious,
    Grave,
    Critical,
}

impl BugSeverity {
    /// Whether this severity is release-critical.
    pub fn is_release_critical(self) -> bool {
        self >= BugSeverity::Serious
    }
}

/// The `dehs_status` type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(type_name = "dehs_status", rename_all = "kebab-case")]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum DehsStatus {
    Error,
    Uptodate,
    Outdated,
    NewerInDebian,
}

/// The `ftp_autoreject_level` type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(type_name = "ftp_autoreject_level", rename_all = "lowercase")]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum FtpAutorejectLevel {
    Fatal,
    Nonfatal,
}

/// The `ftp_autoreject_type` type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(type_name = "ftp_autoreject_type", rename_all = "lowercase")]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum FtpAutorejectType {
    Lintian,
}

/// The `lintian_tagtype` type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(type_name = "lintian_tagtype", rename_all = "lowercase")]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum LintianTagType {
    Experimental,
    Overridden,
    Pedantic,
    Information,
    Warning,
    Error,
    Classification,
    Masked,
}

/// The `security_issues_releases_status` type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(
    type_name = "security_issues_releases_status",
    rename_all = "lowercase"
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum SecurityIssueReleaseStatus {
    Open,
    Resolved,
    Undetermined,
}

/// The `security_issues_scope` type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(type_name = "security_issues_scope", rename_all = "lowercase")]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum SecurityIssueScope {
    Local,
    Remote,
}

#[cfg(test)]
mod tests {
    use super::BugSeverity;

    #[test]
    fn release_critical() {
        assert!(!BugSeverity::Important.is_release_critical());
        assert!(BugSeverity::Serious.is_release_critical());
        assert!(BugSeverity::Critical.is_release_critical());
    }
}
