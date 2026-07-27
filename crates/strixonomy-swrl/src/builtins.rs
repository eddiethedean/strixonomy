//! Core SWRL built-ins registry (`swrlb:`).

pub const SWRLB_NS: &str = "http://www.w3.org/2003/11/swrlb#";

/// Built-ins Strixonomy validates and documents for v0.23 (execution may still be partial).
pub const SUPPORTED_BUILTINS: &[&str] = &[
    "equal",
    "notEqual",
    "lessThan",
    "lessThanOrEqual",
    "greaterThan",
    "greaterThanOrEqual",
    "add",
    "subtract",
    "multiply",
    "divide",
    "stringEqualIgnoreCase",
    "stringConcat",
    "substring",
    "stringLength",
    "contains",
    "startsWith",
    "endsWith",
    "matches",
    "booleanNot",
];

pub fn is_supported_builtin(predicate_iri: &str) -> bool {
    // Require the SWRLB namespace (#363). Foreign IRIs that only share a local name
    // (e.g. `http://evil.example/equal`) must not validate as supported.
    let Some(local) = predicate_iri.strip_prefix(SWRLB_NS) else {
        return false;
    };
    SUPPORTED_BUILTINS.contains(&local)
}

#[allow(dead_code)]
pub fn builtin_iri(local: &str) -> String {
    format!("{SWRLB_NS}{local}")
}
