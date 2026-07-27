use crate::bridge::bridge_ontology;
use crate::error::{OwlError, Result};
use crate::OwlBridgeResult;
use horned_owl::io::rdf::reader::read;
use horned_owl::model::{RcAnnotatedComponent, RcStr};
use horned_owl::ontology::component_mapped::ComponentMappedOntology;
use horned_owl::ontology::set::SetOntology;
use oxigraph::io::{RdfFormat, RdfParser, RdfSerializer};
use oxigraph::model::Quad;
use std::collections::BTreeMap;
use std::io::Cursor;
use std::path::Path;
use strixonomy_core::OntologyFormat;

/// Result of loading a document through the Horned-OWL pipeline.
#[derive(Debug, Clone)]
pub struct OwlLoadResult {
    pub bridge: OwlBridgeResult,
    pub incomplete: bool,
    pub load_warning: Option<String>,
    /// RDF quads for the Oxigraph SPARQL store (empty for Turtle — caller already has quads).
    pub quads: Vec<Quad>,
}

/// Load Turtle source via Oxigraph quads → RDF/XML → Horned-OWL.
pub fn load_turtle_text(
    path: &Path,
    ontology_id: &str,
    source_text: &str,
    quads: &[Quad],
    namespaces: &BTreeMap<String, String>,
) -> Result<OwlLoadResult> {
    let _ = path;
    match load_from_quads(quads) {
        Ok((ontology, incomplete)) => {
            let bridge = bridge_ontology(ontology, ontology_id, source_text, namespaces);
            Ok(OwlLoadResult {
                bridge,
                incomplete,
                load_warning: if incomplete {
                    Some("Horned-OWL reported incomplete RDF parse".to_string())
                } else {
                    None
                },
                quads: Vec::new(),
            })
        }
        Err(e) => Err(e),
    }
}

/// Convert Oxigraph quads to Horned-OWL ontology via RDF/XML.
pub fn load_from_quads(
    quads: &[Quad],
) -> Result<(
    horned_owl::io::rdf::reader::ConcreteRDFOntology<
        horned_owl::model::RcStr,
        horned_owl::model::RcAnnotatedComponent,
    >,
    bool,
)> {
    let rdf_xml = quads_to_rdf_xml(quads).map_err(OwlError::LoadFailed)?;
    let mut cursor = Cursor::new(rdf_xml);
    let (ontology, incomplete) =
        read(&mut cursor, Default::default()).map_err(|e| OwlError::LoadFailed(e.to_string()))?;
    Ok((ontology, !incomplete.is_complete()))
}

fn quads_to_rdf_xml(quads: &[Quad]) -> std::result::Result<Vec<u8>, String> {
    let mut serializer = RdfSerializer::from_format(RdfFormat::RdfXml).for_writer(Vec::new());
    for quad in quads {
        serializer.serialize_quad(quad).map_err(|e| e.to_string())?;
    }
    serializer.finish().map_err(|e| e.to_string())
}

/// Project a Horned ontology to Oxigraph quads (N-Triples via Horned RDF writer).
fn ontology_to_quads(
    ont: &ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
) -> Result<Vec<Quad>> {
    let mut buf = Vec::new();
    horned_owl::io::rdf::writer::write_to_rdf_format(&mut buf, ont, "ttl")
        .map_err(|e| OwlError::LoadFailed(e.to_string()))?;
    let parser = RdfParser::from_format(RdfFormat::NTriples);
    parser
        .for_reader(buf.as_slice())
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|e| OwlError::LoadFailed(e.to_string()))
}

