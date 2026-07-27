//! Deprecated compatibility façade for OntoCore → Strixonomy (v0.27).
//!
//! Prefer the [`strixonomy`] crate. This package re-exports the same API and will be
//! removed after the 1.0 compatibility window.
#![doc(html_root_url = "https://docs.rs/ontocore")]
#![deprecated(
    since = "0.27.0",
    note = "renamed to the `strixonomy` crate; see docs/migration/v0.27.md"
)]

pub use strixonomy::*;
