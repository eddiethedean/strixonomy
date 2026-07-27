use crate::adapter::ReasonerId;
use crate::input::ReasonerInput;
use crate::result::{ClassificationResult, ReasonerSnapshot};
use std::collections::HashMap;

/// Keep at most one classification entry per profile (plus a small global cap).
const MAX_CACHE_ENTRIES: usize = 8;

#[derive(Debug, Clone)]
pub struct ReasonerCache {
    pub content_hash: String,
    pub profile: ReasonerId,
    pub snapshot: ReasonerSnapshot,
    pub classification: ClassificationResult,
    pub input: ReasonerInput,
}

#[derive(Debug, Default)]
pub struct ReasonerCacheStore {
    entries: HashMap<(String, String), ReasonerCache>,
    /// Insertion order for eviction (oldest first).
    order: Vec<(String, String)>,
}

impl ReasonerCacheStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, content_hash: &str, profile: ReasonerId) -> Option<&ReasonerCache> {
        self.entries.get(&(content_hash.to_string(), profile.as_str().to_string()))
    }

    pub fn get_any_for_profile(&self, profile: ReasonerId) -> Option<&ReasonerCache> {
        self.entries.values().find(|e| e.profile == profile)
    }

    pub fn insert(&mut self, cache: ReasonerCache) {
        let profile_key = cache.profile.as_str().to_string();
        let key = (cache.content_hash.clone(), profile_key.clone());

        // One entry per profile: drop any prior entry for the same profile.
        let stale: Vec<_> =
            self.entries.keys().filter(|(_, p)| p == &profile_key).cloned().collect();
        for stale_key in stale {
            self.entries.remove(&stale_key);
            self.order.retain(|k| k != &stale_key);
        }

        if self.entries.insert(key.clone(), cache).is_none() {
            self.order.push(key);
        } else if let Some(pos) = self.order.iter().position(|k| k == &key) {
            let k = self.order.remove(pos);
            self.order.push(k);
        }

        while self.entries.len() > MAX_CACHE_ENTRIES {
            if let Some(oldest) = self.order.first().cloned() {
                self.entries.remove(&oldest);
                self.order.remove(0);
            } else {
                break;
            }
        }
    }

    pub fn invalidate(&mut self) {
        self.entries.clear();
        self.order.clear();
    }

    pub fn latest_snapshot(&self) -> Option<&ReasonerSnapshot> {
        self.entries.values().max_by_key(|e| e.snapshot.classified_at).map(|e| &e.snapshot)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn store_classification(
        &mut self,
        input: ReasonerInput,
        profile: ReasonerId,
        classification: ClassificationResult,
    ) -> ReasonerSnapshot {
        let snapshot = ReasonerSnapshot::from(classification.clone());
        let cache = ReasonerCache {
            content_hash: input.content_hash.clone(),
            profile,
            snapshot: snapshot.clone(),
            classification,
            input,
        };
        self.insert(cache);
        snapshot
    }

    /// Persist an enriched snapshot (realization / consistency) after ABox enrich (#355).
    pub fn update_snapshot(
        &mut self,
        content_hash: &str,
        profile: ReasonerId,
        snapshot: ReasonerSnapshot,
    ) -> bool {
        let key = (content_hash.to_string(), profile.as_str().to_string());
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.classification.consistent = snapshot.consistent;
            entry.snapshot = snapshot;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::ReasonerInput;
    use crate::result::{ClassificationResult, InferredHierarchy};
    use ontologos_core::Ontology;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use strixonomy_catalog::ClassHierarchy;

    fn dummy_input(hash: &str) -> ReasonerInput {
        ReasonerInput {
            workspace: PathBuf::from("/tmp"),
            content_hash: hash.to_string(),
            ontology: Ontology::new(),
            asserted_hierarchy: ClassHierarchy::default(),
            document_overrides: HashMap::new(),
        }
    }

    fn dummy_classification(profile: &str) -> ClassificationResult {
        ClassificationResult {
            profile_used: profile.to_string(),
            consistent: true,
            unsatisfiable: Vec::new(),
            inferred: InferredHierarchy {
                edges: Vec::new(),
                unsatisfiable: Vec::new(),
                combined: ClassHierarchy::default(),
            },
            new_inferences: Vec::new(),
            warnings: Vec::new(),
            duration_ms: 0,
            subsumption_count: 0,
            inferred_axiom_count: 0,
            equivalences: Vec::new(),
        }
    }

    #[test]
    fn one_entry_per_profile() {
        let mut store = ReasonerCacheStore::new();
        store.store_classification(dummy_input("a"), ReasonerId::El, dummy_classification("el"));
        store.store_classification(dummy_input("b"), ReasonerId::El, dummy_classification("el"));
        assert_eq!(store.len(), 1);
        assert!(store.get("b", ReasonerId::El).is_some());
        assert!(store.get("a", ReasonerId::El).is_none());
    }

    #[test]
    fn update_snapshot_preserves_realization() {
        let mut store = ReasonerCacheStore::new();
        let mut snapshot = store.store_classification(
            dummy_input("h"),
            ReasonerId::Dl,
            dummy_classification("dl"),
        );
        assert!(snapshot.realization.is_none());
        snapshot.realization = Some(crate::result::RealizationResult {
            profile_used: "dl".into(),
            individuals: Vec::new(),
            duration_ms: 1,
            truncated: false,
            entailment_errors: 0,
        });
        assert!(store.update_snapshot("h", ReasonerId::Dl, snapshot.clone()));
        let cached = store.get("h", ReasonerId::Dl).expect("cached");
        assert!(cached.snapshot.realization.is_some());
    }
}
