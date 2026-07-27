//! Protégé Wave 2 port: annotation link extractors.
//! Upstream: DOI/PubMed/ORCID/OMIM/OMIMPS/Orphanet/ISBN/Wikipedia*LinkExtractor,
//! RegExBasedLinkExtractor.

use regex::Regex;
use strixonomy_owl::{
    extract_first_link_url, extract_links, extract_with_pattern, linkify_markdown_text,
};

#[test]
fn links_doi() {
    let links = extract_links("cite DOI:10.1000/182");
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].kind, "DOI");
    assert_eq!(links[0].url, "https://doi.org/10.1000/182");
}

#[test]
fn links_pmid() {
    assert_eq!(
        extract_first_link_url("PMID: 12345678").as_deref(),
        Some("http://www.ncbi.nlm.nih.gov/pubmed/12345678")
    );
}

#[test]
fn links_orcid() {
    let links = extract_links("ORCID: 0000-0002-1825-0097");
    assert_eq!(links[0].url, "https://orcid.org/0000-0002-1825-0097");
}

#[test]
fn links_omim_and_omimps() {
    let omim = extract_links("OMIM: 100100");
    assert_eq!(omim[0].kind, "OMIM");
    assert_eq!(omim[0].url, "https://omim.org/entry/100100");

    let series = extract_links("OMIMPS: 123");
    assert_eq!(series[0].kind, "OMIMPS");
    assert!(series[0].url.contains("phenotypicSeries/123"));
}

#[test]
fn links_orphanet_isbn_wikipedia() {
    assert!(extract_links("Orphanet: 558")[0].url.contains("Expert=558"));
    assert_eq!(
        extract_links("ISBN:0123456789")[0].url,
        "http://www.isbnsearch.org/isbn/0123456789"
    );
    assert_eq!(extract_links("Wikipedia:Ontology")[0].url, "https://wikipedia.org/wiki/Ontology");
    assert!(extract_links("WikipediaVersioned:Ontology")[0].url.contains("title=Ontology"));
}

#[test]
fn links_multi_non_overlapping() {
    let links = extract_links("See PMID: 1 and DOI: 10.1/x");
    assert_eq!(links.len(), 2);
    assert_eq!(links[0].kind, "PubMedId");
    assert_eq!(links[1].kind, "DOI");
}

#[test]
fn links_custom_regex_extractor() {
    let re = Regex::new(r"(?i)TICKET:\s*(\d+)").unwrap();
    let link =
        extract_with_pattern("Ticket", &re, "https://issues.example/$1", "TICKET: 42").unwrap();
    assert_eq!(link.url, "https://issues.example/42");
}

#[test]
fn links_linkify_markdown() {
    let md = linkify_markdown_text("PMID: 9 rest");
    assert!(md.contains("[PMID: 9](http://www.ncbi.nlm.nih.gov/pubmed/9)"));
    assert!(md.contains("rest"));
}
