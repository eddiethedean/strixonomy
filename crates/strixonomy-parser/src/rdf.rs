use crate::vocab::{Rdf, Rdfs, OWL};
use oxigraph::io::{RdfFormat, RdfParseError, RdfParser, RdfSerializer};
use oxigraph::model::{GraphName, Literal, NamedNode, Quad, Subject, Term};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;
use strixonomy_core::{
    limits::{MAX_FILE_BYTES, MAX_TRIPLES_PER_FILE},
    read_file_capped, Annotation, Axiom, Entity, EntityKind, Import, Namespace, OntologyFormat,
    ParseStatus, SourceLocation, AXIOM_KIND_SUB_CLASS_OF,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("RDF parse error: {0}")]
    Rdf(String),

    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("limit exceeded: {0}")]
    LimitExceeded(String),

    #[error("invalid UTF-8: {0}")]
    InvalidUtf8(String),
}

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParsedOntology {
    pub ontology_id: String,
    pub base_iri: Option<String>,
    pub version_iri: Option<String>,
    pub imports: Vec<String>,
    pub namespaces: BTreeMap<String, String>,
    pub entities: Vec<Entity>,
    pub annotations: Vec<Annotation>,
    pub axioms: Vec<Axiom>,
    pub namespace_rows: Vec<Namespace>,
    pub import_rows: Vec<Import>,
    pub parse_status: ParseStatus,
    pub parse_message: Option<String>,
    pub parse_error_location: Option<SourceLocation>,
    pub triple_count: usize,
    quads: Vec<Quad>,
}

impl ParsedOntology {
    /// RDF quads for catalog indexing only — not a stable public API.
    #[doc(hidden)]
    pub fn quads(&self) -> &[Quad] {
        &self.quads
    }
}

pub fn parse_ontology_file(
    path: &Path,
    format: OntologyFormat,
    ontology_id: &str,
    content_hash: &str,
    modified_time: u64,
) -> Result<ParsedOntology> {
    if format == OntologyFormat::Obo {
        return crate::obo::parse_obo_file(path, ontology_id, content_hash, modified_time);
    }
    if format == OntologyFormat::OwlXml {
        let content = read_file_capped(path, MAX_FILE_BYTES)
            .map_err(|e| ParseError::LimitExceeded(e.to_string()))?;
        let _source_text = String::from_utf8(content).map_err(|e| {
            ParseError::InvalidUtf8(format!("invalid UTF-8 in {}: {e}", path.display()))
        })?;
        return Ok(empty_parsed_ontology(ontology_id));
    }
    let _ = (content_hash, modified_time);
    let content = read_file_capped(path, MAX_FILE_BYTES)
        .map_err(|e| ParseError::LimitExceeded(e.to_string()))?;
    let source_text = String::from_utf8(content).map_err(|e| {
        ParseError::InvalidUtf8(format!("invalid UTF-8 in {}: {e}", path.display()))
    })?;
    parse_ontology_text(path, format, ontology_id, &source_text, source_text.as_bytes())
}

