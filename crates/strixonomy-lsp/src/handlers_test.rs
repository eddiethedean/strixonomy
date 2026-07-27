use crate::handlers::{
    handle_apply_axiom_patch, handle_apply_refactor, handle_create_ontology, handle_delete_impact,
    handle_export_ontology, handle_find_usages, handle_get_catalog_snapshot, handle_get_entity,
    handle_goto_definition, handle_hover, handle_query, handle_references, handle_search,
    handle_sparql, handle_standard_request, handle_workspace_symbol, StandardRequestOutcome,
};
use crate::index_worker::IndexWorker;
use crate::protocol::{
    ApplyAxiomPatchParams, ApplyRefactorParams, CreateOntologyParams, DeleteImpactParams,
    ExportOntologyParams, FindUsagesParams, GetEntityParams, QueryParams, SearchParams,
    SparqlParams,
};
use crate::state::{path_to_uri, ServerState};
use crossbeam_channel::unbounded;
use lsp_server::Message;
use lsp_types::{
    GotoDefinitionParams, GotoDefinitionResponse, HoverContents, HoverParams, Position,
    ReferenceContext, ReferenceParams, TextDocumentIdentifier, TextDocumentPositionParams, Uri,
    WorkspaceSymbolParams, WorkspaceSymbolResponse,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use strixonomy_refactor::{preview_rename_iri, RefactorRequest};

fn fixture_workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

fn indexed_state() -> ServerState {
    let state = ServerState::new();
    let ws = fixture_workspace();
    state.set_workspace_roots(vec![ws.clone()]).expect("set workspace");
    state.index_workspace(ws).expect("index fixture workspace");
    state
}

fn fixture_ttl_uri() -> Uri {
    let path = fixture_workspace().join("example.ttl");
    Uri::from_str(&path_to_uri(&path)).expect("fixture uri")
}

#[test]
fn index_workspace_respects_requested_root_in_multi_root() {
    let state = ServerState::new();
    let root_a = tempfile::tempdir().unwrap();
    let root_b = tempfile::tempdir().unwrap();

    let a_ttl = root_a.path().join("a.ttl");
    std::fs::write(
        &a_ttl,
        "@prefix exa: <http://example.org/a#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nexa:AClass a owl:Class .\n",
    )
    .unwrap();

    let b_ttl = root_b.path().join("b.ttl");
    std::fs::write(
        &b_ttl,
        "@prefix exb: <http://example.org/b#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nexb:BClass a owl:Class .\n",
    )
    .unwrap();

    state
        .set_workspace_roots(vec![root_a.path().to_path_buf(), root_b.path().to_path_buf()])
        .expect("set workspace");

    state.index_workspace(root_b.path().to_path_buf()).expect("index root_b");

    let b =
        handle_get_entity(&state, GetEntityParams { iri: "http://example.org/b#BClass".into() })
            .expect("getEntity B");
    assert_eq!(b.detail.entity.short_name, "BClass");

    let a =
        handle_get_entity(&state, GetEntityParams { iri: "http://example.org/a#AClass".into() })
            .expect("getEntity A");
    assert_eq!(a.detail.entity.short_name, "AClass");
}

#[test]
fn get_catalog_snapshot_before_index_returns_not_indexed() {
    let state = ServerState::new();
    let err = handle_get_catalog_snapshot(&state).unwrap_err();
    assert_eq!(err.code, "NOT_INDEXED");
}

#[test]
fn get_entity_returns_person_detail() {
    let state = indexed_state();
    let result = handle_get_entity(
        &state,
        GetEntityParams { iri: "http://example.org/people#Person".into() },
    )
    .expect("getEntity");

    assert_eq!(result.detail.entity.short_name, "Person");
    assert!(!result.detail.parents.is_empty());
    assert!(result.detail.source.is_some());
}

#[test]
fn get_entity_unknown_iri_returns_not_found() {
    let state = indexed_state();
    let err = handle_get_entity(
        &state,
        GetEntityParams { iri: "http://example.org/people#Missing".into() },
    )
    .unwrap_err();
    assert_eq!(err.code, "ENTITY_NOT_FOUND");
}

#[test]
fn hover_rejects_document_outside_workspace() {
    let state = indexed_state();
    let outside = tempfile::tempdir().unwrap();
    let ttl = outside.path().join("other.ttl");
    std::fs::write(&ttl, "@prefix ex: <http://ex/> .\n").unwrap();
    let uri = Uri::from_str(&path_to_uri(&ttl)).expect("uri");

    let hover = handle_hover(
        &state,
        HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position::new(0, 0),
            },
            work_done_progress_params: Default::default(),
        },
    );
    assert!(hover.is_none());
}

#[test]
fn get_catalog_snapshot_returns_fixture_entities() {
    let state = indexed_state();
    let snapshot = handle_get_catalog_snapshot(&state).expect("snapshot");

    assert_eq!(snapshot.documents.len(), 6);
    assert!(snapshot.entities.iter().any(|e| e.iri == "http://example.org/people#Person"));
    assert!(!snapshot.hierarchy.edges.is_empty());
}

