import type { Entity, OntologyDocument } from "../lsp/protocol";

export function normalizeOntologyIri(iri: string): string {
  return iri.replace(/[#/]+$/, "");
}

/** Mirror `document_matches_entity` in strixonomy-core (no IRI prefix matching). */
export function entityBelongsToDocument(
  entity: Entity,
  doc: OntologyDocument
): boolean {
  if (entity.ontology_id === doc.id) {
    return true;
  }
  if (doc.base_iri) {
    if (normalizeOntologyIri(doc.base_iri) === normalizeOntologyIri(entity.ontology_id)) {
      return true;
    }
  }
  return false;
}

/**
 * Resolve the owl:Ontology subject IRI declared in a Turtle document.
 * Prefers the ontology matching `doc.base_iri` when multiple are present (#118).
 */
export function resolveOntologyIri(
  doc: OntologyDocument,
  entities: Entity[]
): string | undefined {
  const candidates = entities.filter(
    (e) => e.kind === "ontology" && entityBelongsToDocument(e, doc)
  );
  if (candidates.length === 0) {
    return undefined;
  }
  if (candidates.length === 1) {
    return candidates[0]?.iri;
  }
  if (doc.base_iri) {
    const normalizedBase = normalizeOntologyIri(doc.base_iri);
    const baseMatch = candidates.find(
      (e) => normalizeOntologyIri(e.iri) === normalizedBase
    );
    if (baseMatch) {
      return baseMatch.iri;
    }
  }
  // Ambiguous multi-ontology file without a clear base match — refuse to guess.
  return undefined;
}

export const MISSING_ONTOLOGY_HEADER_MESSAGE =
  "This file has no owl:Ontology declaration. Add an ontology header before managing imports.";

export const AMBIGUOUS_ONTOLOGY_HEADER_MESSAGE =
  "This file declares multiple owl:Ontology subjects and none match the document base IRI. Manage imports requires a single primary ontology.";