/// Parse ontology source text (used for LSP open buffers and file parsing).
pub fn parse_ontology_text(
    path: &Path,
    format: OntologyFormat,
    ontology_id: &str,
    source_text: &str,
    raw_bytes: &[u8],
) -> Result<ParsedOntology> {
    if source_text.len() as u64 > MAX_FILE_BYTES || raw_bytes.len() as u64 > MAX_FILE_BYTES {
        return Err(ParseError::LimitExceeded(format!(
            "source exceeds {MAX_FILE_BYTES} bytes: {}",
            path.display()
        )));
    }
    if format == OntologyFormat::Obo {
        return crate::obo::parse_obo_text(path, ontology_id, source_text);
    }
    if format == OntologyFormat::OwlXml {
        return Ok(empty_parsed_ontology(ontology_id));
    }
    let rdf_format = to_rdf_format(format, path)?;

    let mut quads = Vec::new();
    let mut parse_message = None;
    let mut parse_error_location = None;
    let mut parse_status = ParseStatus::Ok;

    let parser = RdfParser::from_format(rdf_format);
    for quad in parser.for_reader(raw_bytes) {
        match quad {
            Ok(q) => {
                if quads.len() >= MAX_TRIPLES_PER_FILE {
                    return Err(ParseError::LimitExceeded(format!(
                        "file exceeds {MAX_TRIPLES_PER_FILE} triples: {}",
                        path.display()
                    )));
                }
                quads.push(q);
            }
            Err(e) => {
                parse_status = ParseStatus::Error;
                parse_message = Some(format_parse_error(&e));
                parse_error_location = extract_parse_error_location(&e, source_text);
                break;
            }
        }
    }

    // Keep partial quads on error so a trailing syntax fault does not wipe the catalog.
    if parse_status == ParseStatus::Error && quads.is_empty() {
        return Ok(empty_result(
            ontology_id,
            parse_status,
            parse_message,
            parse_error_location,
            BTreeMap::new(),
        ));
    }

    let mut namespaces =
        if format == OntologyFormat::TriG { extract_prefixes(&quads) } else { BTreeMap::new() };
    namespaces.extend(extract_declared_prefixes(source_text, format));
    if namespaces.is_empty() {
        namespaces.insert("".to_string(), default_base_iri(path));
    }

    let mut builder = OntologyBuilder::new(ontology_id.to_string(), namespaces.clone());
    for quad in &quads {
        builder.ingest_quad(quad);
    }
    builder.finish(parse_status, parse_message, parse_error_location, source_text, quads)
}

fn empty_parsed_ontology(ontology_id: &str) -> ParsedOntology {
    ParsedOntology {
        ontology_id: ontology_id.to_string(),
        base_iri: None,
        version_iri: None,
        imports: Vec::new(),
        namespaces: BTreeMap::new(),
        entities: Vec::new(),
        annotations: Vec::new(),
        axioms: Vec::new(),
        namespace_rows: Vec::new(),
        import_rows: Vec::new(),
        parse_status: ParseStatus::Ok,
        parse_message: None,
        parse_error_location: None,
        triple_count: 0,
        quads: Vec::new(),
    }
}

fn to_rdf_format(format: OntologyFormat, path: &Path) -> Result<RdfFormat> {
    match format {
        OntologyFormat::Turtle => Ok(RdfFormat::Turtle),
        OntologyFormat::RdfXml | OntologyFormat::Owl => Ok(RdfFormat::RdfXml),
        OntologyFormat::JsonLd => Ok(RdfFormat::JsonLd { profile: Default::default() }),
        OntologyFormat::NTriples => Ok(RdfFormat::NTriples),
        OntologyFormat::NQuads => Ok(RdfFormat::NQuads),
        OntologyFormat::TriG => Ok(RdfFormat::TriG),
        OntologyFormat::Obo => {
            Err(ParseError::UnsupportedFormat("OBO format must use parse_obo_text".to_string()))
        }
        OntologyFormat::OwlXml => {
            Err(ParseError::UnsupportedFormat("OWL/XML must use parse_ontology_text".to_string()))
        }
        OntologyFormat::Unknown => Err(ParseError::UnsupportedFormat(path.display().to_string())),
    }
}

fn format_parse_error(error: &RdfParseError) -> String {
    error.to_string()
}

fn extract_parse_error_location(
    error: &RdfParseError,
    _source_text: &str,
) -> Option<SourceLocation> {
    let msg = error.to_string();
    let line = msg.split_whitespace().collect::<Vec<_>>().windows(2).find_map(|w| {
        if w[0].eq_ignore_ascii_case("line") {
            w[1].trim_end_matches(':').parse().ok()
        } else {
            None
        }
    });
    let column = msg.split_whitespace().collect::<Vec<_>>().windows(2).find_map(|w| {
        if w[0].eq_ignore_ascii_case("column") || w[0].eq_ignore_ascii_case("col") {
            w[1].trim_end_matches(':').parse().ok()
        } else {
            None
        }
    });
    if line.is_some() || column.is_some() {
        Some(SourceLocation { line, column, ..Default::default() })
    } else {
        None
    }
}

