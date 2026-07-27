//! Catalog lint rules and diagnostic collection for Strixonomy v0.3.
//!
//! # API stability
//!
//! **Pre-1.0:** diagnostic rule codes and severities are stable within a minor release
//! but new rules may be added.

mod config;
mod engine;
mod input;
mod location;
mod rules;

pub use config::{find_config, DiagnosticConfig, RuleConfig};
pub use engine::{
    collect_diagnostics, collect_diagnostics_with_config, collect_diagnostics_with_sources,
};
pub use input::DiagnosticInput;
pub use location::{entity_needles, find_in_source};