#[test]
fn find_usages_returns_person_references() {
    let state = indexed_state();
    let result = handle_find_usages(
        &state,
        FindUsagesParams { iri: "http://example.org/people#Person".to_string() },
    )
    .expect("find usages");
    assert!(!result.usages.is_empty());
}

#[test]
fn delete_impact_lists_referencing_entities() {
    let state = indexed_state();
    let result = handle_delete_impact(
        &state,
        DeleteImpactParams { entity_iri: "http://example.org/people#Thing".to_string() },
    )
    .expect("delete impact");
    assert!(result.usage_count > 0, "Thing should have usages");
    assert!(
        result.referencing_entities.iter().any(|iri| iri == "http://example.org/people#Person"),
        "Person subclasses Thing; got {:?}",
        result.referencing_entities
    );
    assert!(
        result
            .referencing_entities
            .iter()
            .any(|iri| iri == "http://example.org/people#Organization"),
        "Organization subclasses Thing; got {:?}",
        result.referencing_entities
    );
    assert!(
        !result.referencing_entities.iter().any(|iri| iri == "http://example.org/people#Thing"),
        "target itself must not appear in referencing_entities"
    );
}

#[test]
fn references_span_covers_token_not_single_character() {
    let state = indexed_state();
    let content = std::fs::read_to_string(fixture_workspace().join("example.ttl")).unwrap();
    let person_line =
        content.lines().position(|l| l.contains("ex:Person")).expect("Person line") as u32;
    let refs = handle_references(
        &state,
        ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: fixture_ttl_uri() },
                position: Position { line: person_line, character: 0 },
            },
            context: ReferenceContext { include_declaration: true },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        },
    )
    .expect("references");
    assert!(!refs.is_empty());
    assert!(
        refs.iter().any(|r| r.range.end.character > r.range.start.character.saturating_add(1)),
        "at least one reference range should span the token, got {:?}",
        refs.iter().map(|r| r.range).collect::<Vec<_>>()
    );
}

/// Contract test: LSP JSON must use snake_case enum strings (extension + docs/lsp-api.md).
#[test]
fn catalog_snapshot_wire_format_uses_snake_case_enums() {
    let state = indexed_state();
    let snapshot = handle_get_catalog_snapshot(&state).expect("snapshot");
    let json = serde_json::to_value(&snapshot).expect("serialize snapshot");

    let person = json
        .get("entities")
        .and_then(|e| e.as_array())
        .and_then(|arr| {
            arr.iter().find(|e| {
                e.get("iri").and_then(|v| v.as_str()) == Some("http://example.org/people#Person")
            })
        })
        .expect("Person in entities");
    assert_eq!(person.get("kind").and_then(|v| v.as_str()), Some("class"));

    let doc = json
        .get("documents")
        .and_then(|d| d.as_array())
        .and_then(|arr| {
            arr.iter().find(|d| {
                d.get("path").and_then(|v| v.as_str()).is_some_and(|p| p.ends_with("example.ttl"))
            })
        })
        .expect("example.ttl document");
    assert_eq!(doc.get("parse_status").and_then(|v| v.as_str()), Some("ok"));
    assert_eq!(doc.get("format").and_then(|v| v.as_str()), Some("turtle"));
}

#[test]
fn hover_on_person_returns_markdown() {
    let state = indexed_state();
    let content = std::fs::read_to_string(fixture_workspace().join("example.ttl")).unwrap();
    let person_line =
        content.lines().position(|l| l.starts_with("ex:Person")).expect("Person line") as u32;

    let hover = handle_hover(
        &state,
        HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: fixture_ttl_uri() },
                position: Position { line: person_line, character: 2 },
            },
            work_done_progress_params: Default::default(),
        },
    )
    .expect("hover");

    let HoverContents::Markup(markup) = hover.contents else {
        panic!("expected markdown hover");
    };
    assert!(markup.value.contains("Person"));
    assert!(markup.value.contains("class"));
}

#[test]
fn goto_definition_on_person_points_to_source() {
    let state = indexed_state();
    let content = std::fs::read_to_string(fixture_workspace().join("example.ttl")).unwrap();
    let person_line =
        content.lines().position(|l| l.starts_with("ex:Person")).expect("Person line") as u32;

    let response = handle_goto_definition(
        &state,
        GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: fixture_ttl_uri() },
                position: Position { line: person_line, character: 2 },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        },
    )
    .expect("definition");

    let GotoDefinitionResponse::Scalar(location) = response else {
        panic!("expected scalar location");
    };
    assert!(location.uri.as_str().contains("example.ttl"));
    assert_eq!(location.range.start.line, person_line);
}

#[test]
fn workspace_symbol_finds_person() {
    let state = indexed_state();
    let response = handle_workspace_symbol(
        &state,
        WorkspaceSymbolParams {
            query: "Person".into(),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        },
    )
    .expect("workspace symbols");

    let WorkspaceSymbolResponse::Flat(symbols) = response else {
        panic!("expected flat symbols");
    };
    assert!(symbols.iter().any(|s| s.name == "Person"));
}

#[test]
fn search_finds_person_by_short_name() {
    let state = indexed_state();
    let result = handle_search(&state, SearchParams { query: "Person".into(), limit: Some(50) })
        .expect("search");
    assert!(result.entities.iter().any(|e| e.entity.short_name == "Person"));
}

