//! Canonical Strixonomy SWRL IR (aligns with horned-owl Atom; keeps BuiltIns).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwrlIArg {
    Individual(String),
    Variable(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwrlDArg {
    Literal { lexical: String, datatype: Option<String> },
    Variable(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SwrlAtom {
    Class { class: String, arg: SwrlIArg },
    ObjectProperty { property: String, subject: SwrlIArg, object: SwrlIArg },
    DataProperty { property: String, subject: SwrlIArg, value: SwrlDArg },
    SameIndividual { left: SwrlIArg, right: SwrlIArg },
    DifferentIndividuals { left: SwrlIArg, right: SwrlIArg },
    BuiltIn { predicate: String, args: Vec<SwrlDArg> },
    DataRange { range: String, arg: SwrlDArg },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwrlRule {
    /// Stable id when known (blank node or named rule IRI).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub body: Vec<SwrlAtom>,
    pub head: Vec<SwrlAtom>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwrlRuleSummary {
    pub id: String,
    pub label: String,
    pub body_count: usize,
    pub head_count: usize,
    pub enabled: bool,
}

impl SwrlRule {
    pub fn summary(&self, index: usize) -> SwrlRuleSummary {
        let id = self.id.clone().unwrap_or_else(|| format!("rule-{index}"));
        let label = format!(
            "{} → {}",
            self.body.len(),
            self.head.first().map(atom_short).unwrap_or_else(|| "∅".into())
        );
        SwrlRuleSummary {
            id,
            label,
            body_count: self.body.len(),
            head_count: self.head.len(),
            enabled: self.enabled,
        }
    }

    pub fn referenced_iris(&self) -> Vec<String> {
        let mut out = Vec::new();
        for atom in self.body.iter().chain(self.head.iter()) {
            collect_iris(atom, &mut out);
        }
        out.sort();
        out.dedup();
        out
    }

    /// Rewrite absolute IRIs in the rule (`from` → `to`). Returns `true` if anything changed.
    pub fn remap_iri(&mut self, from: &str, to: &str) -> bool {
        if from == to {
            return false;
        }
        let mut changed = false;
        for atom in self.body.iter_mut().chain(self.head.iter_mut()) {
            if remap_atom(atom, from, to) {
                changed = true;
            }
        }
        if let Some(id) = &mut self.id {
            if id == from {
                *id = to.to_string();
                changed = true;
            }
        }
        changed
    }
}

fn remap_atom(atom: &mut SwrlAtom, from: &str, to: &str) -> bool {
    let mut changed = false;
    let replace = |s: &mut String| {
        if s.as_str() == from {
            *s = to.to_string();
            true
        } else {
            false
        }
    };
    match atom {
        SwrlAtom::Class { class, arg } => {
            changed |= replace(class);
            if let SwrlIArg::Individual(i) = arg {
                changed |= replace(i);
            }
        }
        SwrlAtom::ObjectProperty { property, subject, object } => {
            changed |= replace(property);
            if let SwrlIArg::Individual(i) = subject {
                changed |= replace(i);
            }
            if let SwrlIArg::Individual(i) = object {
                changed |= replace(i);
            }
        }
        SwrlAtom::DataProperty { property, subject, .. } => {
            changed |= replace(property);
            if let SwrlIArg::Individual(i) = subject {
                changed |= replace(i);
            }
        }
        SwrlAtom::SameIndividual { left: a, right: b }
        | SwrlAtom::DifferentIndividuals { left: a, right: b } => {
            if let SwrlIArg::Individual(i) = a {
                changed |= replace(i);
            }
            if let SwrlIArg::Individual(i) = b {
                changed |= replace(i);
            }
        }
        SwrlAtom::BuiltIn { predicate, .. } => {
            changed |= replace(predicate);
        }
        SwrlAtom::DataRange { range, .. } => {
            changed |= replace(range);
        }
    }
    changed
}

fn atom_short(atom: &SwrlAtom) -> String {
    match atom {
        SwrlAtom::Class { class, .. } => short(class),
        SwrlAtom::ObjectProperty { property, .. } => short(property),
        SwrlAtom::DataProperty { property, .. } => short(property),
        SwrlAtom::BuiltIn { predicate, .. } => short(predicate),
        SwrlAtom::SameIndividual { .. } => "sameAs".into(),
        SwrlAtom::DifferentIndividuals { .. } => "differentFrom".into(),
        SwrlAtom::DataRange { range, .. } => short(range),
    }
}

fn short(iri: &str) -> String {
    iri.rsplit(['#', '/']).next().unwrap_or(iri).to_string()
}

fn collect_iris(atom: &SwrlAtom, out: &mut Vec<String>) {
    match atom {
        SwrlAtom::Class { class, arg } => {
            out.push(class.clone());
            if let SwrlIArg::Individual(i) = arg {
                out.push(i.clone());
            }
        }
        SwrlAtom::ObjectProperty { property, subject, object } => {
            out.push(property.clone());
            if let SwrlIArg::Individual(i) = subject {
                out.push(i.clone());
            }
            if let SwrlIArg::Individual(i) = object {
                out.push(i.clone());
            }
        }
        SwrlAtom::DataProperty { property, subject, .. } => {
            out.push(property.clone());
            if let SwrlIArg::Individual(i) = subject {
                out.push(i.clone());
            }
        }
        SwrlAtom::SameIndividual { left: a, right: b }
        | SwrlAtom::DifferentIndividuals { left: a, right: b } => {
            if let SwrlIArg::Individual(i) = a {
                out.push(i.clone());
            }
            if let SwrlIArg::Individual(i) = b {
                out.push(i.clone());
            }
        }
        SwrlAtom::BuiltIn { predicate, .. } => out.push(predicate.clone()),
        SwrlAtom::DataRange { range, .. } => out.push(range.clone()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwrlSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwrlDiagnostic {
    pub code: String,
    pub severity: SwrlSeverity,
    pub message: String,
}
