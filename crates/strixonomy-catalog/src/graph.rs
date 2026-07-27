//! Graph export for visualization webviews.

use crate::entity_api::SubclassEdge;
use crate::OntologyCatalog;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
use strixonomy_core::{
    limits::{MAX_GRAPH_EDGES, MAX_GRAPH_NODES},
    Entity, EntityKind, AXIOM_KIND_CLASS_ASSERTION, AXIOM_KIND_DOMAIN, AXIOM_KIND_EQUIVALENT_CLASS,
    AXIOM_KIND_OBJECT_PROPERTY_ASSERTION, AXIOM_KIND_RANGE, AXIOM_KIND_SUB_CLASS_OF,
    AXIOM_KIND_SUB_DATA_PROPERTY_OF, AXIOM_KIND_SUB_OBJECT_PROPERTY_OF,
};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct GraphError(String);

impl From<String> for GraphError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

pub type GraphResult<T> = std::result::Result<T, GraphError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphKind {
    Class,
    Property,
    ObjectProperty,
    DataProperty,
    Individual,
    Import,
    Dependency,
    Neighborhood,
    QueryResult,
    RefactorPreview,
}

impl GraphKind {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "class" => Some(Self::Class),
            "property" => Some(Self::Property),
            "object_property" => Some(Self::ObjectProperty),
            "data_property" => Some(Self::DataProperty),
            "individual" => Some(Self::Individual),
            "import" => Some(Self::Import),
            "dependency" => Some(Self::Dependency),
            "neighborhood" => Some(Self::Neighborhood),
            "query_result" => Some(Self::QueryResult),
            "refactor_preview" => Some(Self::RefactorPreview),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Class => "class",
            Self::Property => "property",
            Self::ObjectProperty => "object_property",
            Self::DataProperty => "data_property",
            Self::Individual => "individual",
            Self::Import => "import",
            Self::Dependency => "dependency",
            Self::Neighborhood => "neighborhood",
            Self::QueryResult => "query_result",
            Self::RefactorPreview => "refactor_preview",
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GraphFilters {
    pub ontology_iri: Option<String>,
    #[serde(default)]
    pub hide_deprecated: bool,
    #[serde(default)]
    pub entity_kinds: Vec<String>,
    #[serde(default)]
    pub namespaces: Vec<String>,
    #[serde(default)]
    pub relationship_kinds: Vec<String>,
    pub search_text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GraphRequest {
    pub graph_kind: String,
    pub root_iri: Option<String>,
    /// Multi-root set for `query_result` / `refactor_preview` (and optional neighborhood seeds).
    #[serde(default)]
    pub root_iris: Vec<String>,
    #[serde(default = "default_depth")]
    pub depth: u32,
    #[serde(default)]
    pub include_inferred: bool,
    #[serde(default)]
    pub filters: GraphFilters,
}

fn default_depth() -> u32 {
    2
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ontology_iri: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub deprecated: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub unsatisfiable: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub equivalent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub kind: String,
    pub inferred: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphPayload {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub truncated: bool,
    pub graph_kind: String,
}

pub struct GraphBuilder<'a> {
    catalog: &'a OntologyCatalog,
    inferred_edges: Option<&'a [SubclassEdge]>,
}

impl<'a> GraphBuilder<'a> {
    pub fn new(catalog: &'a OntologyCatalog) -> Self {
        Self { catalog, inferred_edges: None }
    }

    pub fn with_inferred_edges(mut self, edges: &'a [SubclassEdge]) -> Self {
        self.inferred_edges = Some(edges);
        self
    }

    pub fn build(&self, request: &GraphRequest) -> GraphResult<GraphPayload> {
        let kind = GraphKind::parse(&request.graph_kind)
            .ok_or_else(|| GraphError(format!("unknown graph_kind: {}", request.graph_kind)))?;
        let depth = request.depth.clamp(1, 5);

        let mut payload = match kind {
            GraphKind::Class => self.build_class_graph(request),
            GraphKind::Property => self.build_property_graph(request, None),
            GraphKind::ObjectProperty => {
                self.build_property_hierarchy_graph(request, EntityKind::ObjectProperty)
            }
            GraphKind::DataProperty => {
                self.build_property_hierarchy_graph(request, EntityKind::DataProperty)
            }
            GraphKind::Individual => {
                let roots = collect_roots(request);
                if roots.is_empty() {
                    return Err(GraphError(
                        "individual graph requires root_iri or root_iris".to_string(),
                    ));
                }
                self.build_individual_graph(request, &roots, depth)
            }
            GraphKind::Import => self.build_import_graph(request, false),
            GraphKind::Dependency => self.build_import_graph(request, true),
            GraphKind::Neighborhood => {
                let root = request.root_iri.as_deref().ok_or_else(|| {
                    GraphError("neighborhood graph requires root_iri".to_string())
                })?;
                self.build_neighborhood_graph(request, root, depth)
            }
            GraphKind::QueryResult | GraphKind::RefactorPreview => {
                let roots = collect_roots(request);
                if roots.is_empty() {
                    return Err(GraphError(format!(
                        "{} graph requires root_iri or root_iris",
                        kind.as_str()
                    )));
                }
                self.build_seeded_result_graph(request, &roots, depth, kind)
            }
        }?;

        payload.graph_kind = kind.as_str().to_string();
        Ok(payload)
    }

    fn entity_allowed(&self, iri: &str, filters: &GraphFilters) -> bool {
        let Some(entity) = self.catalog.find_entity(iri) else {
            // Unknown IRIs are not in any indexed ontology — exclude them when filtering
            // by ontology_iri so external/imported parents cannot bypass the filter (#117).
            if filters.ontology_iri.is_some() {
                return false;
            }
            if !filters.entity_kinds.is_empty() || !filters.namespaces.is_empty() {
                return false;
            }
            if let Some(ref q) = filters.search_text {
                let q = q.trim().to_lowercase();
                if !q.is_empty()
                    && !iri.to_lowercase().contains(&q)
                    && !short_name(iri).to_lowercase().contains(&q)
                {
                    return false;
                }
            }
            return !filters.hide_deprecated;
        };
        if filters.hide_deprecated && entity.deprecated {
            return false;
        }
        if let Some(ref ont) = filters.ontology_iri {
            if entity.ontology_id != *ont {
                return false;
            }
        }
        if !filters.entity_kinds.is_empty() {
            let kind = entity.kind.as_str();
            if !filters.entity_kinds.iter().any(|k| k == kind) {
                return false;
            }
        }
        if !filters.namespaces.is_empty() {
            let ns = namespace_of(iri);
            if !filters.namespaces.iter().any(|n| n == &ns || iri.starts_with(n)) {
                return false;
            }
        }
        if let Some(ref q) = filters.search_text {
            let q = q.trim().to_lowercase();
            if !q.is_empty() {
                let label_hit = entity.labels.iter().any(|l| l.to_lowercase().contains(&q));
                let iri_hit = entity.iri.to_lowercase().contains(&q)
                    || entity.short_name.to_lowercase().contains(&q);
                if !label_hit && !iri_hit {
                    return false;
                }
            }
        }
        true
    }

    fn relationship_allowed(&self, kind: &str, filters: &GraphFilters) -> bool {
        if filters.relationship_kinds.is_empty() {
            return true;
        }
        filters.relationship_kinds.iter().any(|k| k == kind)
    }

    fn add_node_for(
        &self,
        nodes: &mut Vec<GraphNode>,
        node_ids: &mut HashSet<String>,
        truncated: &mut bool,
        iri: &str,
    ) {
        if node_ids.contains(iri) {
            return;
        }
        if nodes.len() >= MAX_GRAPH_NODES {
            *truncated = true;
            return;
        }
        node_ids.insert(iri.to_string());
        nodes.push(self.make_node(iri));
    }

    fn make_node(&self, iri: &str) -> GraphNode {
        if let Some(entity) = self.catalog.find_entity(iri) {
            return graph_node_from_entity(entity);
        }
        GraphNode {
            id: iri.to_string(),
            label: short_name(iri),
            kind: "other".to_string(),
            namespace: Some(namespace_of(iri)),
            ontology_iri: None,
            deprecated: false,
            unsatisfiable: false,
            equivalent: false,
        }
    }

    fn add_edge(
        &self,
        edges: &mut Vec<GraphEdge>,
        truncated: &mut bool,
        filters: &GraphFilters,
        edge: GraphEdge,
    ) {
        if !self.relationship_allowed(&edge.kind, filters) {
            return;
        }
        if edges.len() >= MAX_GRAPH_EDGES {
            *truncated = true;
            return;
        }
        edges.push(edge);
    }

    fn build_class_graph(&self, request: &GraphRequest) -> Result<GraphPayload, String> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_ids = HashSet::new();
        let mut truncated = false;

        let hierarchy = self.catalog.class_hierarchy();
        for edge in &hierarchy.edges {
            if !self.entity_allowed(&edge.child, &request.filters)
                || !self.entity_allowed(&edge.parent, &request.filters)
            {
                continue;
            }
            self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, &edge.child);
            self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, &edge.parent);
            self.add_edge(
                &mut edges,
                &mut truncated,
                &request.filters,
                GraphEdge {
                    source: edge.child.clone(),
                    target: edge.parent.clone(),
                    kind: "sub_class_of".to_string(),
                    inferred: false,
                },
            );
        }