fn default_base_iri(path: &Path) -> String {
    strixonomy_core::file_uri_for_path(path)
}

fn extract_declared_prefixes(
    source_text: &str,
    format: OntologyFormat,
) -> BTreeMap<String, String> {
    let mut prefixes = BTreeMap::new();

    if matches!(format, OntologyFormat::Turtle | OntologyFormat::TriG) {
        for line in source_text.lines() {
            let trimmed = line.trim();
            let rest = trimmed
                .strip_prefix("@prefix ")
                .or_else(|| trimmed.strip_prefix("@PREFIX "))
                .or_else(|| trimmed.strip_prefix("PREFIX "));
            let Some(rest) = rest else {
                continue;
            };
            let Some((prefix_part, iri_part)) = rest.split_once('<') else {
                continue;
            };
            let prefix = prefix_part.trim().trim_end_matches(':');
            let Some(iri) = iri_part.split('>').next() else {
                continue;
            };
            prefixes.insert(prefix.to_string(), iri.to_string());
        }
        return prefixes;
    }

    if matches!(format, OntologyFormat::RdfXml | OntologyFormat::Owl) {
        static XMLNS_ATTR: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            regex::Regex::new(r#"xmlns(?::([A-Za-z][\w-]*))?="([^"]+)""#).expect("xmlns regex")
        });
        for cap in XMLNS_ATTR.captures_iter(source_text) {
            let prefix = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let iri = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            if !iri.is_empty() {
                prefixes.insert(prefix.to_string(), iri.to_string());
            }
        }
    }

    prefixes
}

fn extract_prefixes(quads: &[Quad]) -> BTreeMap<String, String> {
    let mut prefixes = BTreeMap::new();
    for quad in quads {
        if let oxigraph::model::GraphNameRef::NamedNode(graph) = quad.graph_name.as_ref() {
            let iri = graph.as_str();
            if let Some((prefix, _)) = iri.rsplit_once('#') {
                if let Some((p, _)) = prefix.rsplit_once('/') {
                    prefixes
                        .entry(short_name_from_iri(p))
                        .or_insert_with(|| format!("{}/", prefix.trim_end_matches('#')));
                }
            }
        }
    }
    prefixes
}

pub(crate) fn assemble_parsed_ontology(
    ontology_id: &str,
    base_iri: Option<String>,
    namespaces: BTreeMap<String, String>,
    entities: Vec<Entity>,
    annotations: Vec<Annotation>,
    axioms: Vec<Axiom>,
) -> ParsedOntology {
    let namespace_rows: Vec<Namespace> = namespaces
        .iter()
        .map(|(prefix, iri)| Namespace {
            prefix: prefix.clone(),
            iri: iri.clone(),
            ontology_id: ontology_id.to_string(),
        })
        .collect();
    let quads = materialize_catalog_quads(&entities, &annotations, &axioms);
    let triple_count = quads.len().max(entities.len() + annotations.len() + axioms.len());
    ParsedOntology {
        ontology_id: ontology_id.to_string(),
        base_iri,
        version_iri: None,
        imports: Vec::new(),
        namespaces: namespaces.clone(),
        entities,
        annotations,
        axioms,
        namespace_rows,
        import_rows: Vec::new(),
        parse_status: ParseStatus::Ok,
        parse_message: None,
        parse_error_location: None,
        triple_count,
        quads,
    }
}

