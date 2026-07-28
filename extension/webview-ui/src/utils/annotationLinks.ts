/**
 * Annotation hyperlink extractors (Protégé *LinkExtractor ports).
 * Keep pattern tables in sync with crates/strixonomy-owl/src/links.rs.
 */

export type AnnotationLink = {
  kind: string
  matchedText: string
  url: string
  start: number
  end: number
}

type Extractor = {
  name: string
  pattern: RegExp
  urlTemplate: string
}

const EXTRACTORS: Extractor[] = [
  {
    name: "DOI",
    pattern: /DOI:\s*([^\s]+)/gi,
    urlTemplate: "https://doi.org/$1",
  },
  {
    name: "PubMedId",
    pattern: /PMID:\s*(\d+)/gi,
    urlTemplate: "http://www.ncbi.nlm.nih.gov/pubmed/$1",
  },
  {
    name: "ORCID",
    pattern: /ORCID:\s*([^\s]+)/gi,
    urlTemplate: "https://orcid.org/$1",
  },
  {
    name: "OMIMPS",
    pattern: /OMIMPS:\s*(\d+)/gi,
    urlTemplate: "https://www.omim.org/phenotypicSeries/$1",
  },
  {
    name: "OMIM",
    pattern: /OMIM:\s*(\d+)/gi,
    urlTemplate: "https://omim.org/entry/$1",
  },
  {
    name: "Orphanet",
    pattern: /Orphanet:\s*(\d+)/gi,
    urlTemplate: "https://www.orpha.net/consor/cgi-bin/OC_Exp.php?Expert=$1",
  },
  {
    name: "ISBN-10",
    pattern: /ISBN:(\d{10})/gi,
    urlTemplate: "http://www.isbnsearch.org/isbn/$1",
  },
  {
    name: "WikipediaVersioned",
    pattern: /WikipediaVersioned:([^\s]+)/gi,
    urlTemplate: "https://wikipedia.org/wiki/index.php?title=$1",
  },
  {
    name: "Wikipedia",
    pattern: /Wikipedia:([^\s]+)/gi,
    urlTemplate: "https://wikipedia.org/wiki/$1",
  },
]

function applyTemplate(template: string, capture: string): string {
  return template.replace("$1", capture)
}

/** Extract non-overlapping annotation hyperlinks (left-to-right). */
export function extractLinks(text: string): AnnotationLink[] {
  const found: AnnotationLink[] = []
  for (const ex of EXTRACTORS) {
    const re = new RegExp(ex.pattern.source, ex.pattern.flags)
    let m: RegExpExecArray | null
    while ((m = re.exec(text)) !== null) {
      found.push({
        kind: ex.name,
        matchedText: m[0],
        url: applyTemplate(ex.urlTemplate, m[1] ?? ""),
        start: m.index,
        end: m.index + m[0].length,
      })
    }
  }
  found.sort((a, b) => a.start - b.start || a.end - b.end)
  const out: AnnotationLink[] = []
  let cursor = 0
  for (const link of found) {
    if (link.start < cursor) continue
    cursor = link.end
    out.push(link)
  }
  return out
}

export type AnnotationTextPart =
  | { type: "text"; value: string }
  | { type: "link"; value: string; url: string; kind: string }

/** Split annotation text into plain text and link parts for React rendering. */
export function annotationTextParts(text: string): AnnotationTextPart[] {
  const links = extractLinks(text)
  if (links.length === 0) {
    return [{ type: "text", value: text }]
  }
  const parts: AnnotationTextPart[] = []
  let idx = 0
  for (const link of links) {
    if (link.start > idx) {
      parts.push({ type: "text", value: text.slice(idx, link.start) })
    }
    parts.push({
      type: "link",
      value: link.matchedText,
      url: link.url,
      kind: link.kind,
    })
    idx = link.end
  }
  if (idx < text.length) {
    parts.push({ type: "text", value: text.slice(idx) })
  }
  return parts
}
