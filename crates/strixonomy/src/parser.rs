//! RDF and OBO parsing.

pub use strixonomy_parser::{
    parse_obo_file, parse_obo_text, parse_ontology_file, parse_ontology_text,
    serialize_quads_turtle, ParseError, ParsedOntology, OWL,
};