/// Build RDF quads from catalog entities/axioms so SPARQL sees OBO (and similar) content.
fn materialize_catalog_quads(
    entities: &[Entity],
    annotations: &[Annotation],
    axioms: &[Axiom],
) -> Vec<Quad> {
    let mut quads = Vec::new();
    for entity in entities {
        let Some(subject) = named_node(&entity.iri) else {
            continue;
        };
        let type_iri = match entity.kind {
            EntityKind::Class => OWL::class(),
            EntityKind::ObjectProperty => OWL::object_property(),
            EntityKind::DataProperty => OWL::datatype_property(),
            EntityKind::AnnotationProperty => OWL::annotation_property(),
            EntityKind::Individual => OWL::named_individual(),
            EntityKind::Datatype => Rdfs::datatype(),
            EntityKind::Ontology => OWL::ontology(),
            EntityKind::Other => continue,
        };
        quads.push(Quad::new(
            subject.clone(),
            Rdf::type_().into_owned(),
            type_iri.into_owned(),
            GraphName::DefaultGraph,
        ));
        for label in &entity.labels {
            quads.push(Quad::new(
                subject.clone(),
                Rdfs::label().into_owned(),
                Literal::new_simple_literal(label),
                GraphName::DefaultGraph,
            ));
        }
        for comment in &entity.comments {
            quads.push(Quad::new(
                subject.clone(),
                Rdfs::comment().into_owned(),
                Literal::new_simple_literal(comment),
                GraphName::DefaultGraph,
            ));
        }
        if entity.deprecated {
            quads.push(Quad::new(
                subject,
                OWL::deprecated().into_owned(),
                Literal::from(true),
                GraphName::DefaultGraph,
            ));
        }
    }
    for axiom in axioms {
        if axiom.axiom_kind != AXIOM_KIND_SUB_CLASS_OF {
            continue;
        }
        let (Some(s), Some(o)) = (named_node(&axiom.subject), named_node(&axiom.object)) else {
            continue;
        };
        quads.push(Quad::new(s, Rdfs::sub_class_of().into_owned(), o, GraphName::DefaultGraph));
    }
    for ann in annotations {
        let Some(s) = named_node(&ann.subject) else {
            continue;
        };
        let pred_iri = expand_annotation_predicate(&ann.predicate);
        let Some(p) = pred_iri.as_deref().and_then(named_node) else {
            continue;
        };
        if let Some(o) = named_node(&ann.object) {
            quads.push(Quad::new(s, p, o, GraphName::DefaultGraph));
        } else {
            quads.push(Quad::new(
                s,
                p,
                Literal::new_simple_literal(&ann.object),
                GraphName::DefaultGraph,
            ));
        }
    }
    quads
}

fn named_node(iri: &str) -> Option<NamedNode> {
    NamedNode::new(iri).ok()
}

const OBO_INOWL_NS: &str = "http://www.geneontology.org/formats/oboInOwl#";
const OBO_PURL_NS: &str = "http://purl.obolibrary.org/obo/";

/// Expand OBO-style annotation CURIEs used by the OBO parser.
fn expand_annotation_predicate(pred: &str) -> Option<String> {
    if pred.contains("://") {
        return Some(pred.to_string());
    }
    let (prefix, local) = pred.split_once(':')?;
    match prefix {
        "obo" => Some(expand_obo_curie(local)),
        _ => None,
    }
}

fn expand_obo_curie(local: &str) -> String {
    match local {
        "hasExactSynonym" | "hasBroadSynonym" | "hasNarrowSynonym" | "hasRelatedSynonym"
        | "hasDbXref" => format!("{OBO_INOWL_NS}{local}"),
        // OBO term IDs (e.g. IAO_0000115) use the PURL namespace.
        _ if local.contains('_') => format!("{OBO_PURL_NS}{local}"),
        _ => format!("{OBO_INOWL_NS}{local}"),
    }
}

/// Serialize RDF quads as Turtle (used to bridge OBO catalog quads into OntoLogos).
pub fn serialize_quads_turtle(quads: &[Quad]) -> Result<String> {
    let mut serializer = RdfSerializer::from_format(RdfFormat::Turtle).for_writer(Vec::new());
    for quad in quads {
        serializer.serialize_quad(quad).map_err(|e| ParseError::Rdf(e.to_string()))?;
    }
    let bytes = serializer.finish().map_err(|e| ParseError::Rdf(e.to_string()))?;
    String::from_utf8(bytes).map_err(|e| ParseError::Rdf(e.to_string()))
}

