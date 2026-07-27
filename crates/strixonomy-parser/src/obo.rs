//! OBO Format 1.4 parser via [`fastobo`] → Strixonomy catalog model.

use fastobo::ast::{HeaderClause, PropertyValue, TermClause};
use std::collections::BTreeMap;
use std::path::Path;
use strixonomy_core::{
    limits::{MAX_FILE_BYTES, MAX_TRIPLES_PER_FILE},
    read_to_string_capped, Annotation, Axiom, Entity, EntityKind, SourceLocation,
    AXIOM_KIND_SUB_CLASS_OF,
};

use crate::rdf::{assemble_parsed_ontology, ParseError, ParsedOntology, Result};

/// Default OBO PURL prefix (terms use `GO:0000001` → `…/obo/GO_0000001`).
const DEFAULT_OBO_BASE: &str = "http://purl.obolibrary.org/obo/";
const RDFS_SUBCLASS_OF: &str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
const IAO_DEFINITION: &str = "http://purl.obolibrary.org/obo/IAO_0000115";
const OBO_INOWL_NS: &str = "http://www.geneontology.org/formats/oboInOwl#";

pub fn parse_obo_text(path: &Path, ontology_id: &str, source_text: &str) -> Result<ParsedOntology> {
    let doc = fastobo::from_str(source_text)
        .map_err(|e| ParseError::Rdf(format!("OBO parse error in {}: {e}", path.display())))?;

    let mut namespaces = BTreeMap::new();
    for clause in doc.header().iter() {
        if let HeaderClause::Idspace(prefix, url, _) = clause {
            namespaces.insert(prefix.to_string(), url.to_string());
        }
    }

    let mut entities = Vec::new();
    let mut annotations = Vec::new();
    let mut axioms = Vec::new();
    let mut axiom_counter = 0usize;

    for entity in doc.entities() {
        let Some(term) = entity.as_term() else {
            continue;
        };
        let obo_id = ident_to_string(term.id().as_inner());
        let iri = obo_id_to_iri(&obo_id, &namespaces);
        let short_name = obo_id.split(':').next_back().unwrap_or(&obo_id).to_string();

        let mut labels = Vec::new();
        let mut comments = Vec::new();
        let mut deprecated = false;

        for clause in term.clauses() {
            match clause.as_inner() {
                TermClause::Name(name) => labels.push(name.to_string()),
                TermClause::Comment(comment) => comments.push(comment.to_string()),
                TermClause::Def(def) => {
                    // Definition is IAO_0000115 only — do not also emit rdfs:comment (#308).
                    annotations.push(Annotation {
                        subject: iri.clone(),
                        predicate: IAO_DEFINITION.to_string(),
                        object: def.text().to_string(),
                        ontology_id: ontology_id.to_string(),
                        source_location: SourceLocation::default(),
                    });
                }
                TermClause::IsObsolete(true) => deprecated = true,
                TermClause::IsA(parent) => {
                    axiom_counter += 1;
                    let parent_id = ident_to_string(parent.as_ref());
                    axioms.push(Axiom {
                        id: format!("{ontology_id}#axiom-{axiom_counter}"),
                        ontology_id: ontology_id.to_string(),
                        subject: iri.clone(),
                        // Absolute IRIs so catalog SQL / SPARQL / diff agree (#305).
                        predicate: RDFS_SUBCLASS_OF.to_string(),
                        object: obo_id_to_iri(&parent_id, &namespaces),
                        axiom_kind: AXIOM_KIND_SUB_CLASS_OF.to_string(),
                        source_location: SourceLocation::default(),
                        annotations: Vec::new(),
                    });
                }
                TermClause::Synonym(syn) => {
                    annotations.push(Annotation {
                        subject: iri.clone(),
                        predicate: format!("{OBO_INOWL_NS}has{}Synonym", scope_label(syn.scope())),
                        object: syn.description().to_string(),
                        ontology_id: ontology_id.to_string(),
                        source_location: SourceLocation::default(),
                    });
                }
                TermClause::Xref(xref) => {
                    annotations.push(Annotation {
                        subject: iri.clone(),
                        predicate: format!("{OBO_INOWL_NS}hasDbXref"),
                        object: xref.as_ref().to_string(),
                        ontology_id: ontology_id.to_string(),
                        source_location: SourceLocation::default(),
                    });
                }
                TermClause::PropertyValue(pv) => {
                    let (predicate, object) = property_value_parts(pv, &namespaces);
                    annotations.push(Annotation {
                        subject: iri.clone(),
                        predicate,
                        object,
                        ontology_id: ontology_id.to_string(),
                        source_location: SourceLocation::default(),
                    });
                }
                _ => {}
            }
        }

        entities.push(Entity {
            iri,
            short_name,
            kind: EntityKind::Class,
            ontology_id: ontology_id.to_string(),
            source_location: SourceLocation::default(),
            labels,
            comments,
            deprecated,
            obo_id: Some(obo_id),
            characteristics: Default::default(),
        });

        if entities.len() + annotations.len() + axioms.len() > MAX_TRIPLES_PER_FILE {
            return Err(ParseError::LimitExceeded(format!(
                "OBO file exceeds entity/axiom limit: {}",
                path.display()
            )));
        }
    }

    if !namespaces.contains_key("") {
        namespaces.insert(String::new(), DEFAULT_OBO_BASE.to_string());
    }

    Ok(assemble_parsed_ontology(
        ontology_id,
        Some(DEFAULT_OBO_BASE.to_string()),
        namespaces,
        entities,
        annotations,
        axioms,
    ))
}

