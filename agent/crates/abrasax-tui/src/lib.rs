#![allow(
    unknown_lints,
    clippy::collapsible_match,
    clippy::manual_checked_ops,
    clippy::unnecessary_sort_by,
    clippy::useless_conversion
)]

//! Presentation layer for abrasax (terminal UI + offline replay export).
//!
//! This crate holds the `tui` and `video_export` modules that were extracted
//! out of the monolithic root `abrasax` crate so they compile as a separate
//! rustc unit. The application core it builds on (server, agent, provider,
//! auth, session, tool, config, ...) lives in `abrasax-app-core` and is
//! re-exported here via `pub use abrasax_app_core::*`, so every existing
//! `crate::<module>` path (e.g. `crate::config`, `crate::server`) keeps
//! resolving unchanged across the tui code. The root `abrasax` crate (cli + bin)
//! re-exports this crate via `pub use abrasax_tui::*`.

// Application core: re-export every `abrasax-app-core` module (which itself
// re-exports `abrasax-base`) so `crate::<module>` paths resolve here exactly as
// they did before the split.
pub use abrasax_app_core::*;

// Presentation layer (kept in this crate).
pub mod tui;
pub mod video_export;
