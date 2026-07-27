use crate::adapter::{ReasonerAdapter, ReasonerId, ReasonerProfile};
use crate::error::{ReasonerError, Result};
use crate::explain::explain_unsatisfiable_el;
use crate::input::ReasonerInput;
use crate::result::{
    build_inferred_hierarchy, new_inferences, taxonomy_equivalence_clusters, taxonomy_to_iri_edges,
    unsatisfiable_iris, ClassificationResult, ExplanationRequest, ExplanationResult,
};
use ontologos_el::ElClassifier;
use std::time::Instant;

pub struct ElAdapter;

impl ReasonerAdapter for ElAdapter {
    fn id(&self) -> ReasonerId {
        ReasonerId::El
    }

    fn profile(&self) -> ReasonerProfile {
        ReasonerProfile::OwlEl
    }

    fn classify(&self, input: &ReasonerInput) -> Result<ClassificationResult> {
        let started = Instant::now();
        let taxonomy = ElClassifier::new()
            .classify(&input.ontology)
            .map_err(|e| ReasonerError::Classify(e.to_string()))?;

        let iri_edges =
            taxonomy_to_iri_edges(&input.ontology, &taxonomy).map_err(ReasonerError::Classify)?;
        let equivalences = taxonomy_equivalence_clusters(&input.ontology, &taxonomy)
            .map_err(ReasonerError::Classify)?;
        let reported =
            unsatisfiable_iris(&input.ontology, &taxonomy).map_err(ReasonerError::Classify)?;
        let inferred = build_inferred_hierarchy(&iri_edges, &reported, &input.asserted_hierarchy);
        let unsatisfiable = inferred.unsatisfiable.clone();
        let new_inferences = new_inferences(&input.asserted_hierarchy, &inferred.edges);

        Ok(ClassificationResult {
            profile_used: "el".to_string(),
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

    fn explain(
        &self,
        input: &ReasonerInput,
        request: &ExplanationRequest,
    ) -> Result<ExplanationResult> {
        explain_unsatisfiable_el(&input.ontology, &request.class_iri)
    }
}