fn empty_result(
    ontology_id: &str,
    parse_status: ParseStatus,
    parse_message: Option<String>,
    parse_error_location: Option<SourceLocation>,
    namespaces: BTreeMap<String, String>,
) -> ParsedOntology {
    ParsedOntology {
        ontology_id: ontology_id.to_string(),
        base_iri: namespaces.values().next().cloned(),
        version_iri: None,
        imports: Vec::new(),
        namespaces: namespaces.clone(),
        entities: Vec::new(),
        annotations: Vec::new(),
        axioms: Vec::new(),
        namespace_rows: namespaces
            .into_iter()
            .map(|(prefix, iri)| Namespace { prefix, iri, ontology_id: ontology_id.to_string() })
            .collect(),
        import_rows: Vec::new(),
        parse_status,
        parse_message,
        parse_error_location,
        triple_count: 0,
        quads: Vec::new(),
    }
}

struct EntityState {
    kind: EntityKind,
    labels: Vec<String>,
    comments: Vec<String>,
    deprecated: bool,
    types: BTreeSet<String>,
}

struct OntologyBuilder {
    ontology_id: String,
    namespaces: BTreeMap<String, String>,
    entities: HashMap<String, EntityState>,
    annotations: Vec<Annotation>,
    axioms: Vec<Axiom>,
    imports: BTreeSet<String>,
    ontology_iris: BTreeSet<String>,
    version_iris: BTreeSet<String>,
    triple_count: usize,
    axiom_counter: usize,
}

impl OntologyBuilder {
    fn new(ontology_id: String, namespaces: BTreeMap<String, String>) -> Self {
        Self {
            ontology_id,
            namespaces,
            entities: HashMap::new(),
            annotations: Vec::new(),
            axioms: Vec::new(),
            imports: BTreeSet::new(),
            ontology_iris: BTreeSet::new(),
            version_iris: BTreeSet::new(),
            triple_count: 0,
            axiom_counter: 0,
        }
    }

    fn ingest_quad(&mut self, quad: &Quad) {
        self.triple_count += 1;
        let subject = subject_to_string(&quad.subject);
        let predicate = quad.predicate.as_str().to_string();
        let object = term_to_string(&quad.object);

        if quad.predicate == Rdf::type_() {
            if let Term::NamedNode(node) = &quad.object {
                let type_iri = node.as_str();
                if type_iri == OWL::ontology().as_str() {
                    self.ontology_iris.insert(subject.clone());
                }
                let kind = entity_kind_for_type(type_iri);
                if kind != EntityKind::Other {
                    let entry =
                        self.entities.entry(subject.clone()).or_insert_with(|| EntityState {
                            kind,
                            labels: Vec::new(),
                            comments: Vec::new(),
                            deprecated: false,
                            types: BTreeSet::new(),
                        });
                    entry.types.insert(type_iri.to_string());
                    if entry.kind == EntityKind::Other
                        || kind_priority(kind) > kind_priority(entry.kind)
                    {
                        entry.kind = kind;
                    }
                }
            }
            return;
        }

        if quad.predicate == OWL::imports() {
            self.imports.insert(object.clone());
            return;
        }

        if quad.predicate == OWL::version_iri() {
            self.version_iris.insert(object.clone());
            return;
        }

        if quad.predicate == Rdfs::label() {
            if let Some(entity) = self.entities.get_mut(&subject) {
                entity.labels.push(object.clone());
            } else {
                self.entities.entry(subject.clone()).or_insert_with(|| EntityState {
                    kind: EntityKind::Other,
                    labels: vec![object.clone()],
                    comments: Vec::new(),
                    deprecated: false,
                    types: BTreeSet::new(),
                });
            }
            self.annotations.push(Annotation {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: object.clone(),
                ontology_id: self.ontology_id.clone(),
                source_location: SourceLocation::default(),
            });
            return;
        }

        if quad.predicate == Rdfs::comment() {
            if let Some(entity) = self.entities.get_mut(&subject) {
                entity.comments.push(object.clone());
            } else {
                self.entities.entry(subject.clone()).or_insert_with(|| EntityState {
                    kind: EntityKind::Other,
                    labels: Vec::new(),
                    comments: vec![object.clone()],
                    deprecated: false,
                    types: BTreeSet::new(),
                });
            }
            self.annotations.push(Annotation {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: object.clone(),
                ontology_id: self.ontology_id.clone(),
                source_location: SourceLocation::default(),
            });
            return;
        }

        if quad.predicate == OWL::deprecated() {
            let entry = self.entities.entry(subject.clone()).or_insert_with(|| EntityState {
                kind: EntityKind::Other,
                labels: Vec::new(),
                comments: Vec::new(),
                deprecated: false,
                types: BTreeSet::new(),
            });
            entry.deprecated = strixonomy_core::parse_boolean_literal(&object).unwrap_or(false);
            return;
        }

        if quad.predicate == OWL::same_as() {
            self.annotations.push(Annotation {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: object.clone(),
                ontology_id: self.ontology_id.clone(),
                source_location: SourceLocation::default(),
            });
            return;
        }

        if quad.predicate == Rdfs::sub_class_of() {
            self.axiom_counter += 1;
            self.axioms.push(Axiom {
                id: format!("{}#axiom-{}", self.ontology_id, self.axiom_counter),
                ontology_id: self.ontology_id.clone(),
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: object.clone(),
                axiom_kind: AXIOM_KIND_SUB_CLASS_OF.to_string(),
                source_location: SourceLocation::default(),
                annotations: Vec::new(),
            });
        }
    }