#[test]
fn search_empty_query_returns_no_hits() {
    let state = indexed_state();
    let result =
        handle_search(&state, SearchParams { query: "".into(), limit: None }).expect("search");
    assert!(result.entities.is_empty());
}

#[test]
fn hover_on_blank_line_returns_null_result() {
    let state = indexed_state();
    let uri = fixture_ttl_uri();
    let outcome = handle_standard_request(
        &state,
        "textDocument/hover",
        Some(serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": 9999, "character": 0 }
        })),
    );
    match outcome {
        StandardRequestOutcome::Ok(value) => assert!(value.is_null()),
        other => panic!("expected null hover result, got {other:?}"),
    }
}

#[test]
fn open_buffer_override_surfaces_undefined_prefix_in_snapshot() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("live.ttl");
    let base = "@prefix ex: <http://example.org/live#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n<http://example.org/live> a owl:Ontology .\n";
    std::fs::write(&path, format!("{base}ex:Ok a owl:Class .\n")).unwrap();

    let state = ServerState::new();
    let ws = dir.path().to_path_buf();
    state.set_workspace_roots(vec![ws.clone()]).expect("set workspace");
    state.index_workspace(ws.clone()).expect("initial index");

    let doc_path = state
        .with_catalog(|catalog| {
            catalog
                .data()
                .documents
                .iter()
                .find(|d| d.path.file_name() == path.file_name())
                .map(|d| d.path.clone())
        })
        .expect("indexed catalog")
        .expect("live.ttl document");

    let _ = state
        .set_document_text(doc_path, format!("{base}ex:Ok a owl:Class .\nun:Bad a owl:Class .\n"));
    state.index_workspace(ws).expect("reindex with buffer");

    let snapshot = handle_get_catalog_snapshot(&state).expect("snapshot");
    assert!(
        snapshot.diagnostics.iter().any(|d| {
            d.severity == "error"
                && d.message.contains("un:")
                && (d.code == "undefined_prefix" || d.code == "parse_error")
        }),
        "expected undeclared prefix diagnostic from open buffer, got: {:?}",
        snapshot.diagnostics
    );
}

#[test]
fn index_workspace_params_accept_snake_case_and_legacy_camel_case() {
    let snake: crate::protocol::IndexWorkspaceParams =
        serde_json::from_value(serde_json::json!({ "workspace_uri": "file:///tmp/ws" }))
            .expect("workspace_uri");
    assert_eq!(snake.workspace_uri.as_deref(), Some("file:///tmp/ws"));

    let legacy: crate::protocol::IndexWorkspaceParams =
        serde_json::from_value(serde_json::json!({ "workspaceUri": "file:///tmp/ws" }))
            .expect("workspaceUri alias");
    assert_eq!(legacy.workspace_uri.as_deref(), Some("file:///tmp/ws"));
}

#[test]
fn query_returns_classes() {
    let state = indexed_state();
    let result =
        handle_query(&state, QueryParams { sql: "SELECT short_name FROM classes".to_string() })
            .expect("query");
    assert!(!result.columns.is_empty());
    assert!(!result.rows.is_empty());
}

#[test]
fn sparql_returns_triples() {
    let state = indexed_state();
    let result = handle_sparql(
        &state,
        SparqlParams { query: "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 5".to_string() },
    )
    .expect("sparql");
    assert!(!result.columns.is_empty());
}

#[test]
fn apply_axiom_patch_uses_open_buffer_not_disk() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("example.ttl");
    std::fs::copy(fixture_workspace().join("example.ttl"), &path).unwrap();

    let state = ServerState::new();
    let ws = dir.path().to_path_buf();
    state.set_workspace_roots(vec![ws.clone()]).expect("set workspace");
    state.index_workspace(ws.clone()).expect("index");

    let buffer_marker = "# unsaved buffer marker\n";
    let disk = std::fs::read_to_string(&path).unwrap();
    let buffer = format!("{buffer_marker}{disk}");
    state.set_document_text(path.clone(), buffer).expect("set buffer");

    let (tx, _rx) = unbounded::<Message>();
    let worker = IndexWorker::spawn(state.clone(), tx);

    let uri = path_to_uri(&path);
    let result = handle_apply_axiom_patch(
        &state,
        &worker,
        ApplyAxiomPatchParams {
            document_uri: uri,
            patches: serde_json::to_value(vec![strixonomy_owl::PatchOp::AddLabel {
                entity_iri: "http://example.org/people#Person".into(),
                value: "Human".into(),
            }])
            .expect("patch json"),
            preview_only: false,
        },
    )
    .expect("apply patch");

    assert!(result.patch.applied);
    assert!(result.workspace_edit.is_some(), "patch must return workspace_edit for editor sync");
    let updated = state.document_text(&path).expect("buffer after patch");
    assert!(
        updated.contains(buffer_marker),
        "patch should apply to open buffer, not disk-only source"
    );
    assert_eq!(
        updated.matches("rdfs:label \"Human\"").count(),
        1,
        "label must be inserted exactly once"
    );
}