        if request.include_inferred {
            if let Some(inferred) = self.inferred_edges {
                for edge in inferred {
                    if !self.entity_allowed(&edge.child, &request.filters)
                        || !self.entity_allowed(&edge.parent, &request.filters)
                    {
                        continue;
                    }
                    let is_new = !hierarchy
                        .edges
                        .iter()
                        .any(|e| e.child == edge.child && e.parent == edge.parent);
                    if !is_new {
                        continue;
                    }
                    self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, &edge.child);
                    self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, &edge.parent);
                    self.add_edge(
                        &mut edges,
                        &mut truncated,
                        &request.filters,
                        GraphEdge {
                            source: edge.child.clone(),
                            target: edge.parent.clone(),
                            kind: "sub_class_of".to_string(),
                            inferred: true,
                        },
                    );
                }
            }
        }

        Ok(GraphPayload { nodes, edges, truncated, graph_kind: String::new() })
    }

    fn build_property_graph(
        &self,
        request: &GraphRequest,
        kind_filter: Option<EntityKind>,
    ) -> Result<GraphPayload, String> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_ids = HashSet::new();
        let mut truncated = false;

        for entity in &self.catalog.data().entities {
            let matches_kind = match kind_filter {
                Some(k) => entity.kind == k,
                None => {
                    entity.kind == EntityKind::ObjectProperty
                        || entity.kind == EntityKind::DataProperty
                }
            };
            if !matches_kind {
                continue;
            }
            if !self.entity_allowed(&entity.iri, &request.filters) {
                continue;
            }
            self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, &entity.iri);
        }

        for axiom in &self.catalog.data().axioms {
            let edge_kind = match axiom.axiom_kind.as_str() {
                AXIOM_KIND_DOMAIN => "domain",
                AXIOM_KIND_RANGE => "range",
                _ => continue,
            };
            let Some(prop) = self.catalog.find_entity(&axiom.subject) else {
                continue;
            };
            let matches_kind = match kind_filter {
                Some(k) => prop.kind == k,
                None => {
                    prop.kind == EntityKind::ObjectProperty || prop.kind == EntityKind::DataProperty
                }
            };
            if !matches_kind {
                continue;
            }
            if !self.entity_allowed(&axiom.subject, &request.filters) {
                continue;
            }
            self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, &axiom.object);
            self.add_edge(
                &mut edges,
                &mut truncated,
                &request.filters,
                GraphEdge {
                    source: axiom.subject.clone(),
                    target: axiom.object.clone(),
                    kind: edge_kind.to_string(),
                    inferred: false,
                },
            );
        }

        Ok(GraphPayload { nodes, edges, truncated, graph_kind: String::new() })
    }

    fn build_property_hierarchy_graph(
        &self,
        request: &GraphRequest,
        prop_kind: EntityKind,
    ) -> Result<GraphPayload, String> {
        let mut payload = self.build_property_graph(request, Some(prop_kind))?;
        let axiom_kind = match prop_kind {
            EntityKind::ObjectProperty => AXIOM_KIND_SUB_OBJECT_PROPERTY_OF,
            EntityKind::DataProperty => AXIOM_KIND_SUB_DATA_PROPERTY_OF,
            _ => return Ok(payload),
        };
        let mut node_ids: HashSet<String> = payload.nodes.iter().map(|n| n.id.clone()).collect();
        let mut truncated = payload.truncated;

        for axiom in &self.catalog.data().axioms {
            if axiom.axiom_kind != axiom_kind {
                continue;
            }
            if !self.entity_allowed(&axiom.subject, &request.filters)
                || !self.entity_allowed(&axiom.object, &request.filters)
            {
                continue;
            }
            self.add_node_for(&mut payload.nodes, &mut node_ids, &mut truncated, &axiom.subject);
            self.add_node_for(&mut payload.nodes, &mut node_ids, &mut truncated, &axiom.object);
            self.add_edge(
                &mut payload.edges,
                &mut truncated,
                &request.filters,
                GraphEdge {
                    source: axiom.subject.clone(),
                    target: axiom.object.clone(),
                    kind: "sub_property_of".to_string(),
                    inferred: false,
                },
            );
        }

        // Subclass-style inferred edges are class-only today; property inferred overlays
        // fall back to asserted hierarchy (documented UI behavior).
        payload.truncated = truncated;
        Ok(payload)
    }

    fn build_import_graph(
        &self,
        request: &GraphRequest,
        full_closure: bool,
    ) -> Result<GraphPayload, String> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_ids = HashSet::new();
        let mut truncated = false;

        let mut adjacency: Vec<(String, String)> = Vec::new();
        for doc in &self.catalog.data().documents {
            let ont_iri = doc.base_iri.clone().unwrap_or_else(|| doc.id.clone());
            for import in &doc.imports {
                adjacency.push((ont_iri.clone(), import.clone()));
            }
        }

        let seeds: Vec<String> = if let Some(ref filter) = request.filters.ontology_iri {
            self.catalog
                .data()
                .documents
                .iter()
                .filter_map(|doc| {
                    let ont_iri = doc.base_iri.clone().unwrap_or_else(|| doc.id.clone());
                    if &ont_iri == filter || &doc.id == filter {
                        Some(ont_iri)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            self.catalog
                .data()
                .documents
                .iter()
                .map(|doc| doc.base_iri.clone().unwrap_or_else(|| doc.id.clone()))
                .collect()
        };

        if full_closure {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            for seed in &seeds {
                queue.push_back(seed.clone());
                visited.insert(seed.clone());
            }
            while let Some(current) = queue.pop_front() {
                self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, &current);
                for (src, tgt) in &adjacency {
                    if src != &current {
                        continue;
                    }
                    self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, tgt);
                    self.add_edge(
                        &mut edges,
                        &mut truncated,
                        &request.filters,
                        GraphEdge {
                            source: src.clone(),
                            target: tgt.clone(),
                            kind: "imports".to_string(),
                            inferred: false,
                        },
                    );
                    if visited.insert(tgt.clone()) {
                        queue.push_back(tgt.clone());
                    }
                }
            }
        } else {
            for seed in &seeds {
                self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, seed);
            }
            for (src, tgt) in &adjacency {
                if request.filters.ontology_iri.is_some() && !seeds.iter().any(|s| s == src) {
                    continue;
                }
                self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, src);
                self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, tgt);
                self.add_edge(
                    &mut edges,
                    &mut truncated,
                    &request.filters,
                    GraphEdge {
                        source: src.clone(),
                        target: tgt.clone(),
                        kind: "imports".to_string(),
                        inferred: false,
                    },
                );
            }
        }

        Ok(GraphPayload { nodes, edges, truncated, graph_kind: String::new() })
    }

    fn build_individual_graph(
        &self,
        request: &GraphRequest,
        roots: &[String],
        depth: u32,
    ) -> Result<GraphPayload, String> {
        let mut adjacency: Vec<(String, String, String, bool)> = Vec::new();
        for axiom in &self.catalog.data().axioms {
            match axiom.axiom_kind.as_str() {
                AXIOM_KIND_CLASS_ASSERTION => {
                    adjacency.push((
                        axiom.subject.clone(),
                        axiom.object.clone(),
                        "type".to_string(),
                        false,
                    ));
                    adjacency.push((
                        axiom.object.clone(),
                        axiom.subject.clone(),
                        "instance".to_string(),
                        false,
                    ));
                }
                AXIOM_KIND_OBJECT_PROPERTY_ASSERTION => {
                    adjacency.push((
                        axiom.subject.clone(),
                        axiom.object.clone(),
                        "assertion".to_string(),
                        false,
                    ));
                    adjacency.push((
                        axiom.object.clone(),
                        axiom.subject.clone(),
                        "assertion".to_string(),
                        false,
                    ));
                    // Also surface the property as a connector via predicate label on edges.
                    let _ = &axiom.predicate;
                }
                _ => {}
            }
        }
        self.bfs_from_roots(request, roots, depth, &adjacency, "individual")
    }

    fn build_neighborhood_graph(
        &self,
        request: &GraphRequest,
        root: &str,
        depth: u32,
    ) -> Result<GraphPayload, String> {
        let adjacency = self.class_neighborhood_adjacency(request.include_inferred);
        self.bfs_from_roots(request, &[root.to_string()], depth, &adjacency, "neighborhood")
    }

    fn build_seeded_result_graph(
        &self,
        request: &GraphRequest,
        roots: &[String],
        depth: u32,
        kind: GraphKind,
    ) -> Result<GraphPayload, String> {
        let adjacency = self.class_neighborhood_adjacency(request.include_inferred);
        let edge_kind_prefix = match kind {
            GraphKind::QueryResult => "query",
            GraphKind::RefactorPreview => "refactor",
            _ => "seed",
        };
        let mut payload =
            self.bfs_from_roots(request, roots, depth, &adjacency, edge_kind_prefix)?;
        // Ensure every seed appears even when isolated.
        let mut node_ids: HashSet<String> = payload.nodes.iter().map(|n| n.id.clone()).collect();
        let mut truncated = payload.truncated;
        for root in roots {
            self.add_node_for(&mut payload.nodes, &mut node_ids, &mut truncated, root);
        }
        // Link consecutive seeds so multi-root results form a visible connected set.
        if roots.len() > 1 {
            for window in roots.windows(2) {
                self.add_edge(
                    &mut payload.edges,
                    &mut truncated,
                    &request.filters,
                    GraphEdge {
                        source: window[0].clone(),
                        target: window[1].clone(),
                        kind: format!("{edge_kind_prefix}_result"),
                        inferred: false,
                    },
                );
            }
        }
        payload.truncated = truncated;
        Ok(payload)
    }

    fn class_neighborhood_adjacency(
        &self,
        include_inferred: bool,
    ) -> Vec<(String, String, String, bool)> {
        let hierarchy = self.catalog.class_hierarchy();
        let mut adjacency: Vec<(String, String, String, bool)> = Vec::new();

        for edge in &hierarchy.edges {
            adjacency.push((
                edge.child.clone(),
                edge.parent.clone(),
                "sub_class_of".to_string(),
                false,
            ));
            adjacency.push((
                edge.parent.clone(),
                edge.child.clone(),
                "super_class_of".to_string(),
                false,
            ));
        }

        if include_inferred {
            if let Some(inferred) = self.inferred_edges {
                for edge in inferred {
                    adjacency.push((
                        edge.child.clone(),
                        edge.parent.clone(),
                        "sub_class_of".to_string(),
                        true,
                    ));
                    adjacency.push((
                        edge.parent.clone(),
                        edge.child.clone(),
                        "super_class_of".to_string(),
                        true,
                    ));
                }
            }
        }

        for axiom in &self.catalog.data().axioms {
            if axiom.axiom_kind == AXIOM_KIND_EQUIVALENT_CLASS
                && (axiom.object.starts_with("http://") || axiom.object.starts_with("https://"))
            {
                adjacency.push((
                    axiom.subject.clone(),
                    axiom.object.clone(),
                    "equivalent_class".to_string(),
                    false,
                ));
                adjacency.push((
                    axiom.object.clone(),
                    axiom.subject.clone(),
                    "equivalent_class".to_string(),
                    false,
                ));
            } else if axiom.axiom_kind == AXIOM_KIND_SUB_CLASS_OF
                && !axiom.object.starts_with("http://")
                && !axiom.object.starts_with("https://")
            {
                for filler in restriction_fillers_in_expr(&axiom.object, self.catalog) {
                    adjacency.push((
                        axiom.subject.clone(),
                        filler,
                        "some_values_from".to_string(),
                        false,
                    ));
                }
            }
        }
        adjacency
    }

    fn bfs_from_roots(
        &self,
        request: &GraphRequest,
        roots: &[String],
        depth: u32,
        adjacency: &[(String, String, String, bool)],
        _tag: &str,
    ) -> Result<GraphPayload, String> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_ids = HashSet::new();
        let mut truncated = false;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        for root in roots {
            queue.push_back((root.clone(), 0u32));
            visited.insert(root.clone());
            self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, root);
        }

        while let Some((current, d)) = queue.pop_front() {
            if d >= depth {
                continue;
            }
            for (src, tgt, kind, inferred) in adjacency {
                if src != &current {
                    continue;
                }
                if !self.entity_allowed(tgt, &request.filters) {
                    continue;
                }
                self.add_node_for(&mut nodes, &mut node_ids, &mut truncated, tgt);
                self.add_edge(
                    &mut edges,
                    &mut truncated,
                    &request.filters,
                    GraphEdge {
                        source: src.clone(),
                        target: tgt.clone(),
                        kind: kind.clone(),
                        inferred: *inferred,
                    },
                );
                if visited.insert(tgt.clone()) {
                    queue.push_back((tgt.clone(), d + 1));
                }
            }
        }

        Ok(GraphPayload { nodes, edges, truncated, graph_kind: String::new() })
    }
}

