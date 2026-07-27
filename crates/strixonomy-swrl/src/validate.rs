//! SWRL rule validation (variables, DLSafe heuristics, builtins).

use crate::builtins::is_supported_builtin;
use crate::model::{SwrlAtom, SwrlDArg, SwrlDiagnostic, SwrlIArg, SwrlRule, SwrlSeverity};
use std::collections::BTreeSet;

const MAX_ATOMS: usize = 256;
const MAX_IRI_LEN: usize = 2048;

pub fn validate_rule(rule: &SwrlRule) -> Vec<SwrlDiagnostic> {
    let mut diags = Vec::new();
    if rule.body.is_empty() {
        diags.push(SwrlDiagnostic {
            code: "swrl_empty_body".into(),
            severity: SwrlSeverity::Error,
            message: "SWRL rule body must not be empty".into(),
        });
    }
    if rule.head.is_empty() {
        diags.push(SwrlDiagnostic {
            code: "swrl_empty_head".into(),
            severity: SwrlSeverity::Error,
            message: "SWRL rule head must not be empty".into(),
        });
    }
    let total_atoms = rule.body.len() + rule.head.len();
    if total_atoms > MAX_ATOMS {
        diags.push(SwrlDiagnostic {
            code: "swrl_rule_too_large".into(),
            severity: SwrlSeverity::Error,
            message: format!("SWRL rule has {total_atoms} atoms; max is {MAX_ATOMS}"),
        });
    }

    let mut body_vars = BTreeSet::new();
    for atom in &rule.body {
        collect_vars(atom, &mut body_vars);
        check_atom(atom, &mut diags, false);
    }
    for atom in &rule.head {
        check_atom(atom, &mut diags, true);
        for v in atom_vars(atom) {
            if !body_vars.contains(&v) {
                diags.push(SwrlDiagnostic {
                    code: "swrl_unbound_head_var".into(),
                    severity: SwrlSeverity::Error,
                    message: format!("variable ?{v} appears in head but is not bound in body"),
                });
            }
        }
    }

    for atom in &rule.body {
        warn_non_executable_atom(atom, false, &mut diags);
    }
    for atom in &rule.head {
        warn_non_executable_atom(atom, true, &mut diags);
        if matches!(atom, SwrlAtom::BuiltIn { .. }) {
            diags.push(SwrlDiagnostic {
                code: "swrl_builtin_in_head".into(),
                severity: SwrlSeverity::Warning,
                message: "Built-in atoms in rule heads are not executed by Strixonomy/Ontologos"
                    .into(),
            });
        }
    }

    diags
}

fn warn_non_executable_atom(atom: &SwrlAtom, in_head: bool, diags: &mut Vec<SwrlDiagnostic>) {
    let where_ = if in_head { "head" } else { "body" };
    match atom {
        SwrlAtom::BuiltIn { predicate, .. } => {
            diags.push(SwrlDiagnostic {
                code: "swrl_builtin_not_executable".into(),
                severity: SwrlSeverity::Warning,
                message: format!(
                    "BuiltIn atom ({predicate}) in rule {where_} is not injected into Ontologos materialization; the rule will be skipped at classify"
                ),
            });
        }
        SwrlAtom::DataRange { range, .. } => {
            diags.push(SwrlDiagnostic {
                code: "swrl_datarange_not_executable".into(),
                severity: SwrlSeverity::Warning,
                message: format!(
                    "DataRange atom ({range}) in rule {where_} is not injected into Ontologos materialization; the rule will be skipped at classify"
                ),
            });
        }
        _ => {}
    }
}

fn check_atom(atom: &SwrlAtom, diags: &mut Vec<SwrlDiagnostic>, _in_head: bool) {
    for iri in atom_iris(atom) {
        if iri.trim().is_empty() {
            diags.push(SwrlDiagnostic {
                code: "swrl_empty_iri".into(),
                severity: SwrlSeverity::Error,
                message: "SWRL atom contains an empty IRI".into(),
            });
        } else if iri.len() > MAX_IRI_LEN {
            diags.push(SwrlDiagnostic {
                code: "swrl_iri_too_long".into(),
                severity: SwrlSeverity::Error,
                message: format!("SWRL IRI exceeds {MAX_IRI_LEN} characters"),
            });
        }
    }
    if let SwrlAtom::BuiltIn { predicate, .. } = atom {
        if !is_supported_builtin(predicate) {
            diags.push(SwrlDiagnostic {
                code: "swrl_unsupported_builtin".into(),
                severity: SwrlSeverity::Warning,
                message: format!(
                    "built-in {predicate} is not in the Strixonomy supported swrlb: registry"
                ),
            });
        }
    }
}