#[test]
fn apply_axiom_patch_does_not_pin_closed_file_as_open_buffer() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("example.ttl");
    std::fs::copy(fixture_workspace().join("example.ttl"), &path).unwrap();

    let state = ServerState::new();
    let ws = dir.path().to_path_buf();
    state.set_workspace_roots(vec![ws.clone()]).expect("set workspace");
    state.index_workspace(ws.clone()).expect("index");

    let (tx, _rx) = unbounded::<Message>();
    let worker = IndexWorker::spawn(state.clone(), tx);
    let uri = path_to_uri(&path);
    handle_apply_axiom_patch(
        &state,
        &worker,
        ApplyAxiomPatchParams {
            document_uri: uri,
            patches: serde_json::to_value(vec![strixonomy_owl::PatchOp::AddLabel {
                entity_iri: "http://example.org/people#Person".into(),
                value: "Human".into(),
            }])
            .expect("patch json"),
            preview_only: false,
        },
    )
    .expect("apply patch");

    assert!(
        state.open_document_overrides().is_empty(),
        "closed files must not become phantom open_documents"
    );
    let disk = std::fs::read_to_string(&path).unwrap();
    assert!(disk.contains("Human"));
}

fn refactor_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/refactor").join(path)
}

#[test]
fn apply_refactor_tracks_only_open_buffers() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("people.ttl");
    std::fs::copy(refactor_fixture("people.ttl"), &path).unwrap();

    let state = ServerState::new();
    let ws = dir.path().to_path_buf();
    state.set_workspace_roots(vec![ws.clone()]).expect("set workspace");
    state.index_workspace(ws.clone()).expect("index");

    let from = "http://example.org/org#Agent";
    let to = "http://example.org/org#Worker";
    let overrides = HashMap::new();
    let plan = state
        .with_catalog(|catalog| preview_rename_iri(catalog, from, to, &overrides).ok())
        .flatten()
        .expect("plan");

    let (tx, _rx) = unbounded::<Message>();
    let worker = IndexWorker::spawn(state.clone(), tx);

    let result = handle_apply_refactor(
        &state,
        &worker,
        ApplyRefactorParams {
            plan: plan.clone(),
            request: RefactorRequest::RenameIri {
                from_iri: from.to_string(),
                to_iri: to.to_string(),
            },
            preview_only: false,
        },
    )
    .expect("apply refactor");

    assert!(result.workspace_edit.is_some());
    assert!(
        state.open_document_overrides().is_empty(),
        "closed files must not be injected into open_documents"
    );

    let disk = std::fs::read_to_string(&path).unwrap();
    assert!(disk.contains("Worker") || disk.contains("ex:Worker"));

    let buffer_marker = "# unsaved buffer marker\n";
    let buffer = format!("{buffer_marker}{}", std::fs::read_to_string(&path).unwrap());
    state.set_document_text(path.clone(), buffer.clone()).expect("open buffer");

    let overrides = state.open_document_overrides();
    let plan2 = state
        .with_catalog(|catalog| preview_rename_iri(catalog, to, from, &overrides).ok())
        .flatten()
        .expect("plan2");

    handle_apply_refactor(
        &state,
        &worker,
        ApplyRefactorParams {
            plan: plan2,
            request: RefactorRequest::RenameIri {
                from_iri: to.to_string(),
                to_iri: from.to_string(),
            },
            preview_only: false,
        },
    )
    .expect("apply refactor to open buffer");

    let updated = state.document_text(&path).expect("buffer after refactor");
    assert!(updated.contains(buffer_marker), "refactor should update open buffer in place");
    assert!(updated.contains("Agent") || updated.contains("ex:Agent"));
}

#[test]
fn list_sql_schema_returns_axiom_tables() {
    let state = indexed_state();
    let schema = crate::handlers::handle_list_sql_schema(&state).expect("listSqlSchema");
    let names: Vec<_> = schema.tables.iter().map(|t| t.name.as_str()).collect();
    assert!(names.contains(&"domain_axioms"));
    assert!(names.contains(&"restrictions"));
    let restrictions = schema.tables.iter().find(|t| t.name == "restrictions").unwrap();
    assert_eq!(restrictions.columns.len(), 4);
}

#[test]
fn semantic_tokens_full_returns_tokens_for_open_ttl_buffer() {
    let state = indexed_state();
    let path = state
        .with_catalog(|catalog| {
            catalog
                .data()
                .documents
                .iter()
                .find(|d| d.path.file_name().and_then(|n| n.to_str()) == Some("example.ttl"))
                .map(|d| d.path.clone())
        })
        .expect("indexed catalog")
        .expect("example.ttl document");
    let text = std::fs::read_to_string(&path).expect("read example.ttl");
    state.set_document_text(path.clone(), text).expect("open buffer");

    let uri = Uri::from_str(&path_to_uri(&path)).expect("uri");
    let outcome = handle_standard_request(
        &state,
        "textDocument/semanticTokens/full",
        Some(serde_json::json!({
            "textDocument": { "uri": uri.to_string() }
        })),
    );
    match outcome {
        StandardRequestOutcome::Ok(value) => {
            let data =
                value.get("data").and_then(|d| d.as_array()).expect("semantic token data array");
            assert!(!data.is_empty(), "expected semantic tokens for example.ttl");
        }
        other => panic!("unexpected semantic tokens outcome: {other:?}"),
    }
}

