//! RDF/OWL parsing into [`strixonomy_core`] entities via Oxigraph.
//!
//! Entry point: [`parse_ontology_file`].
//!
//! # API stability
//!
//! **Pre-1.0:** parsing APIs and [`ParsedOntology`] fields may change between minor releases.

mod obo;
mod rdf;
mod vocab;

pub use obo::{parse_obo_file, parse_obo_text};
pub use rdf::{
    parse_ontology_file, parse_ontology_text, serialize_quads_turtle, ParseError, ParsedOntology,
};
pub use vocab::OWL;