/// Load OWL/XML (`.owx`) source via Horned-OWL.
///
/// Horned's OWX reader does not expose RDF `IncompleteParse`. After a successful OWX load we
/// project the ontology to RDF for the SPARQL store and propagate incompleteness from that
/// RDF round-trip (analogous to Turtle `load_from_quads`).
pub fn load_owx_text(
    path: &Path,
    ontology_id: &str,
    source_text: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<OwlLoadResult> {
    let _ = path;
    use horned_owl::io::owx::reader::read as read_owx;
    let mut cursor = Cursor::new(source_text.as_bytes());
    let (set_ont, mapping): (SetOntology<RcStr>, _) = read_owx(&mut cursor, Default::default())
        .map_err(|e| OwlError::LoadFailed(e.to_string()))?;

    let mut merged_ns = namespaces.clone();
    for (prefix, iri) in mapping.mappings() {
        merged_ns.entry(prefix.clone()).or_insert_with(|| iri.clone());
    }

    let mapped: ComponentMappedOntology<RcStr, RcAnnotatedComponent> = set_ont.into();
    let quads = ontology_to_quads(&mapped)?;
    let incomplete = match load_from_quads(&quads) {
        Ok((_, incomplete)) => incomplete,
        Err(_) => {
            // Projection produced quads Horned cannot re-parse as RDF/XML — treat as incomplete.
            true
        }
    };
    let bridge = bridge_ontology(mapped, ontology_id, source_text, &merged_ns);
    Ok(OwlLoadResult {
        bridge,
        incomplete,
        load_warning: if incomplete {
            Some("Horned-OWL reported incomplete RDF projection of OWL/XML".to_string())
        } else {
            None
        },
        quads,
    })
}

/// Whether Horned-OWL loading is supported for this format.
pub fn supports_horned_load(format: OntologyFormat) -> bool {
    matches!(
        format,
        OntologyFormat::Turtle
            | OntologyFormat::Owl
            | OntologyFormat::RdfXml
            | OntologyFormat::OwlXml
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxigraph::io::RdfParser;

    #[test]
    fn loads_fixture_turtle_via_horned() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let parser = RdfParser::from_format(RdfFormat::Turtle);
        let quads: Vec<Quad> =
            parser.for_reader(ttl.as_bytes()).collect::<std::result::Result<Vec<_>, _>>().unwrap();
        let namespaces = BTreeMap::from([
            ("ex".to_string(), "http://example.org/people#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ]);
        let result = load_turtle_text(Path::new("example.ttl"), "doc-1", ttl, &quads, &namespaces)
            .expect("load");
        assert!(!result.bridge.entities.is_empty());
        assert!(result.bridge.entities.iter().any(|e| e.short_name == "Person"));
        assert!(result.quads.is_empty());
    }

    #[test]
    fn loads_owx_with_rdf_quads_for_sparql() {
        let owx = include_str!("../../../examples/protege-roundtrip/example.owx");
        let result = load_owx_text(Path::new("example.owx"), "doc-owx", owx, &BTreeMap::new())
            .expect("load owx");
        assert!(result.bridge.entities.iter().any(|e| e.short_name == "Department"));
        assert!(!result.quads.is_empty(), "OWL/XML load must project RDF quads for SPARQL");
        assert!(
            result.quads.iter().any(|q| {
                q.subject.to_string().contains("Department")
                    || q.object.to_string().contains("Department")
            }),
            "expected Department IRI in projected quads"
        );
        // Well-formed OWX projects cleanly; incompleteness is computed (not hardcoded false).
        assert!(!result.incomplete);
        assert!(result.load_warning.is_none());
    }

    #[test]
    fn owx_prefixes_merged_into_bridge_namespaces() {
        let owx = include_str!("../../../examples/protege-roundtrip/example.owx");
        let result = load_owx_text(Path::new("example.owx"), "doc-owx", owx, &BTreeMap::new())
            .expect("load owx");
        assert!(
            result
                .bridge
                .namespace_rows
                .iter()
                .any(|n| n.prefix == "ex" && n.iri.contains("example.org/org")),
            "expected ex: prefix from OWX PrefixMapping, got {:?}",
            result.bridge.namespace_rows
        );
    }
}