#[test]
fn create_ontology_rejects_adversarial_iri() {
    let dir = tempfile::tempdir().unwrap();
    let state = ServerState::new();
    state.set_workspace_roots(vec![dir.path().to_path_buf()]).expect("set workspace");
    let path = dir.path().join("evil.ttl");
    let evil = "http://ex.org/x> a owl:Class . <http://ex.org/y".to_string();
    let err = handle_create_ontology(
        &state,
        CreateOntologyParams {
            path: path.display().to_string(),
            ontology_iri: evil,
            version_iri: None,
            format: Some("ttl".into()),
            prefixes: None,
        },
    )
    .expect_err("unsafe IRI must be rejected");
    assert!(
        err.message.contains("cannot be safely written") || err.message.contains("ontology IRI"),
        "unexpected error: {}",
        err.message
    );
    assert!(!path.exists());
}

#[test]
#[cfg(unix)]
fn create_ontology_rejects_dangling_leaf_symlink() {
    let dir = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let state = ServerState::new();
    state.set_workspace_roots(vec![dir.path().to_path_buf()]).expect("set workspace");
    let path = dir.path().join("new.ttl");
    let outside_target = outside.path().join("pwn.ttl");
    std::os::unix::fs::symlink(&outside_target, &path).unwrap();

    let err = handle_create_ontology(
        &state,
        CreateOntologyParams {
            path: path.display().to_string(),
            ontology_iri: "http://example.org/ont".into(),
            version_iri: None,
            format: Some("ttl".into()),
            prefixes: None,
        },
    )
    .expect_err("dangling leaf symlink must be rejected");
    assert!(
        err.message.contains("dangling symlink")
            || err.message.contains("symlink")
            || err.message.contains("outside"),
        "unexpected error: {}",
        err.message
    );
    assert!(!outside_target.exists(), "must not create outside the workspace");
}

#[test]
fn create_ontology_rejects_unsafe_prefix_iri() {
    let dir = tempfile::tempdir().unwrap();
    let state = ServerState::new();
    state.set_workspace_roots(vec![dir.path().to_path_buf()]).expect("set workspace");
    let path = dir.path().join("evil-prefix.ttl");
    let mut prefixes = std::collections::BTreeMap::new();
    prefixes.insert("ex".to_string(), "http://ex.org/x> . <http://ex.org/y".to_string());
    let err = handle_create_ontology(
        &state,
        CreateOntologyParams {
            path: path.display().to_string(),
            ontology_iri: "http://example.org/ont".into(),
            version_iri: None,
            format: Some("ttl".into()),
            prefixes: Some(prefixes),
        },
    )
    .expect_err("unsafe prefix IRI must be rejected");
    assert!(
        err.message.contains("prefix") || err.message.contains("IRI"),
        "unexpected error: {}",
        err.message
    );
    assert!(!path.exists());
}

#[test]
fn create_ontology_writes_safe_turtle() {
    let dir = tempfile::tempdir().unwrap();
    let state = ServerState::new();
    state.set_workspace_roots(vec![dir.path().to_path_buf()]).expect("set workspace");
    let path = dir.path().join("fresh.ttl");
    let result = handle_create_ontology(
        &state,
        CreateOntologyParams {
            path: path.display().to_string(),
            ontology_iri: "http://example.org/ont".into(),
            version_iri: Some("http://example.org/ont/1".into()),
            format: Some("ttl".into()),
            prefixes: None,
        },
    )
    .expect("create ontology");
    assert_eq!(result.ontology_iri, "http://example.org/ont");
    let created = PathBuf::from(&result.path);
    let contents = std::fs::read_to_string(&created).expect("read created file");
    assert!(contents.contains("<http://example.org/ont> a owl:Ontology"));
    assert!(contents.contains("owl:versionIRI <http://example.org/ont/1>"));
    assert!(!contents.contains("a owl:Class"));
}

#[test]
fn create_ontology_rejects_ttl_format_under_owl_extension() {
    let dir = tempfile::tempdir().unwrap();
    let state = ServerState::new();
    state.set_workspace_roots(vec![dir.path().to_path_buf()]).expect("set workspace");
    let err = handle_create_ontology(
        &state,
        CreateOntologyParams {
            path: dir.path().join("fresh.owl").display().to_string(),
            ontology_iri: "http://example.org/ont".into(),
            version_iri: None,
            format: Some("ttl".into()),
            prefixes: None,
        },
    )
    .expect_err("ttl under .owl must fail");
    assert!(
        err.message.contains("does not match path extension"),
        "unexpected message: {}",
        err.message
    );
}

