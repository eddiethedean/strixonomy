use crate::adapter::{ApplyTextResult, EditFormat};
use crate::change::SemanticChange;
use crate::error::{EditError, Result};
use crate::invert::invert_change;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strixonomy_obo::OboPatchOp;
use strixonomy_owl::PatchOp;

/// Ordered semantic edits applied as one atomic unit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Transaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub changes: Vec<SemanticChange>,
}

impl Transaction {
    pub fn new(changes: Vec<SemanticChange>) -> Self {
        Self { id: None, label: None, changes }
    }

    pub fn from_turtle(changes: Vec<PatchOp>) -> Self {
        Self::new(changes.into_iter().map(SemanticChange::turtle).collect())
    }

    pub fn from_obo(changes: Vec<OboPatchOp>) -> Self {
        Self::new(changes.into_iter().map(SemanticChange::obo).collect())
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    pub fn format(&self) -> Result<EditFormat> {
        if self.changes.is_empty() {
            return Err(EditError::Empty);
        }
        let mut turtle = false;
        let mut obo = false;
        for change in &self.changes {
            if change.is_turtle() {
                turtle = true;
            }
            if change.is_obo() {
                obo = true;
            }
        }
        if turtle && obo {
            return Err(EditError::MixedFormats);
        }
        if turtle {
            Ok(EditFormat::Turtle)
        } else if obo {
            Ok(EditFormat::Obo)
        } else {
            Err(EditError::Empty)
        }
    }

    pub fn compose(mut self, other: Transaction) -> Result<Self> {
        if !self.changes.is_empty() && !other.changes.is_empty() {
            let left = self.format()?;
            let right = other.format()?;
            if left != right {
                return Err(EditError::MixedFormats);
            }
        }
        self.changes.extend(other.changes);
        Ok(self)
    }

    pub fn invert(&self) -> Result<Self> {
        if self.is_empty() {
            return Err(EditError::Empty);
        }
        let mut inverted = Vec::with_capacity(self.changes.len());
        for change in self.changes.iter().rev() {
            inverted.push(invert_change(change)?);
        }
        Ok(Self { id: self.id.clone(), label: self.label.clone(), changes: inverted })
    }

    pub fn validate(&self) -> Result<()> {
        if self.is_empty() {
            return Err(EditError::Validation("transaction has no changes".into()));
        }
        let _ = self.format()?;
        Ok(())
    }

    pub fn apply_to_text(
        &self,
        source: &str,
        preview_only: bool,
        namespaces: &BTreeMap<String, String>,
    ) -> Result<ApplyTextResult> {
        self.validate()?;
        crate::adapter::apply_transaction_to_text(self, source, preview_only, namespaces)
    }

    /// Apply with an explicit document format (RDF/XML / OWL/XML use Horned re-serialize).
    pub fn apply_to_text_as(
        &self,
        source: &str,
        preview_only: bool,
        namespaces: &BTreeMap<String, String>,
        format: EditFormat,
    ) -> Result<ApplyTextResult> {
        self.validate()?;
        crate::adapter::apply_transaction_to_text_as(self, source, preview_only, namespaces, format)
    }

    pub fn turtle_patches(&self) -> Result<Vec<PatchOp>> {
        self.changes
            .iter()
            .map(|c| match c {
                SemanticChange::Turtle { change } => Ok(change.clone()),
                SemanticChange::Obo { .. } => Err(EditError::MixedFormats),
            })
            .collect()
    }

    pub fn obo_patches(&self) -> Result<Vec<OboPatchOp>> {
        self.changes
            .iter()
            .map(|c| match c {
                SemanticChange::Obo { change } => Ok(change.clone()),
                SemanticChange::Turtle { .. } => Err(EditError::MixedFormats),
            })
            .collect()
    }
}

/// Parse legacy patch JSON (array) or `{ "transaction": { ... } }` envelope.
///
/// Envelope `changes` may be either raw Turtle ops (documented wire format) or
/// tagged [`SemanticChange`] values. Rejects envelopes whose changes are not Turtle.
pub fn parse_turtle_input(value: serde_json::Value) -> Result<Transaction> {
    if let Some(txn) = value.get("transaction") {
        if let Some(changes) = txn.get("changes") {
            if let Ok(patches) = serde_json::from_value::<Vec<PatchOp>>(changes.clone()) {
                let mut parsed = Transaction::from_turtle(patches);
                parsed.id = txn.get("id").and_then(|v| v.as_str()).map(str::to_string);
                parsed.label = txn.get("label").and_then(|v| v.as_str()).map(str::to_string);
                return Ok(parsed);
            }
        }
        let parsed: Transaction = serde_json::from_value(txn.clone())?;
        match parsed.format()? {
            EditFormat::Turtle => Ok(parsed),
            EditFormat::Obo => Err(EditError::Validation(
                "transaction contains OBO changes; refuse Turtle apply path".into(),
            )),
            EditFormat::RdfXml | EditFormat::OwlXml => Err(EditError::Validation(
                "unexpected XML edit format on ParseOps from tagged SemanticChange".into(),
            )),
        }
    } else {
        let patches: Vec<PatchOp> = serde_json::from_value(value)?;
        Ok(Transaction::from_turtle(patches))
    }
}

/// Parse legacy OBO patch JSON (array) or `{ "transaction": { ... } }` envelope.
pub fn parse_obo_input(value: serde_json::Value) -> Result<Transaction> {
    if let Some(txn) = value.get("transaction") {
        if let Some(changes) = txn.get("changes") {
            if let Ok(patches) = serde_json::from_value::<Vec<OboPatchOp>>(changes.clone()) {
                let mut parsed = Transaction::from_obo(patches);
                parsed.id = txn.get("id").and_then(|v| v.as_str()).map(str::to_string);
                parsed.label = txn.get("label").and_then(|v| v.as_str()).map(str::to_string);
                return Ok(parsed);
            }
        }
        let parsed: Transaction = serde_json::from_value(txn.clone())?;
        match parsed.format()? {
            EditFormat::Obo => Ok(parsed),
            EditFormat::Turtle | EditFormat::RdfXml | EditFormat::OwlXml => {
                Err(EditError::Validation(
                    "transaction contains Turtle changes; refuse OBO apply path".into(),
                ))
            }
        }
    } else {
        let patches: Vec<OboPatchOp> = serde_json::from_value(value)?;
        Ok(Transaction::from_obo(patches))
    }
}
