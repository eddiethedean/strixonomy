mod support;

use std::path::PathBuf;
use strixonomy_reasoner::{classify, ReasonerId, WorkspaceInputLoader};

fn el_only_workspace() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    std::fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/reasoner-el.ttl"),
        workspace.join("reasoner-el.ttl"),
    )
    .expect("copy fixture");
    (dir, workspace)
}

fn has_edge(
    edges: &[strixonomy_catalog::SubclassEdge],
    child_suffix: &str,
    parent_suffix: &str,
) -> bool {
    edges.iter().any(|e| e.child.ends_with(child_suffix) && e.parent.ends_with(parent_suffix))
}

fn reaches_ancestor(
    parents: &std::collections::BTreeMap<String, Vec<String>>,
    child_iri: &str,
    ancestor_iri: &str,
) -> bool {
    let mut stack = vec![child_iri.to_string()];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(cur) = stack.pop() {
        if !seen.insert(cur.clone()) {
            continue;
        }
        if cur == ancestor_iri {
            return true;
        }
        if let Some(ps) = parents.get(&cur) {
            stack.extend(ps.iter().cloned());
        }
    }
    false
}

#[test]
fn el_classify_el_fixture_workspace() {
    let (_dir, workspace) = el_only_workspace();
    let input = WorkspaceInputLoader::new(&workspace).load().expect("load");
    let result = classify(ReasonerId::El, &input, false).expect("classify");

    assert_eq!(result.profile_used, "el");
    assert!(result.consistent);
    assert!(result.unsatisfiable.is_empty());

    let dog = "http://example.org/reasoner-el#Dog";
    let mammal = "http://example.org/reasoner-el#Mammal";
    let animal = "http://example.org/reasoner-el#Animal";
    let combined = &result.inferred.combined;

    assert!(
        has_edge(&combined.edges, "#Dog", "#Mammal"),
        "expected Dog ⊑ Mammal in combined hierarchy, got {:?}",
        combined.edges
    );
    assert!(
        has_edge(&combined.edges, "#Mammal", "#Animal"),
        "expected Mammal ⊑ Animal in combined hierarchy, got {:?}",
        combined.edges
    );
    assert!(
        reaches_ancestor(&combined.parents, dog, animal),
        "Dog must reach Animal via combined parents map"
    );
    assert!(
        combined.parents.get(dog).is_some_and(|p| p.iter().any(|x| x == mammal)),
        "Dog's direct parent must include Mammal"
    );
}

#[test]
fn dl_profile_classifies_el_fixture_workspace() {
    let (_dir, workspace) = el_only_workspace();
    let input = WorkspaceInputLoader::new(&workspace).load().expect("load");
    let result = classify(ReasonerId::Dl, &input, false).expect("classify");

    assert_eq!(result.profile_used, "dl");
    assert!(result.consistent);
    assert!(has_edge(&result.inferred.combined.edges, "#Dog", "#Mammal"));
    assert!(has_edge(&result.inferred.combined.edges, "#Mammal", "#Animal"));
}

#[test]
fn auto_profile_classifies_el_fixture_workspace() {
    let (_dir, workspace) = el_only_workspace();
    let input = WorkspaceInputLoader::new(&workspace).load().expect("load");
    let result = classify(ReasonerId::Auto, &input, false).expect("classify");

    assert_eq!(
        result.profile_used, "el",
        "Auto should report the concrete engine used for an EL ontology"
    );
    assert!(result.consistent);
    assert!(has_edge(&result.inferred.combined.edges, "#Dog", "#Mammal"));
    assert!(has_edge(&result.inferred.combined.edges, "#Mammal", "#Animal"));
}