#[test]
fn create_ontology_rejects_rdfxml_and_owl_extension() {
    let dir = tempfile::tempdir().unwrap();
    let state = ServerState::new();
    state.set_workspace_roots(vec![dir.path().to_path_buf()]).expect("set workspace");
    let err = handle_create_ontology(
        &state,
        CreateOntologyParams {
            path: dir.path().join("fresh.owl").display().to_string(),
            ontology_iri: "http://example.org/ont".into(),
            version_iri: None,
            format: Some("rdfxml".into()),
            prefixes: None,
        },
    )
    .expect_err("rdfxml create must fail");
    assert!(err.message.contains("unsupported createOntology format"));
}

struct CwdGuard {
    previous: PathBuf,
}

impl CwdGuard {
    fn enter(dir: &std::path::Path) -> Self {
        let previous = std::env::current_dir().expect("current dir");
        std::env::set_current_dir(dir).expect("set cwd");
        Self { previous }
    }
}

impl Drop for CwdGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.previous);
    }
}

#[test]
fn create_ontology_writes_under_workspace_not_cwd() {
    let workspace = tempfile::tempdir().unwrap();
    let cwd = tempfile::tempdir().unwrap();
    let state = ServerState::new();
    state.set_workspace_roots(vec![workspace.path().to_path_buf()]).expect("set workspace");

    let _cwd = CwdGuard::enter(cwd.path());
    let result = handle_create_ontology(
        &state,
        CreateOntologyParams {
            path: "relative-new.ttl".into(),
            ontology_iri: "http://example.org/ont".into(),
            version_iri: None,
            format: Some("ttl".into()),
            prefixes: None,
        },
    )
    .expect("create ontology with relative path");

    let created = PathBuf::from(&result.path);
    assert!(created.exists(), "expected file at {}", created.display());
    let root = workspace.path().canonicalize().expect("canonicalize workspace");
    assert!(
        strixonomy_core::is_path_within(&root, &created),
        "created path {} must be under workspace {}",
        created.display(),
        root.display()
    );
    assert!(
        !cwd.path().join("relative-new.ttl").exists(),
        "must not write relative path against process CWD"
    );
    let contents = std::fs::read_to_string(&created).expect("read created file");
    assert!(contents.contains("<http://example.org/ont> a owl:Ontology"));
}

#[test]
fn export_ontology_writes_under_workspace_not_cwd() {
    let workspace = tempfile::tempdir().unwrap();
    let cwd = tempfile::tempdir().unwrap();
    let source = workspace.path().join("source.ttl");
    std::fs::write(
        &source,
        "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\n<http://example.org/ont> a owl:Ontology .\n",
    )
    .unwrap();

    let state = ServerState::new();
    state.set_workspace_roots(vec![workspace.path().to_path_buf()]).expect("set workspace");

    let _cwd = CwdGuard::enter(cwd.path());
    let result = handle_export_ontology(
        &state,
        ExportOntologyParams {
            source_path: "source.ttl".into(),
            output_path: "exported.ttl".into(),
            format: Some("ttl".into()),
        },
    )
    .expect("export ontology with relative paths");

    let exported = PathBuf::from(&result.output_path);
    assert!(result.success);
    assert!(exported.exists(), "expected export at {}", exported.display());
    let root = workspace.path().canonicalize().expect("canonicalize workspace");
    assert!(
        strixonomy_core::is_path_within(&root, &exported),
        "export path {} must be under workspace {}",
        exported.display(),
        root.display()
    );
    assert!(!cwd.path().join("exported.ttl").exists(), "must not write export against process CWD");
    assert!(!cwd.path().join("source.ttl").exists(), "must not resolve source against process CWD");
}

#[test]
#[cfg(unix)]
fn export_ontology_fallback_rejects_leaf_symlink() {
    // #353 — do not follow a post-validate leaf symlink via fs::copy.
    let workspace = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let source = workspace.path().join("source.ttl");
    std::fs::write(
        &source,
        "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\n<http://example.org/ont> a owl:Ontology .\n",
    )
    .unwrap();
    let output = workspace.path().join("out.ttl");
    let outside_target = outside.path().join("pwn.ttl");
    std::os::unix::fs::symlink(&outside_target, &output).unwrap();

    let state = ServerState::new();
    state.set_workspace_roots(vec![workspace.path().to_path_buf()]).expect("set workspace");

    let err = handle_export_ontology(
        &state,
        ExportOntologyParams {
            source_path: source.display().to_string(),
            output_path: output.display().to_string(),
            format: Some("ttl".into()),
        },
    )
    .expect_err("leaf symlink output must be rejected");
    assert!(
        err.message.contains("symlink") || err.message.contains("outside"),
        "unexpected error: {}",
        err.message
    );
    assert!(!outside_target.exists(), "must not write through symlink outside workspace");
}