    fn finish(
        self,
        parse_status: ParseStatus,
        parse_message: Option<String>,
        parse_error_location: Option<SourceLocation>,
        source_text: &str,
        quads: Vec<Quad>,
    ) -> Result<ParsedOntology> {
        let base_iri = self
            .ontology_iris
            .iter()
            .next()
            .cloned()
            .or_else(|| self.namespaces.get("").cloned())
            .or_else(|| self.namespaces.values().next().cloned());

        let version_iri = self.version_iris.iter().next().cloned();

        let ontology_id = if let Some(iri) = self.ontology_iris.iter().next() {
            iri.clone()
        } else {
            self.ontology_id.clone()
        };

        let mut entities = Vec::new();
        for (iri, state) in &self.entities {
            if state.kind == EntityKind::Other {
                continue;
            }
            let short_name = short_name_from_iri(iri);
            entities.push(Entity {
                iri: iri.clone(),
                short_name: short_name.clone(),
                kind: state.kind,
                ontology_id: ontology_id.clone(),
                source_location: find_entity_source_location(
                    source_text,
                    iri,
                    &short_name,
                    &self.namespaces,
                ),
                labels: state.labels.clone(),
                comments: state.comments.clone(),
                deprecated: state.deprecated,
                obo_id: None,
                characteristics: Default::default(),
            });
        }
        entities.sort_by(|a, b| a.iri.cmp(&b.iri));

        let namespace_rows = self
            .namespaces
            .iter()
            .map(|(prefix, iri)| Namespace {
                prefix: prefix.clone(),
                iri: iri.clone(),
                ontology_id: ontology_id.clone(),
            })
            .collect();

        let import_rows = self
            .imports
            .iter()
            .map(|import_iri| Import {
                ontology_id: ontology_id.clone(),
                import_iri: import_iri.clone(),
            })
            .collect();

        let mut annotations = self.annotations;
        for ann in &mut annotations {
            ann.ontology_id = ontology_id.clone();
        }
        let mut axioms = self.axioms;
        for axiom in &mut axioms {
            axiom.ontology_id = ontology_id.clone();
        }

        Ok(ParsedOntology {
            ontology_id,
            base_iri,
            version_iri,
            imports: self.imports.into_iter().collect(),
            namespaces: self.namespaces.clone(),
            entities,
            annotations,
            axioms,
            namespace_rows,
            import_rows,
            parse_status,
            parse_message,
            parse_error_location,
            triple_count: self.triple_count,
            quads,
        })
    }
}