fn ident_to_string<T: std::fmt::Display>(ident: &T) -> String {
    ident.to_string()
}

fn scope_label(scope: &fastobo::ast::SynonymScope) -> &'static str {
    use fastobo::ast::SynonymScope;
    match scope {
        SynonymScope::Exact => "Exact",
        SynonymScope::Broad => "Broad",
        SynonymScope::Narrow => "Narrow",
        SynonymScope::Related => "Related",
    }
}

/// Split property_value into predicate IRI + object (not the full Display form) (#306).
fn property_value_parts(
    pv: &PropertyValue,
    namespaces: &BTreeMap<String, String>,
) -> (String, String) {
    let predicate = obo_id_to_iri(&ident_to_string(pv.property()), namespaces);
    let object = match pv {
        PropertyValue::Resource(r) => obo_id_to_iri(&ident_to_string(r.target()), namespaces),
        PropertyValue::Literal(l) => l.literal().as_str().to_string(),
    };
    (predicate, object)
}

/// Map an OBO ID to an IRI using `idspace:` expansions when present.
fn obo_id_to_iri(obo_id: &str, namespaces: &BTreeMap<String, String>) -> String {
    if obo_id.starts_with("http://") || obo_id.starts_with("https://") {
        return obo_id.to_string();
    }
    if let Some((prefix, local)) = obo_id.split_once(':') {
        if let Some(ns) = namespaces.get(prefix) {
            // When the idspace URL already ends with `PREFIX_` (e.g. `…/obo/GO_`), append
            // only the local segment. Otherwise append `PREFIX_LOCAL` per OBO PURL rules.
            if ns.ends_with(&format!("{prefix}_")) {
                return format!("{ns}{local}");
            }
            return format!("{ns}{prefix}_{local}");
        }
    }
    let normalized = obo_id.replace(':', "_");
    format!("{DEFAULT_OBO_BASE}{normalized}")
}