fn collect_roots(request: &GraphRequest) -> Vec<String> {
    let mut roots = request.root_iris.clone();
    if let Some(ref root) = request.root_iri {
        if !roots.iter().any(|r| r == root) {
            roots.insert(0, root.clone());
        }
    }
    roots
}

fn graph_node_from_entity(entity: &Entity) -> GraphNode {
    GraphNode {
        id: entity.iri.clone(),
        label: entity.labels.first().cloned().unwrap_or_else(|| entity.short_name.clone()),
        kind: entity.kind.as_str().to_string(),
        namespace: Some(namespace_of(&entity.iri)),
        ontology_iri: Some(entity.ontology_id.clone()),
        deprecated: entity.deprecated,
        unsatisfiable: false,
        equivalent: false,
    }
}

fn namespace_of(iri: &str) -> String {
    if let Some(h) = iri.rfind('#') {
        return iri[..=h].to_string();
    }
    if let Some(s) = iri.rfind('/') {
        return iri[..=s].to_string();
    }
    iri.to_string()
}

fn short_name(iri: &str) -> String {
    let hash = iri.rfind('#');
    let slash = iri.rfind('/');
    match (hash, slash) {
        (Some(h), Some(s)) => iri[h.max(s) + 1..].to_string(),
        (Some(h), None) => iri[h + 1..].to_string(),
        (None, Some(s)) => iri[s + 1..].to_string(),
        _ => iri.to_string(),
    }
}

