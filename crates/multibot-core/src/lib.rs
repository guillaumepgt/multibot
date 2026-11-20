//! Core data models, persistence, and state logic for MultiBot.

pub mod db;
pub mod error;
pub mod models;

pub use db::Database;
pub use error::{CoreError, Result};
pub use models::*;

/// Version de multibot-core.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