fn entity_kind_for_type(type_iri: &str) -> EntityKind {
    match type_iri {
        t if t == OWL::class().as_str() || t == Rdfs::class().as_str() => EntityKind::Class,
        t if t == OWL::object_property().as_str() => EntityKind::ObjectProperty,
        t if t == OWL::datatype_property().as_str() => EntityKind::DataProperty,
        t if t == OWL::annotation_property().as_str() => EntityKind::AnnotationProperty,
        t if t == OWL::named_individual().as_str() => EntityKind::Individual,
        t if t == Rdfs::datatype().as_str() => EntityKind::Datatype,
        t if t == OWL::ontology().as_str() => EntityKind::Ontology,
        _ => EntityKind::Other,
    }
}

fn kind_priority(kind: EntityKind) -> u8 {
    match kind {
        EntityKind::Class => 5,
        EntityKind::ObjectProperty => 5,
        EntityKind::DataProperty => 5,
        EntityKind::AnnotationProperty => 5,
        EntityKind::Individual => 4,
        EntityKind::Datatype => 4,
        EntityKind::Ontology => 3,
        EntityKind::Other => 0,
    }
}

fn subject_to_string(subject: &Subject) -> String {
    match subject {
        Subject::NamedNode(node) => node.as_str().to_string(),
        Subject::BlankNode(node) => format!("_:{}", node.as_str()),
        #[allow(unreachable_patterns)]
        _ => subject.to_string(),
    }
}

fn term_to_string(term: &Term) -> String {
    match term {
        Term::NamedNode(node) => node.as_str().to_string(),
        Term::BlankNode(node) => format!("_:{}", node.as_str()),
        Term::Literal(lit) => lit.value().to_string(),
        #[allow(unreachable_patterns)]
        _ => term.to_string(),
    }
}

