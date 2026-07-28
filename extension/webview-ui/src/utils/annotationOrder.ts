/**
 * Default annotation-property IRI ordering (Protégé AnnotationPropertyComparator).
 * Keep in sync with crates/strixonomy-owl/src/util.rs DEFAULT_ANNOTATION_PROPERTY_ORDER.
 */

export const DEFAULT_ANNOTATION_PROPERTY_ORDER: readonly string[] = [
  "http://www.w3.org/2000/01/rdf-schema#label",
  "http://www.w3.org/2004/02/skos/core#prefLabel",
  "http://purl.org/dc/elements/1.1/title",
  "http://www.geneontology.org/formats/oboInOwl#id",
  "http://www.geneontology.org/formats/oboInOwl#hasAlternativeId",
  "http://www.geneontology.org/formats/oboInOwl#hasOBONamespace",
  "http://purl.obolibrary.org/obo/IAO_0000115",
  "http://www.w3.org/2004/02/skos/core#definition",
  "http://www.w3.org/2004/02/skos/core#note",
  "http://purl.org/dc/elements/1.1/description",
  "http://purl.org/dc/elements/1.1/rights",
  "http://purl.org/dc/terms/license",
  "http://purl.org/dc/elements/1.1/publisher",
  "http://purl.org/dc/elements/1.1/creator",
  "http://purl.org/dc/elements/1.1/contributor",
  "http://www.w3.org/2000/01/rdf-schema#comment",
  "http://www.w3.org/2004/02/skos/core#altLabel",
  "http://www.w3.org/2000/01/rdf-schema#seeAlso",
  "http://www.w3.org/2000/01/rdf-schema#isDefinedBy",
  "http://www.geneontology.org/formats/oboInOwl#hasExactSynonym",
  "http://www.geneontology.org/formats/oboInOwl#hasRelatedSynonym",
  "http://www.geneontology.org/formats/oboInOwl#hasBroadSynonym",
  "http://www.geneontology.org/formats/oboInOwl#hasNarrowSynonym",
]

function orderIndex(iri: string): number | null {
  const i = DEFAULT_ANNOTATION_PROPERTY_ORDER.indexOf(iri)
  return i >= 0 ? i : null
}

export function cmpAnnotationPropertyIri(a: string, b: string): number {
  const ia = orderIndex(a)
  const ib = orderIndex(b)
  if (ia !== null && ib !== null) {
    if (ia !== ib) return ia - ib
    return a < b ? -1 : a > b ? 1 : 0
  }
  if (ia !== null) return -1
  if (ib !== null) return 1
  return a < b ? -1 : a > b ? 1 : 0
}

export function sortAnnotationsByPredicate<T extends { predicate: string }>(
  annotations: T[],
): T[] {
  return [...annotations].sort((x, y) =>
    cmpAnnotationPropertyIri(x.predicate, y.predicate),
  )
}
