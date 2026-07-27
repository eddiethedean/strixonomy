use crate::error::{ReasonerError, Result};
use crate::input::ReasonerInput;
use crate::result::{
    ClassificationResult, ConsistencyDetail, ConsistencyResult, ExplanationRequest,
    ExplanationResult, InferredAssertions, InstanceCheckResult, RealizationResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasonerId {
    El,
    Rl,
    Rdfs,
    Dl,
    Auto,
}

impl ReasonerId {
    pub fn parse(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "el" => Ok(Self::El),
            "rl" => Ok(Self::Rl),
            "rdfs" => Ok(Self::Rdfs),
            "dl" => Ok(Self::Dl),
            "auto" => Ok(Self::Auto),
            _ => Err(ReasonerError::UnsupportedProfile(s.to_string())),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::El => "el",
            Self::Rl => "rl",
            Self::Rdfs => "rdfs",
            Self::Dl => "dl",
            Self::Auto => "auto",
        }
    }

    pub fn is_available(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasonerProfile {
    OwlEl,
    OwlRl,
    Rdfs,
    OwlDl,
    Auto,
}

impl From<ReasonerId> for ReasonerProfile {
    fn from(id: ReasonerId) -> Self {
        match id {
            ReasonerId::El => Self::OwlEl,
            ReasonerId::Rl => Self::OwlRl,
            ReasonerId::Rdfs => Self::Rdfs,
            ReasonerId::Dl => Self::OwlDl,
            ReasonerId::Auto => Self::Auto,
        }
    }
}

pub trait ReasonerAdapter: Send + Sync {
    fn id(&self) -> ReasonerId;
    fn profile(&self) -> ReasonerProfile;
    fn classify(&self, input: &ReasonerInput) -> Result<ClassificationResult>;
    fn check_consistency(&self, input: &ReasonerInput) -> Result<ConsistencyResult> {
        let result = self.classify(input)?;
        let detail =
            crate::abox::check_full_consistency(&input.ontology, self.id(), &result.unsatisfiable)?;
        Ok(ConsistencyResult {
            consistent: detail.consistent,
            unsatisfiable: result.unsatisfiable.clone(),
            detail: Some(detail),
        })
    }
    fn realize(&self, input: &ReasonerInput) -> Result<RealizationResult> {
        crate::abox::realize(&input.ontology, self.id())
    }
    fn check_instance(
        &self,
        input: &ReasonerInput,
        individual_iri: &str,
        class_iri: &str,
    ) -> Result<InstanceCheckResult> {
        crate::abox::check_instance(&input.ontology, self.id(), individual_iri, class_iri)
    }
    fn inferred_assertions(&self, input: &ReasonerInput) -> Result<InferredAssertions> {
        crate::abox::inferred_assertions(&input.ontology, self.id())
    }
    fn consistency_detail(
        &self,
        input: &ReasonerInput,
        unsatisfiable: &[String],
    ) -> Result<ConsistencyDetail> {
        crate::abox::check_full_consistency(&input.ontology, self.id(), unsatisfiable)
    }
    fn explain(
        &self,
        input: &ReasonerInput,
        request: &ExplanationRequest,
    ) -> Result<ExplanationResult>;
}