fn restriction_fillers_in_expr(expr: &str, catalog: &OntologyCatalog) -> Vec<String> {
    catalog
        .data()
        .entities
        .iter()
        .filter(|e| e.kind == EntityKind::Class)
        .filter(|e| expr.contains(&e.iri) || expr.contains(&format!(":{}", e.short_name)))
        .map(|e| e.iri.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IndexBuilder;
    use std::path::Path;

    fn default_request(kind: &str) -> GraphRequest {
        GraphRequest {
            graph_kind: kind.to_string(),
            root_iri: None,
            root_iris: vec![],
            depth: 2,
            include_inferred: false,
            filters: GraphFilters::default(),
        }
    }

    #[test]
    fn class_graph_from_fixtures() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&root).build().expect("build");
        let payload = GraphBuilder::new(&catalog).build(&default_request("class")).expect("graph");
        assert!(!payload.nodes.is_empty());
        assert!(!payload.edges.is_empty());
        assert!(payload.nodes.iter().any(|n| n.namespace.is_some()));
    }

    #[test]
    fn import_graph_from_fixtures() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&root).build().expect("build");
        let payload = GraphBuilder::new(&catalog).build(&default_request("import")).expect("graph");
        assert!(!payload.nodes.is_empty());
    }

    #[test]
    fn dependency_graph_from_fixtures() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&root).build().expect("build");
        let payload =
            GraphBuilder::new(&catalog).build(&default_request("dependency")).expect("graph");
        assert!(!payload.nodes.is_empty());
        assert_eq!(payload.graph_kind, "dependency");
    }

    #[test]
    fn property_graph_includes_domain_range_from_axioms() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&root).build().expect("build");
        let payload =
            GraphBuilder::new(&catalog).build(&default_request("property")).expect("graph");
        assert!(
            payload.edges.iter().any(|e| e.kind == "domain"),
            "expected domain edges from axioms"
        );
        assert!(
            payload.edges.iter().any(|e| e.kind == "range"),
            "expected range edges from axioms"
        );
    }

    #[test]
    fn object_property_hierarchy_includes_sub_property_edges() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("props.ttl"),
            "@prefix ex: <http://ex#> .\n\
             @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
             @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
             <http://ex/onto> a owl:Ontology .\n\
             ex:hasRelative a owl:ObjectProperty .\n\
             ex:hasParent a owl:ObjectProperty ; rdfs:subPropertyOf ex:hasRelative .\n",
        )
        .expect("write");
        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
        let payload =
            GraphBuilder::new(&catalog).build(&default_request("object_property")).expect("graph");
        assert!(
            payload.edges.iter().any(|e| e.kind == "sub_property_of"),
            "expected sub_property_of edges: {:?}",
            payload.edges
        );
    }

    #[test]
    fn individual_graph_includes_type_and_assertions() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("inds.ttl"),
            "@prefix ex: <http://ex#> .\n\
             @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
             @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n\
             <http://ex/onto> a owl:Ontology .\n\
             ex:Person a owl:Class .\n\
             ex:knows a owl:ObjectProperty .\n\
             ex:Alice a owl:NamedIndividual , ex:Person .\n\
             ex:Bob a owl:NamedIndividual .\n\
             ex:Alice ex:knows ex:Bob .\n",
        )
        .expect("write");
        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
        let mut req = default_request("individual");
        req.root_iri = Some("http://ex#Alice".to_string());
        let payload = GraphBuilder::new(&catalog).build(&req).expect("graph");
        assert!(payload.nodes.iter().any(|n| n.id == "http://ex#Alice"));
        assert!(
            payload.edges.iter().any(|e| e.kind == "type" || e.kind == "assertion"),
            "expected type/assertion edges: {:?}",
            payload.edges
        );
    }

    #[test]
    fn query_result_graph_seeds_roots() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&root).build().expect("build");
        let mut req = default_request("query_result");
        req.root_iris = vec![
            "http://example.org/clinic#Patient".to_string(),
            "http://example.org/clinic#MedicalRecord".to_string(),
        ];
        let payload = GraphBuilder::new(&catalog).build(&req).expect("graph");
        assert!(payload.nodes.iter().any(|n| n.id.contains("Patient")));
        assert_eq!(payload.graph_kind, "query_result");
    }

    #[test]
    fn neighborhood_graph_includes_restriction_fillers_from_axioms() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&root).build().expect("build");
        let patient = "http://example.org/clinic#Patient";
        let record = "http://example.org/clinic#MedicalRecord";
        let mut req = default_request("neighborhood");
        req.root_iri = Some(patient.to_string());
        let payload = GraphBuilder::new(&catalog).build(&req).expect("graph");
        assert!(
            payload.edges.iter().any(|e| e.source == patient && e.target == record),
            "expected Patient -> MedicalRecord restriction edge"
        );
    }

    #[test]
    fn ontology_iri_filter_excludes_unknown_entities() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("local.ttl"),
            "@prefix ex: <http://ex#> .\n\
             @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
             @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
             <http://ex/onto> a owl:Ontology .\n\
             ex:Child a owl:Class ; rdfs:subClassOf <http://external.example/Parent> .\n",
        )
        .expect("write");
        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
        let ont_id = catalog.data().documents.first().map(|d| d.id.clone()).expect("doc");
        let unfiltered =
            GraphBuilder::new(&catalog).build(&default_request("class")).expect("unfiltered");
        assert!(
            unfiltered.nodes.iter().any(|n| n.id == "http://external.example/Parent"),
            "unknown parent should appear without ontology filter"
        );

        let mut filtered_req = default_request("class");
        filtered_req.filters =
            GraphFilters { ontology_iri: Some(ont_id), ..GraphFilters::default() };
        let filtered = GraphBuilder::new(&catalog).build(&filtered_req).expect("filtered");
        assert!(
            filtered.nodes.iter().all(|n| n.id != "http://external.example/Parent"),
            "unknown parent must not bypass ontology_iri filter"
        );
        assert!(
            filtered.edges.iter().all(|e| {
                e.source != "http://external.example/Parent"
                    && e.target != "http://external.example/Parent"
            }),
            "edges to unknown parent must be excluded when filtering"
        );
    }

    #[test]
    fn search_text_and_entity_kind_filters() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        let catalog = IndexBuilder::new().workspace(&root).build().expect("build");
        let mut req = default_request("class");
        req.filters.search_text = Some("Patient".to_string());
        req.filters.entity_kinds = vec!["class".to_string()];
        let payload = GraphBuilder::new(&catalog).build(&req).expect("graph");
        assert!(payload.nodes.iter().all(|n| {
            n.kind == "class"
                && (n.label.to_lowercase().contains("patient")
                    || n.id.to_lowercase().contains("patient"))
        }));
    }

    #[test]
    fn large_graph_truncation_cap_is_honored() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut ttl = String::from(
            "@prefix ex: <http://ex#> .\n\
             @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
             @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
             <http://ex/onto> a owl:Ontology .\n\
             ex:Root a owl:Class .\n",
        );
        // Enough subclass edges that truncation may trigger under low test caps —
        // assert non-empty and that builder completes quickly for a sizable hierarchy.
        for i in 0..200 {
            ttl.push_str(&format!("ex:C{i} a owl:Class ; rdfs:subClassOf ex:Root .\n"));
        }
        std::fs::write(dir.path().join("large.ttl"), ttl).expect("write");
        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
        let started = std::time::Instant::now();
        let payload = GraphBuilder::new(&catalog).build(&default_request("class")).expect("graph");
        assert!(started.elapsed().as_secs() < 5, "class graph build too slow");
        assert!(!payload.nodes.is_empty());
        assert!(payload.nodes.len() <= MAX_GRAPH_NODES);
        assert!(payload.edges.len() <= MAX_GRAPH_EDGES);
    }
}