#[test]
fn reasoner_classify_releases_ops_lock_during_classify() {
    use crate::handlers::handle_run_reasoner_lsp;
    use crate::protocol::RunReasonerParams;
    use crate::state::{
        TEST_REASONER_CLASSIFY_IN_PAUSE, TEST_REASONER_CLASSIFY_LOCK,
        TEST_REASONER_CLASSIFY_PAUSE_MS,
    };
    use crossbeam_channel::unbounded;
    use lsp_server::RequestId;
    use std::collections::VecDeque;
    use std::sync::atomic::Ordering;
    use std::thread;
    use std::time::{Duration, Instant};

    let state = indexed_state();
    let _classify_pause_guard =
        TEST_REASONER_CLASSIFY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    TEST_REASONER_CLASSIFY_IN_PAUSE.store(false, Ordering::SeqCst);
    TEST_REASONER_CLASSIFY_PAUSE_MS.store(200, Ordering::SeqCst);

    let state_reasoner = state.clone();
    let (_tx, rx) = unbounded();
    let reasoner = thread::spawn(move || {
        let generation = state_reasoner.begin_reasoner_run(RequestId::from(42i32));
        let mut deferred = VecDeque::new();
        let result = handle_run_reasoner_lsp(
            &state_reasoner,
            RunReasonerParams { profile: "el".into(), auto_detect: false },
            &rx,
            &mut deferred,
            generation,
        );
        state_reasoner.clear_active_reasoner_request();
        result.expect("reasoner should complete")
    });

    let deadline = Instant::now() + Duration::from_secs(5);
    while !TEST_REASONER_CLASSIFY_IN_PAUSE.load(Ordering::SeqCst) {
        assert!(Instant::now() < deadline, "reasoner never entered classify pause");
        thread::sleep(Duration::from_millis(1));
    }

    let lock_start = Instant::now();
    let lock_elapsed = {
        let ops_lock = state.ops_lock();
        let _guard = ops_lock.lock().expect("ops_lock should be available during classify");
        let elapsed = lock_start.elapsed();
        // Release before join: the reasoner reacquires ops_lock to store the snapshot.
        elapsed
    };
    TEST_REASONER_CLASSIFY_PAUSE_MS.store(0, Ordering::SeqCst);

    reasoner.join().expect("reasoner thread");
    assert!(
        lock_elapsed < Duration::from_millis(100),
        "ops_lock blocked for {lock_elapsed:?} during reasoner classify"
    );
}

#[test]
fn cancelled_reasoner_run_does_not_update_snapshot() {
    use crate::handlers::handle_run_reasoner_lsp;
    use crate::protocol::RunReasonerParams;
    use crate::state::{
        TEST_REASONER_CLASSIFY_IN_PAUSE, TEST_REASONER_CLASSIFY_LOCK,
        TEST_REASONER_CLASSIFY_PAUSE_MS,
    };
    use crossbeam_channel::unbounded;
    use lsp_server::{Message, Notification, RequestId};
    use std::collections::VecDeque;
    use std::sync::atomic::Ordering;
    use std::thread;
    use std::time::{Duration, Instant};

    let state = indexed_state();
    let before = state.with_catalog_and_reasoner(|_, reasoner| reasoner.cloned()).flatten();

    let _classify_pause_guard =
        TEST_REASONER_CLASSIFY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    TEST_REASONER_CLASSIFY_IN_PAUSE.store(false, Ordering::SeqCst);
    TEST_REASONER_CLASSIFY_PAUSE_MS.store(200, Ordering::SeqCst);

    let (tx, rx) = unbounded();
    let state_reasoner = state.clone();
    let reasoner = thread::spawn(move || {
        let generation = state_reasoner.begin_reasoner_run(RequestId::from(99i32));
        let mut deferred = VecDeque::new();
        let result = handle_run_reasoner_lsp(
            &state_reasoner,
            RunReasonerParams { profile: "el".into(), auto_detect: false },
            &rx,
            &mut deferred,
            generation,
        );
        state_reasoner.clear_active_reasoner_request();
        result
    });

    let deadline = Instant::now() + Duration::from_secs(5);
    while !TEST_REASONER_CLASSIFY_IN_PAUSE.load(Ordering::SeqCst) {
        assert!(Instant::now() < deadline, "reasoner never entered classify pause");
        thread::sleep(Duration::from_millis(1));
    }

    let _ = tx.send(Message::Notification(Notification {
        method: "$/cancelRequest".into(),
        params: serde_json::json!({ "id": 99 }),
    }));

    let result = reasoner.join().expect("reasoner thread");
    TEST_REASONER_CLASSIFY_PAUSE_MS.store(0, Ordering::SeqCst);

    assert!(result.is_err(), "cancelled reasoner should return error");
    let after = state.with_catalog_and_reasoner(|_, reasoner| reasoner.cloned()).flatten();
    match (&before, &after) {
        (None, None) => {}
        (Some(b), Some(a)) => assert_eq!(b.profile_used, a.profile_used),
        _ => panic!("cancelled run changed reasoner snapshot presence"),
    }
}

