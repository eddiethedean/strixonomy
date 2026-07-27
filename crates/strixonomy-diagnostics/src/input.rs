use strixonomy_core::{Annotation, Axiom, Diagnostic, Entity, Import, Namespace, OntologyDocument};

/// Read-only catalog snapshot for lint rules (avoids cyclic dependency on `strixonomy-catalog`).
#[derive(Debug, Clone, Default)]
pub struct DiagnosticInput<'a> {
    pub documents: &'a [OntologyDocument],
    pub entities: &'a [Entity],
    pub annotations: &'a [Annotation],
    pub axioms: &'a [Axiom],
    pub namespaces: &'a [Namespace],
    pub imports: &'a [Import],
}

impl<'a> DiagnosticInput<'a> {
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        crate::engine::collect_diagnostics(self)
    }
}
