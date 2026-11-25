//! Row types for the UDD tables.
//!
//! Field types follow the column types in UDD: `debversion` columns map to
//! [`debversion::Version`], nullable columns to [`Option`].

pub mod archive;
pub mod blends;
pub mod bugs;
pub mod carnivore;
pub mod lintian;
pub mod popcon;
pub mod qa;
pub mod security;
pub mod testing;
pub mod ubuntu;
pub mod upload_history;

pub use archive::*;
pub use blends::*;
pub use bugs::*;
pub use carnivore::*;
pub use lintian::*;
pub use popcon::*;
pub use qa::*;
pub use security::*;
pub use testing::*;
pub use ubuntu::*;
pub use upload_history::*;
