//! Reasoning integration via OntoLogos.

pub use strixonomy_reasoner::{
    cancel_requested, check_consistency, check_instance, classify, clear_cancel_flag, explain,
    explain_alternatives, inferred_assertions, install_cancel_flag, realize, run_dl_query,
    ClassificationResult, ConsistencyDetail, ConsistencyResult, DlQueryMode, DlQueryResult,
    ExplanationRequest, ExplanationResult, ExplanationStep, InferredAssertions,
    InferredClassAssertion, InferredHierarchy, InferredObjectPropertyAssertion,
    InstanceCheckResult, RealizationEntry, RealizationResult, ReasonerAdapter, ReasonerCache,
    ReasonerCacheStore, ReasonerError, ReasonerId, ReasonerInput, ReasonerProfile,
    ReasonerSnapshot, ReasonerWarning, SameAsCluster, WorkspaceInputLoader, DL_QUERY_CLASS_IRI,
};
