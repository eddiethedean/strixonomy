//! Strixonomy — semantic workspace engine for ontology development.
//!
//! Strixonomy indexes ontology workspaces and provides search, diagnostics, refactoring,
//! SQL, SPARQL, reasoning integration, and LSP services.
//!
//! Implementation is provided by the `strixonomy-*` crates.
//!
//! # Quick start
//!
//! ```no_run
//! use strixonomy::Workspace;
//!
//! # fn demo() -> Result<(), strixonomy::Error> {
//! let workspace = Workspace::open(".")?;
//! let result = workspace.query("SELECT short_name, labels FROM classes")?;
//! for row in &result.rows {
//!     println!("{:?}", row);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Validate
//!
//! ```no_run
//! use strixonomy::Workspace;
//!
//! # fn demo() -> Result<(), strixonomy::Error> {
//! let workspace = Workspace::open(".")?;
//! let diagnostics = workspace.diagnostics();
//! println!("{} diagnostics", diagnostics.len());
//! # Ok(())
//! # }
//! ```

pub mod catalog;
pub mod diagnostics;
pub mod diff;
pub mod docs;
pub mod edit;
pub mod error;
pub mod obo;
pub mod owl;
pub mod parser;
pub mod query;
pub mod reasoner;
pub mod refactor;
pub mod swrl;
pub mod workspace;

#[cfg(feature = "lsp")]
pub mod lsp;

#[cfg(feature = "plugins")]
pub mod plugin;

pub use error::Error;
pub use strixonomy_core::{Diagnostic, Entity, StrixonomyError};
pub use workspace::{Workspace, WorkspaceOptions};