#[test]
fn reasoner_cancel_drain_preserves_non_cancel_messages() {
    use crate::handlers::handle_run_reasoner_lsp;
    use crate::protocol::RunReasonerParams;
    use crate::state::{
        TEST_REASONER_CLASSIFY_IN_PAUSE, TEST_REASONER_CLASSIFY_LOCK,
        TEST_REASONER_CLASSIFY_PAUSE_MS,
    };
    use crossbeam_channel::unbounded;
    use lsp_server::{Message, Notification, Request, RequestId};
    use std::collections::VecDeque;
    use std::sync::atomic::Ordering;
    use std::thread;
    use std::time::{Duration, Instant};

    let state = indexed_state();
    let _classify_pause_guard =
        TEST_REASONER_CLASSIFY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    TEST_REASONER_CLASSIFY_IN_PAUSE.store(false, Ordering::SeqCst);
    TEST_REASONER_CLASSIFY_PAUSE_MS.store(300, Ordering::SeqCst);

    let (tx, rx) = unbounded();
    let state_reasoner = state.clone();
    let reasoner = thread::spawn(move || {
        let generation = state_reasoner.begin_reasoner_run(RequestId::from(7i32));
        let mut deferred = VecDeque::new();
        let result = handle_run_reasoner_lsp(
            &state_reasoner,
            RunReasonerParams { profile: "el".into(), auto_detect: false },
            &rx,
            &mut deferred,
            generation,
        );
        state_reasoner.clear_active_reasoner_request();
        (result, deferred)
    });

    let deadline = Instant::now() + Duration::from_secs(5);
    while !TEST_REASONER_CLASSIFY_IN_PAUSE.load(Ordering::SeqCst) {
        assert!(Instant::now() < deadline, "reasoner never entered classify pause");
        thread::sleep(Duration::from_millis(1));
    }

    let _ = tx.send(Message::Notification(Notification {
        method: "textDocument/didChange".into(),
        params: serde_json::json!({
            "textDocument": { "uri": "file:///tmp/a.ttl", "version": 2 },
            "contentChanges": [{ "text": "updated" }]
        }),
    }));
    let _ = tx.send(Message::Request(Request {
        id: RequestId::from(100i32),
        method: "strixonomy/getCatalogSnapshot".into(),
        params: serde_json::json!({}),
    }));
    let _ = tx.send(Message::Notification(Notification {
        method: "$/cancelRequest".into(),
        params: serde_json::json!({ "id": 7 }),
    }));

    let (result, deferred) = reasoner.join().expect("reasoner thread");
    TEST_REASONER_CLASSIFY_PAUSE_MS.store(0, Ordering::SeqCst);

    assert!(result.is_err(), "cancelled reasoner should return error");
    assert_eq!(deferred.len(), 2, "didChange and custom request must survive cancel drain");
    match &deferred[0] {
        Message::Notification(n) => assert_eq!(n.method, "textDocument/didChange"),
        other => panic!("expected didChange first, got {other:?}"),
    }
    match &deferred[1] {
        Message::Request(r) => {
            assert_eq!(r.method, "strixonomy/getCatalogSnapshot");
            assert_eq!(r.id, RequestId::from(100i32));
        }
        other => panic!("expected getCatalogSnapshot second, got {other:?}"),
    }
}

#[test]
fn reindex_during_classify_discards_stale_reasoner_snapshot() {
    use crate::handlers::handle_run_reasoner_lsp;
    use crate::protocol::RunReasonerParams;
    use crate::state::{
        TEST_REASONER_CLASSIFY_IN_PAUSE, TEST_REASONER_CLASSIFY_LOCK,
        TEST_REASONER_CLASSIFY_PAUSE_MS,
    };
    use crossbeam_channel::unbounded;
    use lsp_server::RequestId;
    use std::collections::VecDeque;
    use std::sync::atomic::Ordering;
    use std::thread;
    use std::time::{Duration, Instant};

    let state = indexed_state();
    let before = state.with_catalog_and_reasoner(|_, reasoner| reasoner.cloned()).flatten();
    let ws = state.workspace_root().expect("workspace");

    let _classify_pause_guard =
        TEST_REASONER_CLASSIFY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    TEST_REASONER_CLASSIFY_IN_PAUSE.store(false, Ordering::SeqCst);
    TEST_REASONER_CLASSIFY_PAUSE_MS.store(300, Ordering::SeqCst);

    let (_tx, rx) = unbounded();
    let state_reasoner = state.clone();
    let reasoner = thread::spawn(move || {
        let generation = state_reasoner.begin_reasoner_run(RequestId::from(55i32));
        let mut deferred = VecDeque::new();
        let result = handle_run_reasoner_lsp(
            &state_reasoner,
            RunReasonerParams { profile: "el".into(), auto_detect: false },
            &rx,
            &mut deferred,
            generation,
        );
        state_reasoner.clear_active_reasoner_request();
        result
    });

    let deadline = Instant::now() + Duration::from_secs(5);
    while !TEST_REASONER_CLASSIFY_IN_PAUSE.load(Ordering::SeqCst) {
        assert!(Instant::now() < deadline, "reasoner never entered classify pause");
        thread::sleep(Duration::from_millis(1));
    }

    state.index_workspace(ws).expect("reindex during classify");

    let result = reasoner.join().expect("reasoner thread");
    TEST_REASONER_CLASSIFY_PAUSE_MS.store(0, Ordering::SeqCst);

    assert!(result.is_err(), "stale reasoner after reindex must not publish: {result:?}");
    let after = state.with_catalog_and_reasoner(|_, reasoner| reasoner.cloned()).flatten();
    match (&before, &after) {
        (None, None) => {}
        (Some(b), Some(a)) => assert_eq!(b.profile_used, a.profile_used),
        _ => panic!("reindex-during-classify changed reasoner snapshot presence"),
    }
}