pub fn parse_obo_file(
    path: &Path,
    ontology_id: &str,
    _content_hash: &str,
    _modified_time: u64,
) -> Result<ParsedOntology> {
    let content = read_to_string_capped(path, MAX_FILE_BYTES)
        .map_err(|e| ParseError::LimitExceeded(format!("{}: {e}", path.display())))?;
    parse_obo_text(path, ontology_id, &content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn parses_minimal_obo() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            "format-version: 1.2\nontology: test\n\n[Term]\nid: TEST:0000001\nname: example term\nis_a: TEST:0000002 ! parent\n"
        )
        .unwrap();
        let parsed = parse_obo_file(file.path(), "doc-1", "hash", 0).unwrap();
        assert_eq!(parsed.entities.len(), 1);
        assert_eq!(parsed.entities[0].obo_id.as_deref(), Some("TEST:0000001"));
        assert_eq!(parsed.entities[0].iri, "http://purl.obolibrary.org/obo/TEST_0000001");
        assert_eq!(parsed.entities[0].labels, vec!["example term"]);
        assert_eq!(parsed.axioms.len(), 1);
        assert_eq!(parsed.axioms[0].predicate, "http://www.w3.org/2000/01/rdf-schema#subClassOf");
        assert_eq!(parsed.axioms[0].object, "http://purl.obolibrary.org/obo/TEST_0000002");
        assert!(parsed.triple_count > 0);
        assert!(!parsed.quads().is_empty(), "OBO must materialize RDF quads");
    }

    #[test]
    fn materializes_all_synonym_scopes_and_definition_in_sparql_quads() {
        let text = "format-version: 1.2\nontology: test\n\n[Term]\n\
id: TEST:0000001\n\
name: test term\n\
synonym: \"exact syn\" EXACT []\n\
synonym: \"broad syn\" BROAD []\n\
synonym: \"narrow syn\" NARROW []\n\
synonym: \"related syn\" RELATED []\n\
def: \"A definition.\" []\n";
        let parsed = parse_obo_text(Path::new("syn.obo"), "doc-1", text).unwrap();
        let predicates: std::collections::BTreeSet<_> =
            parsed.quads().iter().map(|q| q.predicate.as_str().to_string()).collect();
        assert!(predicates.contains("http://www.geneontology.org/formats/oboInOwl#hasExactSynonym"));
        assert!(predicates.contains("http://www.geneontology.org/formats/oboInOwl#hasBroadSynonym"));
        assert!(
            predicates.contains("http://www.geneontology.org/formats/oboInOwl#hasNarrowSynonym")
        );
        assert!(
            predicates.contains("http://www.geneontology.org/formats/oboInOwl#hasRelatedSynonym")
        );
        assert!(predicates.contains("http://purl.obolibrary.org/obo/IAO_0000115"));
        assert!(
            !predicates.contains("http://www.w3.org/2000/01/rdf-schema#comment"),
            "def: must not also emit rdfs:comment (#308)"
        );
        assert!(parsed.annotations.iter().all(|a| a.predicate.contains("://")));
        assert!(parsed.axioms.iter().all(|a| a.predicate.contains("://")));
    }

    #[test]
    fn parses_def_and_synonym_via_fastobo() {
        let text = "format-version: 1.2\nontology: test\n\n[Term]\n\
id: TEST:0000001\n\
name: example\n\
def: \"A definition.\" []\n\
synonym: \"alt label\" EXACT []\n\
comment: \"A true comment.\"\n";
        let parsed = parse_obo_text(Path::new("t.obo"), "doc-1", text).unwrap();
        assert!(
            !parsed.entities[0].comments.iter().any(|c| c.contains("definition")),
            "def text must not land in entity.comments (#308)"
        );
        assert!(parsed.entities[0].comments.iter().any(|c| c.contains("true comment")));
        assert!(parsed.annotations.iter().any(|a| {
            a.predicate == "http://purl.obolibrary.org/obo/IAO_0000115"
                && a.object.contains("definition")
        }));
        assert!(parsed.annotations.iter().any(|a| {
            a.predicate == "http://www.geneontology.org/formats/oboInOwl#hasExactSynonym"
        }));
    }

    #[test]
    fn property_value_stores_target_not_display() {
        let text = "format-version: 1.2\n\
idspace: dc http://purl.org/dc/elements/1.1/\n\
idspace: ORCID http://orcid.org/\n\n\
[Term]\n\
id: TEST:0000001\n\
name: example\n\
property_value: dc:creator ORCID:0000-0002-3947-4444\n\
property_value: shoe_size \"8\" xsd:positiveInteger\n";
        let parsed = parse_obo_text(Path::new("pv.obo"), "doc-1", text).unwrap();
        let resource = parsed
            .annotations
            .iter()
            .find(|a| {
                a.predicate == "http://purl.org/dc/elements/1.1/dc_creator"
                    || a.predicate == "http://purl.org/dc/elements/1.1/creator"
            })
            .expect("dc:creator annotation");
        assert!(
            !resource.object.contains("dc:creator"),
            "object must not duplicate the property Display: {}",
            resource.object
        );
        assert!(
            resource.object.contains("0000-0002-3947-4444"),
            "object should be the ORCID target: {}",
            resource.object
        );
        let literal =
            parsed.annotations.iter().find(|a| a.object == "8").expect("shoe_size literal");
        assert!(!literal.object.contains("shoe_size"));
        let predicates: std::collections::BTreeSet<_> =
            parsed.quads().iter().map(|q| q.predicate.as_str().to_string()).collect();
        assert!(
            predicates.iter().any(|p| p.contains("creator") || p.contains("shoe_size")),
            "property_value must appear in SPARQL quads"
        );
    }

    #[test]
    fn idspace_overrides_default_base() {
        let text = "format-version: 1.2\n\
idspace: GO http://purl.obolibrary.org/obo/GO_\n\n\
[Term]\n\
id: GO:0000001\n\
name: mitochondrion\n";
        let parsed = parse_obo_text(Path::new("go.obo"), "doc-1", text).unwrap();
        assert_eq!(parsed.entities[0].iri, "http://purl.obolibrary.org/obo/GO_0000001");
    }

    #[test]
    fn idspace_standard_obo_foundry_base_normalizes_colon_to_underscore() {
        let text = "format-version: 1.2\n\
idspace: GO http://purl.obolibrary.org/obo/\n\n\
[Term]\n\
id: GO:0000001\n\
name: mitochondrion\n\
is_a: GO:0000002 ! parent\n";
        let parsed = parse_obo_text(Path::new("go.obo"), "doc-1", text).unwrap();
        assert_eq!(parsed.entities[0].iri, "http://purl.obolibrary.org/obo/GO_0000001");
        assert_eq!(parsed.axioms[0].object, "http://purl.obolibrary.org/obo/GO_0000002");
    }

    #[test]
    fn rejects_oversized_obo_source_text() {
        let huge = "x".repeat((strixonomy_core::MAX_FILE_BYTES as usize) + 1);
        let err = crate::rdf::parse_ontology_text(
            Path::new("big.obo"),
            strixonomy_core::OntologyFormat::Obo,
            "doc-1",
            &huge,
            huge.as_bytes(),
        )
        .unwrap_err();
        assert!(err.to_string().contains("exceeds"));
    }
}
