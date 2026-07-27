//! Serialize Horned ontologies to RDF/XML and OWL/XML (v0.21).

use crate::error::{OwlError, Result};
use curie::PrefixMapping;
use horned_owl::model::{RcAnnotatedComponent, RcStr};
use horned_owl::ontology::component_mapped::ComponentMappedOntology;
use std::collections::BTreeMap;
use std::io::Cursor;

/// Serialize a component-mapped ontology as RDF/XML (`application/rdf+xml`).
pub fn serialize_rdf_xml(
    ont: &ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
) -> Result<String> {
    let mut buf = Vec::new();
    horned_owl::io::rdf::writer::write(&mut buf, ont)
        .map_err(|e| OwlError::SerializeFailed(e.to_string()))?;
    String::from_utf8(buf).map_err(|e| OwlError::SerializeFailed(e.to_string()))
}

/// Serialize a component-mapped ontology as OWL/XML.
///
/// `namespaces` become OWL/XML `Prefix` declarations when non-empty.
pub fn serialize_owl_xml(
    ont: &ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    namespaces: &BTreeMap<String, String>,
) -> Result<String> {
    let mut mapping = PrefixMapping::default();
    for (prefix, iri) in namespaces {
        if prefix.is_empty() {
            continue;
        }
        let _ = mapping.add_prefix(prefix, iri);
    }
    for (prefix, iri) in [
        ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
        ("rdfs", "http://www.w3.org/2000/01/rdf-schema#"),
        ("owl", "http://www.w3.org/2002/07/owl#"),
        ("xsd", "http://www.w3.org/2001/XMLSchema#"),
    ] {
        let _ = mapping.add_prefix(prefix, iri);
    }

    let mut buf = Vec::new();
    horned_owl::io::owx::writer::write(&mut buf, ont, Some(&mapping))
        .map_err(|e| OwlError::SerializeFailed(e.to_string()))?;
    String::from_utf8(buf).map_err(|e| OwlError::SerializeFailed(e.to_string()))
}

/// Load RDF/XML source into a mutable component-mapped ontology.
pub fn load_rdf_xml_ontology(
    source: &str,
) -> Result<(ComponentMappedOntology<RcStr, RcAnnotatedComponent>, bool)> {
    let mut cursor = Cursor::new(source.as_bytes());
    // Horned-OWL 1.4 can panic on some malformed RDF/XML; convert to structured errors.
    let parse = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        horned_owl::io::rdf::reader::read(&mut cursor, Default::default())
    }));
    let (concrete, incomplete) = match parse {
        Ok(Ok(pair)) => pair,
        Ok(Err(e)) => return Err(OwlError::LoadFailed(e.to_string())),
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = payload.downcast_ref::<&str>() {
                (*s).to_string()
            } else {
                "RDF/XML parser panicked on malformed input".into()
            };
            return Err(OwlError::LoadFailed(msg));
        }
    };
    let mapped: ComponentMappedOntology<RcStr, RcAnnotatedComponent> = concrete.into();
    Ok((mapped, !incomplete.is_complete()))
}

/// Load OWL/XML source into a mutable component-mapped ontology plus prefix map.
///
/// The third return value is `true` when an RDF projection round-trip reports incompleteness
/// (same policy as catalog `load_owx_text` / #383).
#[allow(clippy::type_complexity)]
pub fn load_owl_xml_ontology(
    source: &str,
) -> Result<(ComponentMappedOntology<RcStr, RcAnnotatedComponent>, BTreeMap<String, String>, bool)>
{
    use horned_owl::io::owx::reader::read as read_owx;
    use horned_owl::ontology::set::SetOntology;

    let mut cursor = Cursor::new(source.as_bytes());
    let (set_ont, mapping): (SetOntology<RcStr>, _) = read_owx(&mut cursor, Default::default())
        .map_err(|e| OwlError::LoadFailed(e.to_string()))?;
    let mut namespaces = BTreeMap::new();
    for (prefix, iri) in mapping.mappings() {
        namespaces.insert(prefix.clone(), iri.clone());
    }
    let mapped: ComponentMappedOntology<RcStr, RcAnnotatedComponent> = set_ont.into();
    let incomplete = match serialize_rdf_xml(&mapped) {
        Ok(rdf) => load_rdf_xml_ontology(&rdf).map(|(_, inc)| inc).unwrap_or(true),
        Err(_) => true,
    };
    Ok((mapped, namespaces, incomplete))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rdf_xml_roundtrip_preserves_department_label() {
        let src = include_str!("../../../examples/protege-roundtrip/organization.owl");
        let (ont, incomplete) = load_rdf_xml_ontology(src).expect("load");
        assert!(!incomplete);
        let out = serialize_rdf_xml(&ont).expect("serialize");
        let (again, _) = load_rdf_xml_ontology(&out).expect("reload");
        let bridge = crate::bridge_ontology(again, "doc", &out, &BTreeMap::new());
        assert!(bridge.entities.iter().any(|e| e.short_name == "Department"));
        assert!(bridge.entities.iter().any(|e| e.labels.iter().any(|l| l == "Department")));
    }

    #[test]
    fn owl_xml_roundtrip_preserves_department() {
        let src = include_str!("../../../examples/protege-roundtrip/example.owx");
        let (ont, ns, incomplete) = load_owl_xml_ontology(src).expect("load");
        assert!(!incomplete);
        let out = serialize_owl_xml(&ont, &ns).expect("serialize");
        let (again, _, _) = load_owl_xml_ontology(&out).expect("reload");
        let bridge = crate::bridge_ontology(again, "doc", &out, &BTreeMap::new());
        assert!(bridge.entities.iter().any(|e| e.short_name == "Department"));
    }
}
