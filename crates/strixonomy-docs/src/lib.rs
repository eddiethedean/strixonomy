//! Documentation export for indexed ontology workspaces.

use minijinja::{context, AutoEscape, Environment};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use strixonomy_catalog::OntologyCatalog;
use strixonomy_core::{document_for_entity, document_matches_ontology_id, EntityKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Markdown,
    Html,
}

#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub output_dir: PathBuf,
    pub format: ExportFormat,
    pub ontology_id: Option<String>,
}

impl ExportOptions {
    pub fn markdown(output_dir: impl Into<PathBuf>) -> Self {
        Self { output_dir: output_dir.into(), format: ExportFormat::Markdown, ontology_id: None }
    }

    pub fn html(output_dir: impl Into<PathBuf>) -> Self {
        Self { output_dir: output_dir.into(), format: ExportFormat::Html, ontology_id: None }
    }

    pub fn with_ontology_id(mut self, id: impl Into<String>) -> Self {
        self.ontology_id = Some(id.into());
        self
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("template error: {0}")]
    Template(#[from] minijinja::Error),
}

pub type Result<T> = std::result::Result<T, ExportError>;

#[derive(Debug, Clone, serde::Serialize)]
struct EntityDoc {
    iri: String,
    short_name: String,
    kind: String,
    labels: Vec<String>,
    comments: Vec<String>,
    parents: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct OntologyDoc {
    id: String,
    slug: String,
    path: String,
    imports: Vec<String>,
    entities: Vec<EntityDoc>,
}

pub fn export_workspace(catalog: &OntologyCatalog, options: ExportOptions) -> Result<()> {
    fs::create_dir_all(&options.output_dir)?;

    let hierarchy = catalog.class_hierarchy();
    let mut ontologies: Vec<OntologyDoc> = Vec::new();
    let mut used_slugs = BTreeSet::new();

    for doc in &catalog.data().documents {
        if let Some(filter) = &options.ontology_id {
            if !document_matches_ontology_id(filter, doc) {
                continue;
            }
        }
        let mut entities = Vec::new();
        for entity in &catalog.data().entities {
            if document_for_entity(&catalog.data().documents, entity).is_none_or(|d| d.id != doc.id)
            {
                continue;
            }
            let parents = hierarchy.parents.get(&entity.iri).cloned().unwrap_or_default();
            entities.push(EntityDoc {
                iri: entity.iri.clone(),
                short_name: entity.short_name.clone(),
                kind: entity.kind.as_str().to_string(),
                labels: entity.labels.clone(),
                comments: entity.comments.clone(),
                parents,
            });
        }
        entities.sort_by(|a, b| a.short_name.cmp(&b.short_name));
        ontologies.push(OntologyDoc {
            id: doc.id.clone(),
            slug: allocate_slug(&doc.id, &mut used_slugs),
            path: doc.path.display().to_string(),
            imports: doc.imports.clone(),
            entities,
        });
    }

    let index_name = match options.format {
        ExportFormat::Markdown => "index.md",
        ExportFormat::Html => "index.html",
    };
    let index_path = options.output_dir.join(index_name);
    let index_body = render_index(&ontologies, &hierarchy, catalog, options.format)?;
    fs::write(index_path, index_body)?;

    for ont in &ontologies {
        let file_name = match options.format {
            ExportFormat::Markdown => format!("{}.md", ont.slug),
            ExportFormat::Html => format!("{}.html", ont.slug),
        };
        let body = render_ontology(ont, options.format)?;
        fs::write(options.output_dir.join(file_name), body)?;
    }

    Ok(())
}

fn slugify(iri: &str) -> String {
    let slug: String = iri
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .trim_matches('_')
        .chars()
        .take(80)
        .collect();
    if slug.is_empty() {
        "ontology".to_string()
    } else {
        slug
    }
}

/// Stable 8-hex FNV-1a tag so colliding readable slugs stay unique (#24).
fn iri_tag(iri: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in iri.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")[..8].to_string()
}

/// Allocate a filesystem-safe slug unique within one export run.
fn allocate_slug(iri: &str, used: &mut BTreeSet<String>) -> String {
    let base = slugify(iri);
    if used.insert(base.clone()) {
        return base;
    }
    let tag = iri_tag(iri);
    let truncated: String = base.chars().take(71).collect();
    let mut candidate = format!("{truncated}_{tag}");
    let mut n = 2u32;
    while !used.insert(candidate.clone()) {
        candidate = format!("{truncated}_{tag}_{n}");
        n = n.saturating_add(1);
    }
    candidate
}

fn render_index(
    ontologies: &[OntologyDoc],
    hierarchy: &strixonomy_catalog::ClassHierarchy,
    catalog: &OntologyCatalog,
    format: ExportFormat,
) -> Result<String> {
    match format {
        ExportFormat::Markdown => {
            let mut md = String::from("# Ontology documentation\n\n");
            for ont in ontologies {
                md.push_str(&format!(
                    "- [{}]({}.md) — {} entities, {} imports\n",
                    ont.id,
                    ont.slug,
                    ont.entities.len(),
                    ont.imports.len()
                ));
            }
            md.push_str("\n## Class hierarchy\n\n");
            md.push_str(&render_class_hierarchy(catalog, hierarchy));
            md.push_str("\n## Property index\n\n");
            md.push_str(&render_property_index(catalog));
            Ok(md)
        }
        ExportFormat::Html => {
            let env = html_env()?;
            let tmpl = env.get_template("index.html")?;
            Ok(tmpl.render(context! { ontologies => ontologies })?)
        }
    }
}

pub fn render_class_hierarchy(
    catalog: &OntologyCatalog,
    hierarchy: &strixonomy_catalog::ClassHierarchy,
) -> String {
    let mut nodes: BTreeSet<&str> = BTreeSet::new();
    for iri in hierarchy.parents.keys().chain(hierarchy.children.keys()) {
        nodes.insert(iri.as_str());
    }
    let mut roots: Vec<&str> = nodes
        .into_iter()
        .filter(|iri| hierarchy.parents.get(*iri).map(|p| p.is_empty()).unwrap_or(true))
        .collect();
    roots.sort();
    let mut out = String::new();
    for root in roots {
        render_hierarchy_node(catalog, hierarchy, root, 0, &mut out);
    }
    if out.is_empty() {
        out.push_str("_No class hierarchy available._\n");
    }
    out
}

fn render_hierarchy_node(
    catalog: &OntologyCatalog,
    hierarchy: &strixonomy_catalog::ClassHierarchy,
    iri: &str,
    depth: usize,
    out: &mut String,
) {
    let label = catalog
        .data()
        .entities
        .iter()
        .find(|e| e.iri == iri)
        .and_then(|e| e.labels.first())
        .map(|s| s.as_str())
        .unwrap_or(iri);
    out.push_str(&format!("{}{} (`{}`)\n", "  ".repeat(depth), label, iri));
    let mut children: Vec<&str> = hierarchy
        .children
        .get(iri)
        .map(|v| v.iter().map(|s| s.as_str()).collect())
        .unwrap_or_default();
    children.sort();
    for child in children {
        render_hierarchy_node(catalog, hierarchy, child, depth + 1, out);
    }
}

pub fn render_property_index(catalog: &OntologyCatalog) -> String {
    let mut object_props = Vec::new();
    let mut data_props = Vec::new();
    let mut annotation_props = Vec::new();
    for entity in &catalog.data().entities {
        match entity.kind {
            EntityKind::ObjectProperty => object_props.push(entity),
            EntityKind::DataProperty => data_props.push(entity),
            EntityKind::AnnotationProperty => annotation_props.push(entity),
            _ => {}
        }
    }
    let mut out = String::new();
    for (title, props) in [
        ("Object properties", &object_props),
        ("Data properties", &data_props),
        ("Annotation properties", &annotation_props),
    ] {
        if props.is_empty() {
            continue;
        }
        out.push_str(&format!("### {title}\n\n"));
        let mut sorted = props.clone();
        sorted.sort_by(|a, b| a.short_name.cmp(&b.short_name));
        for prop in sorted {
            let label = prop.labels.first().map(|s| s.as_str()).unwrap_or(&prop.short_name);
            out.push_str(&format!("- **{label}** — `{}`\n", prop.iri));
        }
        out.push('\n');
    }
    if out.is_empty() {
        out.push_str("_No properties indexed._\n");
    }
    out
}

fn render_ontology(ont: &OntologyDoc, format: ExportFormat) -> Result<String> {
    match format {
        ExportFormat::Markdown => {
            let mut md = format!("# {}\n\n", ont.id);
            md.push_str(&format!("Source: `{}`\n\n", ont.path));
            if !ont.imports.is_empty() {
                md.push_str("## Imports\n\n");
                for imp in &ont.imports {
                    md.push_str(&format!("- <{imp}>\n"));
                }
                md.push('\n');
            }
            md.push_str("## Entities\n\n");
            for entity in &ont.entities {
                let label = entity.labels.first().map(|s| s.as_str()).unwrap_or(&entity.short_name);
                md.push_str(&format!("### {label}\n\n"));
                md.push_str(&format!("- IRI: `{}`\n", entity.iri));
                md.push_str(&format!("- Kind: {}\n", entity.kind));
                if !entity.comments.is_empty() {
                    md.push_str(&format!("- Comment: {}\n", entity.comments.join("; ")));
                }
                if !entity.parents.is_empty() {
                    md.push_str(&format!("- Parents: {}\n", entity.parents.join(", ")));
                }
                md.push('\n');
            }
            Ok(md)
        }
        ExportFormat::Html => {
            let env = html_env()?;
            let tmpl = env.get_template("ontology.html")?;
            Ok(tmpl.render(context! { ont => ont })?)
        }
    }
}

fn html_env() -> Result<Environment<'static>> {
    let mut env = Environment::new();
    env.set_auto_escape_callback(|name| {
        if name.ends_with(".html") {
            AutoEscape::Html
        } else {
            AutoEscape::None
        }
    });
    env.add_template(
        "index.html",
        r#"<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>Ontology docs</title></head>
<body><h1>Ontology documentation</h1><ul>
{% for ont in ontologies %}
<li><a href="{{ ont.slug }}.html">{{ ont.id }}</a>
 — {{ ont.entities | length }} entities</li>
{% endfor %}
</ul></body></html>"#,
    )?;
    env.add_template(
        "ontology.html",
        r#"<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>{{ ont.id }}</title></head>
<body>
<h1>{{ ont.id }}</h1>
<p>Source: <code>{{ ont.path }}</code></p>
{% if ont.imports %}<h2>Imports</h2><ul>{% for imp in ont.imports %}<li>{{ imp }}</li>{% endfor %}</ul>{% endif %}
<h2>Entities</h2>
<table border="1"><tr><th>Name</th><th>Kind</th><th>IRI</th></tr>
{% for e in ont.entities %}
<tr><td>{{ e.labels[0] if e.labels else e.short_name }}</td><td>{{ e.kind }}</td><td>{{ e.iri }}</td></tr>
{% endfor %}
</table>
</body></html>"#,
    )?;
    Ok(env)
}

pub fn entity_kind_counts(catalog: &OntologyCatalog) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for entity in &catalog.data().entities {
        *counts.entry(entity.kind.as_str().to_string()).or_default() += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use strixonomy_catalog::IndexBuilder;

    #[test]
    fn exports_markdown_for_fixtures() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&fixtures).build().expect("index");
        let dir = tempfile::tempdir().unwrap();
        export_workspace(&catalog, ExportOptions::markdown(dir.path())).expect("export");
        assert!(dir.path().join("index.md").exists());
    }

    #[test]
    fn exports_entities_for_owl_ontology_declarations() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&fixtures).build().expect("index");
        let example = catalog
            .data()
            .documents
            .iter()
            .find(|d| d.path.file_name().and_then(|n| n.to_str()) == Some("example.ttl"))
            .expect("example.ttl indexed");
        let entity_count = catalog
            .data()
            .entities
            .iter()
            .filter(|e| {
                document_for_entity(&catalog.data().documents, e)
                    .is_some_and(|d| d.id == example.id)
            })
            .count();
        assert!(entity_count > 0, "fixture entities should match example.ttl via ontology IRI");

        let dir = tempfile::tempdir().unwrap();
        export_workspace(&catalog, ExportOptions::markdown(dir.path())).expect("export");
        let index = fs::read_to_string(dir.path().join("index.md")).expect("index.md");
        let doc_slug = allocate_slug(&example.id, &mut BTreeSet::new());
        let detail_path = dir.path().join(format!("{doc_slug}.md"));
        assert!(detail_path.exists(), "expected per-ontology export file");
        let detail = fs::read_to_string(detail_path).expect("ontology markdown");
        assert!(detail.contains("## Entities"), "ontology page should list entities");
        assert!(!detail.contains("## Entities\n\n\n#"), "entity section should not be empty");
        assert!(
            index.contains(&format!("{entity_count} entities")),
            "index should report exported entity count for example.ttl"
        );
    }

    #[test]
    fn slugify_collisions_are_disambiguated() {
        assert_eq!(slugify("http://a.com/x/1"), slugify("http://a.com/x_1"));
        let mut used = BTreeSet::new();
        let first = allocate_slug("http://a.com/x/1", &mut used);
        let second = allocate_slug("http://a.com/x_1", &mut used);
        assert_ne!(first, second);
        assert_eq!(first, slugify("http://a.com/x/1"));
        assert!(second.contains('_'), "collision should append a disambiguating tag");
        assert_eq!(used.len(), 2);
    }

    #[test]
    fn export_keeps_both_files_when_ontology_ids_slug_collide() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.ttl");
        let b = dir.path().join("b.ttl");
        // Distinct ontology IRIs that collapse to the same slugify() result.
        fs::write(
            &a,
            concat!(
                "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
                "<http://a.com/x/1> a owl:Ontology .\n",
                "<http://a.com/x/1#A> a owl:Class .\n",
            ),
        )
        .unwrap();
        fs::write(
            &b,
            concat!(
                "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
                "<http://a.com/x_1> a owl:Ontology .\n",
                "<http://a.com/x_1#B> a owl:Class .\n",
            ),
        )
        .unwrap();

        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("index");
        let out = tempfile::tempdir().unwrap();
        export_workspace(&catalog, ExportOptions::markdown(out.path())).expect("export");

        let md_files: Vec<_> = fs::read_dir(out.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.ends_with(".md") && n != "index.md")
            .collect();
        assert_eq!(
            md_files.len(),
            2,
            "both colliding ontology IRIs must produce distinct export files, got {md_files:?}"
        );

        let bodies: Vec<_> = md_files
            .iter()
            .map(|name| fs::read_to_string(out.path().join(name)).expect("read"))
            .collect();
        assert!(bodies.iter().any(|b| b.contains("http://a.com/x/1")));
        assert!(bodies.iter().any(|b| b.contains("http://a.com/x_1")));
    }

    #[test]
    fn markdown_index_includes_hierarchy_and_property_sections() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&fixtures).build().expect("index");
        let dir = tempfile::tempdir().unwrap();
        export_workspace(&catalog, ExportOptions::markdown(dir.path())).expect("export");
        let index = fs::read_to_string(dir.path().join("index.md")).expect("index.md");
        assert!(index.contains("## Class hierarchy"), "index should include hierarchy");
        assert!(index.contains("## Property index"), "index should include properties");
    }

    #[test]
    fn render_class_hierarchy_lists_fixture_classes() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&fixtures).build().expect("index");
        let hierarchy = catalog.class_hierarchy();
        let md = render_class_hierarchy(&catalog, &hierarchy);
        assert!(md.contains("Person") || md.contains("people#Person"));
    }

    #[test]
    fn render_property_index_lists_object_properties() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&fixtures).build().expect("index");
        let md = render_property_index(&catalog);
        assert!(md.contains("Object properties"));
        assert!(md.contains("worksFor") || md.contains("works for"));
    }

    #[test]
    fn markdown_export_includes_person_entity_page() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&fixtures).build().expect("index");
        let dir = tempfile::tempdir().unwrap();
        export_workspace(&catalog, ExportOptions::markdown(dir.path())).expect("export");
        let mut found_person = false;
        for entry in fs::read_dir(dir.path()).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let body = fs::read_to_string(&path).unwrap();
            if body.contains("Person") && body.contains("http://example.org/people#Person") {
                found_person = true;
                break;
            }
        }
        assert!(found_person, "expected a markdown page documenting Person");
    }
}
