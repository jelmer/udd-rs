//! Access to the [Ultimate Debian Database](https://wiki.debian.org/UltimateDebianDatabase).
//!
//! ```no_run
//! # async fn example() -> Result<(), sqlx::Error> {
//! let pool = udd::connect_public_mirror().await?;
//! let sources: Vec<udd::Source> =
//!     sqlx::query_as("SELECT * FROM sources WHERE source = $1 AND release = 'sid'")
//!         .bind("dulwich")
//!         .fetch_all(&pool)
//!         .await?;
//! # Ok(())
//! # }
//! ```

pub mod enums;
pub mod tables;
pub mod types;

pub use debversion::Version;
pub use enums::*;
pub use sqlx;
pub use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
pub use tables::*;
pub use types::UnparsedVersion;

/// Hostname of the public UDD mirror.
pub const PUBLIC_MIRROR_HOST: &str = "udd-mirror.debian.net";

/// Port of the public UDD mirror.
pub const PUBLIC_MIRROR_PORT: u16 = 5432;

/// Username for the public UDD mirror.
pub const PUBLIC_MIRROR_USER: &str = "udd-mirror";

/// Password for the public UDD mirror.
pub const PUBLIC_MIRROR_PASSWORD: &str = "udd-mirror";

/// Database name of UDD.
pub const DATABASE: &str = "udd";

/// Connection options for the public UDD mirror.
pub fn public_mirror_options() -> PgConnectOptions {
    PgConnectOptions::new()
        .host(PUBLIC_MIRROR_HOST)
        .port(PUBLIC_MIRROR_PORT)
        .username(PUBLIC_MIRROR_USER)
        .password(PUBLIC_MIRROR_PASSWORD)
        .database(DATABASE)
}

/// Connect to the public UDD mirror.
pub async fn connect_public_mirror() -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect_with(public_mirror_options())
        .await
}

/// Connect to a UDD instance at the given PostgreSQL URL.
pub async fn connect(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(url).await
}
