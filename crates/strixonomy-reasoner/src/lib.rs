//! OntoLogos-backed reasoner facade for Strixonomy (v0.9).
//!
//! Published as [`strixonomy-reasoner`](https://crates.io/crates/strixonomy-reasoner).

mod abox;
mod adapter;
mod auto;
mod cache;
mod dl;
mod dl_query;
mod el;
mod error;
mod explain;
mod hierarchy;
mod input;
mod rdfs;
mod result;
mod rl;
mod runner;
mod swrl_run;

pub use abox::{
    cancel_requested, check_full_consistency, clear_cancel_flag,
    inferred_assertions_from_realization, install_cancel_flag,
};
pub use adapter::{ReasonerAdapter, ReasonerId, ReasonerProfile};
pub use cache::{ReasonerCache, ReasonerCacheStore};
pub use dl_query::{run_dl_query, DlQueryMode, DlQueryResult, DL_QUERY_CLASS_IRI};
pub use error::{ReasonerError, Result};
pub use input::{ReasonerInput, WorkspaceInputLoader};
pub use result::{
    expand_named_unsatisfiable, ClassificationResult, ConsistencyDetail, ConsistencyResult,
    ExplanationRequest, ExplanationResult, ExplanationStep, InferredAssertions,
    InferredClassAssertion, InferredHierarchy, InferredObjectPropertyAssertion,
    InstanceCheckResult, RealizationEntry, RealizationResult, ReasonerSnapshot, ReasonerWarning,
    SameAsCluster,
};
pub use runner::{
    check_consistency, check_instance, classify, explain, explain_alternatives,
    inferred_assertions, realize,
};
pub use swrl_run::{
    classify_with_swrl, inject_swrl_from_turtle, input_has_swrl_rules, ontology_has_swrl_rules,
    prepare_swrl_ontology,
};