fn find_entity_source_location(
    source_text: &str,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> SourceLocation {
    let mut needles = vec![iri.to_string(), format!("<{iri}>"), format!(":{short_name}")];
    for (prefix, ns) in namespaces {
        if iri.starts_with(ns) && !prefix.is_empty() {
            needles.push(format!("{prefix}:{short_name}"));
        }
    }
    for line in source_text.lines() {
        let trimmed = line.trim();
        if !(trimmed.starts_with("@prefix")
            || trimmed.starts_with("@PREFIX")
            || trimmed.starts_with("PREFIX "))
        {
            continue;
        }
        let prefix_kw_len = if trimmed.to_ascii_lowercase().starts_with("@prefix ") {
            "@prefix ".len()
        } else if trimmed.to_ascii_lowercase().starts_with("prefix ") {
            "PREFIX ".len()
        } else {
            continue;
        };
        if let Some(colon) = trimmed.find(':') {
            if colon < prefix_kw_len {
                continue;
            }
            let prefix = trimmed[prefix_kw_len..colon].trim();
            let prefix = prefix.trim_start_matches('@');
            if let (Some(start), Some(end)) = (line.find('<'), line.find('>')) {
                if start < end {
                    let ns = &line[start + 1..end];
                    if iri.starts_with(ns) && !prefix.is_empty() {
                        needles.push(format!("{prefix}:{short_name}"));
                    }
                }
            }
        }
    }

    for (line_idx, line) in source_text.lines().enumerate() {
        for needle in &needles {
            if let Some(col) = line.find(needle) {
                return SourceLocation {
                    line: Some((line_idx + 1) as u64),
                    column: Some(col as u64),
                    start_byte: None,
                    end_byte: None,
                };
            }
        }
    }

    SourceLocation::default()
}

fn short_name_from_iri(iri: &str) -> String {
    if let Some((_, name)) = iri.rsplit_once('#') {
        return name.to_string();
    }
    if let Some((_, name)) = iri.rsplit_once('/') {
        return name.to_string();
    }
    iri.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn rejects_oversized_source_text() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("huge.ttl");
        let oversized = "x".repeat((MAX_FILE_BYTES + 1) as usize);
        let err = parse_ontology_text(
            &path,
            OntologyFormat::Turtle,
            "doc-1",
            &oversized,
            oversized.as_bytes(),
        )
        .unwrap_err();
        assert!(matches!(err, ParseError::LimitExceeded(_)));
    }

    #[test]
    fn rejects_invalid_utf8_ontology_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.ttl");
        std::fs::write(&path, b"@prefix ex: <http://ex/> .\n\xff\xfe\n").unwrap();
        let err = parse_ontology_file(&path, OntologyFormat::Turtle, "doc-1", "h", 0).unwrap_err();
        assert!(matches!(err, ParseError::InvalidUtf8(_)));
    }

    #[test]
    fn parses_simple_turtle_ontology() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.ttl");
        let mut f = fs::File::create(&path).unwrap();
        writeln!(
            f,
            r#"@prefix ex: <http://example.org/test#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

<http://example.org/test> a owl:Ontology .

ex:Person a owl:Class ;
    rdfs:label "Person" ;
    rdfs:comment "A human being" .

ex:knows a owl:ObjectProperty ;
    rdfs:label "knows" .
"#
        )
        .unwrap();

        let parsed =
            parse_ontology_file(&path, OntologyFormat::Turtle, "doc-1", "hash", 0).unwrap();

        assert_eq!(parsed.parse_status, ParseStatus::Ok);

        let person = parsed
            .entities
            .iter()
            .find(|e| e.iri == "http://example.org/test#Person")
            .expect("Person entity");
        assert_eq!(person.kind, EntityKind::Class);
        assert_eq!(person.labels, vec!["Person".to_string()]);
        assert!(person.source_location.line.is_some());

        let knows = parsed
            .entities
            .iter()
            .find(|e| e.iri == "http://example.org/test#knows")
            .expect("knows property");
        assert_eq!(knows.kind, EntityKind::ObjectProperty);
        assert_eq!(knows.labels, vec!["knows".to_string()]);
    }

    #[test]
    fn extracts_turtle_prefix_declarations() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.ttl");
        let mut f = fs::File::create(&path).unwrap();
        writeln!(
            f,
            r#"@prefix ex: <http://example.org/test#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<http://example.org/test> a owl:Ontology .
ex:Person a owl:Class .
"#
        )
        .unwrap();

        let parsed =
            parse_ontology_file(&path, OntologyFormat::Turtle, "doc-1", "hash", 0).unwrap();

        assert_eq!(parsed.base_iri.as_deref(), Some("http://example.org/test"));
        assert_eq!(
            parsed.namespaces.get("ex").map(String::as_str),
            Some("http://example.org/test#")
        );
    }

    #[test]
    fn trailing_parse_error_keeps_prior_entities() {
        let ttl = r#"@prefix ex: <http://example.org/test#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class ;
    rdfs:label "Person" .

ex:Broken a owl:Class ; this is not valid turtle
"#;
        let parsed = parse_ontology_text(
            Path::new("partial.ttl"),
            OntologyFormat::Turtle,
            "doc-1",
            ttl,
            ttl.as_bytes(),
        )
        .expect("partial parse");
        assert_eq!(parsed.parse_status, ParseStatus::Error);
        assert!(
            parsed.entities.iter().any(|e| e.iri == "http://example.org/test#Person"),
            "entities parsed before the fault must be retained"
        );
        assert!(!parsed.quads().is_empty());
    }

    #[test]
    fn default_base_iri_uses_file_uri_helper() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bare.ttl");
        fs::write(&path, "<> a <http://www.w3.org/2002/07/owl#Ontology> .\n").unwrap();
        let uri = default_base_iri(&path);
        assert!(uri.starts_with("file://"), "got {uri}");
        assert!(!uri.contains('\\'), "raw backslash must not appear: {uri}");
        assert_eq!(uri, strixonomy_core::file_uri_for_path(&path));
    }
}
