use crate::adapter::{ReasonerAdapter, ReasonerId, ReasonerProfile};
use crate::error::{ReasonerError, Result};
use crate::explain::{
    explain_unsatisfiable_dl, explain_unsatisfiable_el, explain_unsatisfiable_rdfs,
    explain_unsatisfiable_rl,
};
use crate::hierarchy::subclass_edges_from_ontology;
use crate::input::ReasonerInput;
use crate::result::{
    build_inferred_hierarchy, detect_unsatisfiable_classes, new_inferences,
    taxonomy_equivalence_clusters, taxonomy_to_iri_edges, unsatisfiable_iris, ClassificationResult,
    ExplanationRequest, ExplanationResult,
};
use ontologos_core::{EngineKind, Ontology, Profile, Reasoner};
use ontologos_facade::ClassifyOutcome;
use std::time::Instant;

pub struct AutoAdapter;

/// Resolve OntoLogos Auto routing to the concrete Strixonomy reasoner id.
///
/// Used by classify (`profile_used`) and by both CLI/LSP explain paths so they
/// share the same backend for a given ontology.
pub(crate) fn resolve_auto_reasoner_id(ontology: &Ontology) -> Result<ReasonerId> {
    let route = ontologos_profile::resolve_route(Profile::Auto, ontology)
        .map_err(|e| ReasonerError::Classify(e.to_string()))?;
    Ok(match route.kind {
        EngineKind::El => ReasonerId::El,
        EngineKind::Rl => ReasonerId::Rl,
        EngineKind::Rdfs => ReasonerId::Rdfs,
        EngineKind::Swrl => ReasonerId::Dl, // SWRL materializes then DL; surface via warnings
        EngineKind::Alc | EngineKind::Dl | EngineKind::Hybrid => ReasonerId::Dl,
    })
}

fn explain_for_concrete(
    concrete: ReasonerId,
    ontology: &Ontology,
    class_iri: &str,
) -> Result<ExplanationResult> {
    match concrete {
        ReasonerId::Dl => explain_unsatisfiable_dl(ontology, class_iri),
        ReasonerId::Rdfs => explain_unsatisfiable_rdfs(ontology, class_iri),
        ReasonerId::Rl => explain_unsatisfiable_rl(ontology, class_iri),
        ReasonerId::El | ReasonerId::Auto => explain_unsatisfiable_el(ontology, class_iri),
    }
}

impl ReasonerAdapter for AutoAdapter {
    fn id(&self) -> ReasonerId {
        ReasonerId::Auto
    }

    fn profile(&self) -> ReasonerProfile {
        ReasonerProfile::Auto
    }

    fn classify(&self, input: &ReasonerInput) -> Result<ClassificationResult> {
        let started = Instant::now();
        let mut reasoner = Reasoner::builder()
            .profile(Profile::Auto)
            .build(input.ontology.clone())
            .map_err(|e| ReasonerError::Classify(e.to_string()))?;

        match ontologos_facade::classify(&mut reasoner)
            .map_err(|e| ReasonerError::Classify(e.to_string()))?
        {
            ClassifyOutcome::Taxonomy(taxonomy) => {
                let iri_edges = taxonomy_to_iri_edges(&input.ontology, &taxonomy)
                    .map_err(ReasonerError::Classify)?;
                let equivalences = taxonomy_equivalence_clusters(&input.ontology, &taxonomy)
                    .map_err(ReasonerError::Classify)?;
                let reported = unsatisfiable_iris(&input.ontology, &taxonomy)
                    .map_err(ReasonerError::Classify)?;
                let inferred =
                    build_inferred_hierarchy(&iri_edges, &reported, &input.asserted_hierarchy);
                let unsatisfiable = inferred.unsatisfiable.clone();
                let new_inferences = new_inferences(&input.asserted_hierarchy, &inferred.edges);
                let profile_used = resolve_auto_reasoner_id(&input.ontology)?.as_str().to_string();

                Ok(ClassificationResult {
                    profile_used,
                    consistent: unsatisfiable.is_empty(),
                    unsatisfiable,
                    inferred,
                    new_inferences,
                    warnings: Vec::new(),
                    duration_ms: started.elapsed().as_millis() as u64,
                    subsumption_count: taxonomy.subsumption_count(),
                    inferred_axiom_count: taxonomy.subsumption_count(),
                    equivalences,
                })
            }
            ClassifyOutcome::Rdfs(report) => {
                let ontology = reasoner.ontology();
                let iri_edges = subclass_edges_from_ontology(ontology, &input.asserted_hierarchy);
                let reported =
                    detect_unsatisfiable_classes(ontology).map_err(ReasonerError::Classify)?;
                let inferred =
                    build_inferred_hierarchy(&iri_edges, &reported, &input.asserted_hierarchy);
                let unsatisfiable = inferred.unsatisfiable.clone();
                let new_inferences = new_inferences(&input.asserted_hierarchy, &inferred.edges);

                Ok(ClassificationResult {
                    profile_used: "rdfs".to_string(),
                    consistent: unsatisfiable.is_empty(),
                    unsatisfiable,
                    inferred,
                    new_inferences,
                    warnings: Vec::new(),
                    duration_ms: started.elapsed().as_millis() as u64,
                    subsumption_count: iri_edges.len(),
                    inferred_axiom_count: report.inferred_total(),
                    equivalences: Vec::new(),
                })
            }
            ClassifyOutcome::Rl(report) => {
                let ontology = reasoner.ontology();
                let iri_edges = subclass_edges_from_ontology(ontology, &input.asserted_hierarchy);
                let reported =
                    detect_unsatisfiable_classes(ontology).map_err(ReasonerError::Classify)?;
                let inferred =
                    build_inferred_hierarchy(&iri_edges, &reported, &input.asserted_hierarchy);
                let unsatisfiable = inferred.unsatisfiable.clone();
                let new_inferences = new_inferences(&input.asserted_hierarchy, &inferred.edges);

                Ok(ClassificationResult {
                    profile_used: "rl".to_string(),
                    consistent: unsatisfiable.is_empty(),
                    unsatisfiable,
                    inferred,
                    new_inferences,
                    warnings: Vec::new(),
                    duration_ms: started.elapsed().as_millis() as u64,
                    subsumption_count: iri_edges.len(),
                    inferred_axiom_count: report.inferred_total(),
                    equivalences: Vec::new(),
                })
            }
            _ => {
                Err(ReasonerError::Classify("unsupported auto classification outcome".to_string()))
            }
        }
    }

    fn explain(
        &self,
        input: &ReasonerInput,
        request: &ExplanationRequest,
    ) -> Result<ExplanationResult> {
        let concrete = resolve_auto_reasoner_id(&input.ontology)?;
        explain_for_concrete(concrete, &input.ontology, &request.class_iri)
    }
}