fn atom_iris(atom: &SwrlAtom) -> Vec<&str> {
    match atom {
        SwrlAtom::Class { class, arg } => {
            let mut v = vec![class.as_str()];
            if let SwrlIArg::Individual(i) = arg {
                v.push(i.as_str());
            }
            v
        }
        SwrlAtom::ObjectProperty { property, subject, object } => {
            let mut v = vec![property.as_str()];
            if let SwrlIArg::Individual(i) = subject {
                v.push(i.as_str());
            }
            if let SwrlIArg::Individual(i) = object {
                v.push(i.as_str());
            }
            v
        }
        SwrlAtom::DataProperty { property, subject, value } => {
            let mut v = vec![property.as_str()];
            if let SwrlIArg::Individual(i) = subject {
                v.push(i.as_str());
            }
            if let SwrlDArg::Literal { datatype: Some(dt), .. } = value {
                v.push(dt.as_str());
            }
            v
        }
        SwrlAtom::BuiltIn { predicate, .. } => vec![predicate.as_str()],
        SwrlAtom::DataRange { range, .. } => vec![range.as_str()],
        SwrlAtom::SameIndividual { left, right }
        | SwrlAtom::DifferentIndividuals { left, right } => {
            let mut v = Vec::new();
            if let SwrlIArg::Individual(i) = left {
                v.push(i.as_str());
            }
            if let SwrlIArg::Individual(i) = right {
                v.push(i.as_str());
            }
            v
        }
    }
}

fn collect_vars(atom: &SwrlAtom, vars: &mut BTreeSet<String>) {
    for v in atom_vars(atom) {
        vars.insert(v);
    }
}

fn atom_vars(atom: &SwrlAtom) -> Vec<String> {
    let mut out = Vec::new();
    match atom {
        SwrlAtom::Class { arg, .. } => push_i(arg, &mut out),
        SwrlAtom::ObjectProperty { subject, object, .. } => {
            push_i(subject, &mut out);
            push_i(object, &mut out);
        }
        SwrlAtom::DataProperty { subject, value, .. } => {
            push_i(subject, &mut out);
            push_d(value, &mut out);
        }
        SwrlAtom::SameIndividual { left: a, right: b }
        | SwrlAtom::DifferentIndividuals { left: a, right: b } => {
            push_i(a, &mut out);
            push_i(b, &mut out);
        }
        SwrlAtom::BuiltIn { args, .. } => {
            for a in args {
                push_d(a, &mut out);
            }
        }
        SwrlAtom::DataRange { arg, .. } => push_d(arg, &mut out),
    }
    out
}

fn push_i(arg: &SwrlIArg, out: &mut Vec<String>) {
    if let SwrlIArg::Variable(v) = arg {
        out.push(v.clone());
    }
}

fn push_d(arg: &SwrlDArg, out: &mut Vec<String>) {
    if let SwrlDArg::Variable(v) = arg {
        out.push(v.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{SwrlAtom, SwrlIArg, SwrlRule};

    #[test]
    fn rejects_unbound_head_var() {
        let rule = SwrlRule {
            id: None,
            body: vec![SwrlAtom::Class {
                class: "http://ex#A".into(),
                arg: SwrlIArg::Variable("x".into()),
            }],
            head: vec![SwrlAtom::Class {
                class: "http://ex#B".into(),
                arg: SwrlIArg::Variable("y".into()),
            }],
            enabled: true,
        };
        let d = validate_rule(&rule);
        assert!(d.iter().any(|x| x.code == "swrl_unbound_head_var"));
    }

    #[test]
    fn rejects_empty_iris() {
        let rule = SwrlRule {
            id: None,
            body: vec![SwrlAtom::Class { class: "".into(), arg: SwrlIArg::Variable("x".into()) }],
            head: vec![SwrlAtom::Class {
                class: "http://ex#B".into(),
                arg: SwrlIArg::Variable("x".into()),
            }],
            enabled: true,
        };
        let d = validate_rule(&rule);
        assert!(d.iter().any(|x| x.code == "swrl_empty_iri"));
    }

    #[test]
    fn warns_builtin_and_datarange_not_executable() {
        let rule = SwrlRule {
            id: None,
            body: vec![
                SwrlAtom::Class {
                    class: "http://ex#A".into(),
                    arg: SwrlIArg::Variable("x".into()),
                },
                SwrlAtom::BuiltIn {
                    predicate: "http://www.w3.org/2003/11/swrlb#equal".into(),
                    args: vec![],
                },
                SwrlAtom::DataRange {
                    range: "http://www.w3.org/2001/XMLSchema#string".into(),
                    arg: crate::model::SwrlDArg::Variable("x".into()),
                },
            ],
            head: vec![SwrlAtom::Class {
                class: "http://ex#B".into(),
                arg: SwrlIArg::Variable("x".into()),
            }],
            enabled: true,
        };
        let d = validate_rule(&rule);
        assert!(d.iter().any(|x| x.code == "swrl_builtin_not_executable"));
        assert!(d.iter().any(|x| x.code == "swrl_datarange_not_executable"));
    }

    #[test]
    fn foreign_namespace_builtin_is_unsupported() {
        let rule = SwrlRule {
            id: None,
            body: vec![SwrlAtom::BuiltIn {
                predicate: "http://evil.example/equal".into(),
                args: vec![],
            }],
            head: vec![SwrlAtom::Class {
                class: "http://ex#B".into(),
                arg: SwrlIArg::Variable("x".into()),
            }],
            enabled: true,
        };
        let d = validate_rule(&rule);
        assert!(d.iter().any(|x| x.code == "swrl_unsupported_builtin"));
        assert!(d.iter().any(|x| x.code == "swrl_builtin_not_executable"));
    }
}
