use crate::adapter::{ReasonerAdapter, ReasonerId};
use crate::auto::AutoAdapter;
use crate::dl::DlAdapter;
use crate::el::ElAdapter;
use crate::error::{ReasonerError, Result};
use crate::input::ReasonerInput;
use crate::rdfs::RdfsAdapter;
use crate::result::{
    ClassificationResult, ConsistencyResult, ExplanationRequest, ExplanationResult,
    InferredAssertions, InstanceCheckResult, RealizationResult,
};
use crate::rl::RlAdapter;
use ontologos_profile::scanner::scan_constructs;

pub fn adapter_for(id: ReasonerId) -> Result<Box<dyn ReasonerAdapter>> {
    let adapter: Box<dyn ReasonerAdapter> = match id {
        ReasonerId::El => Box::new(ElAdapter),
        ReasonerId::Rl => Box::new(RlAdapter),
        ReasonerId::Rdfs => Box::new(RdfsAdapter),
        ReasonerId::Dl => Box::new(DlAdapter),
        ReasonerId::Auto => Box::new(AutoAdapter),
    };
    Ok(adapter)
}

pub fn classify(
    profile: ReasonerId,
    input: &ReasonerInput,
    auto_detect: bool,
) -> Result<ClassificationResult> {
    let mut warnings = profile_warnings(&input.ontology, profile, auto_detect)?;
    if auto_detect {
        if let Some(suggested) = suggest_profile(&input.ontology) {
            if suggested != profile.as_str() {
                warnings.push(crate::result::ReasonerWarning {
                    code: "profile_suggestion".to_string(),
                    message: format!(
                        "ontology may be better suited to profile '{suggested}' (selected: {})",
                        profile.as_str()
                    ),
                    suggested_profile: Some(suggested),
                });
            }
        }
    }
    let adapter = adapter_for(profile)?;
    let swrl_present = crate::swrl_run::input_has_swrl_rules(input);
    let mut used_swrl = false;
    let mut result = if swrl_present && matches!(profile, ReasonerId::Dl | ReasonerId::Auto) {
        match crate::swrl_run::classify_with_swrl(input) {
            Ok(r) => {
                used_swrl = true;
                r
            }
            Err(e) => {
                warnings.push(crate::result::ReasonerWarning {
                    code: "swrl_classify_failed".to_string(),
                    message: format!(
                        "SWRL-aware classify failed ({e}); falling back to {profile:?} without SWRL materialization"
                    ),
                    suggested_profile: None,
                });
                adapter.classify(input)?
            }
        }
    } else {
        if swrl_present {
            // #360 — non-DL/Auto profiles must not silently ignore SWRL.
            warnings.push(crate::result::ReasonerWarning {
                code: "swrl_skipped_for_profile".to_string(),
                message: format!(
                    "SWRL rules are present but profile '{}' does not run SWRL materialization; use profile 'dl' or 'auto'",
                    profile.as_str()
                ),
                suggested_profile: Some("dl".to_string()),
            });
        }
        adapter.classify(input)?
    };
    // When SWRL materialization ran, consistency was already checked on the
    // post-materialization ontology inside classify_with_swrl (#358).
    if !used_swrl {
        if let Ok(detail) =
            crate::abox::check_full_consistency(&input.ontology, profile, &result.unsatisfiable)
        {
            result.consistent = detail.consistent;
            if !detail.abox_clashes.is_empty() {
                for clash in &detail.abox_clashes {
                    warnings.push(crate::result::ReasonerWarning {
                        code: "abox_clash".to_string(),
                        message: clash.clone(),
                        suggested_profile: None,
                    });
                }
            }
            if !detail.complete {
                warnings.push(crate::result::ReasonerWarning {
                    code: "consistency_incomplete".to_string(),
                    message: "consistency check did not complete (budget or cancel)".into(),
                    suggested_profile: None,
                });
            }
        }
    }
    result.warnings.extend(warnings);
    Ok(result)
}

pub fn check_consistency(profile: ReasonerId, input: &ReasonerInput) -> Result<ConsistencyResult> {
    let adapter = adapter_for(profile)?;
    adapter.check_consistency(input)
}

pub fn realize(profile: ReasonerId, input: &ReasonerInput) -> Result<RealizationResult> {
    let adapter = adapter_for(profile)?;
    adapter.realize(input)
}

pub fn check_instance(
    profile: ReasonerId,
    input: &ReasonerInput,
    individual_iri: &str,
    class_iri: &str,
) -> Result<InstanceCheckResult> {
    let adapter = adapter_for(profile)?;
    adapter.check_instance(input, individual_iri, class_iri)
}

pub fn inferred_assertions(
    profile: ReasonerId,
    input: &ReasonerInput,
) -> Result<InferredAssertions> {
    let adapter = adapter_for(profile)?;
    adapter.inferred_assertions(input)
}

pub fn explain(
    profile: ReasonerId,
    input: &ReasonerInput,
    request: &ExplanationRequest,
) -> Result<ExplanationResult> {
    let adapter = adapter_for(profile)?;
    adapter.explain(input, request)
}

pub fn explain_alternatives(
    profile: ReasonerId,
    input: &ReasonerInput,
    request: &ExplanationRequest,
    max_justifications: usize,
) -> Result<Vec<ExplanationResult>> {
    crate::explain::explain_unsatisfiable_alternatives(
        profile,
        &input.ontology,
        &request.class_iri,
        max_justifications,
    )
}

fn profile_warnings(
    ontology: &ontologos_core::Ontology,
    profile: ReasonerId,
    auto_detect: bool,
) -> Result<Vec<crate::result::ReasonerWarning>> {
    if !auto_detect {
        return Ok(Vec::new());
    }
    let report = ontologos_profile::detect_profile(ontology)
        .map_err(|e| ReasonerError::Classify(e.to_string()))?;
    let mut warnings = Vec::new();
    for diag in report.diagnostics {
        warnings.push(crate::result::ReasonerWarning {
            code: "profile_construct".to_string(),
            message: diag.message,
            suggested_profile: report.detected.map(|p| format!("{p:?}").to_ascii_lowercase()),
        });
    }
    if profile == ReasonerId::El {
        let constructs = scan_constructs(ontology);
        for diag in ontologos_profile::el_diagnostics(&constructs) {
            warnings.push(crate::result::ReasonerWarning {
                code: "el_construct".to_string(),
                message: diag.message,
                suggested_profile: Some("el".to_string()),
            });
        }
    }
    Ok(warnings)
}

fn suggest_profile(ontology: &ontologos_core::Ontology) -> Option<String> {
    let report = ontologos_profile::detect_profile(ontology).ok()?;
    report.detected.map(|p| match p {
        ontologos_profile::OwlProfile::El => "el".to_string(),
        ontologos_profile::OwlProfile::Rl => "rl".to_string(),
        ontologos_profile::OwlProfile::Ql => "el".to_string(),
        ontologos_profile::OwlProfile::Dl => "dl".to_string(),
    })
}
