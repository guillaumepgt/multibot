//! Tools for filesystem, process execution and git operations for MultiBot agents.

pub fn tools_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
