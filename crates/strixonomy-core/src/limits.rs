//! Resource limits for indexing and querying (DoS hardening).

/// Maximum size of a single ontology file read from disk or held in an LSP open buffer.
pub const MAX_FILE_BYTES: u64 = 50 * 1024 * 1024;

/// Maximum open LSP document buffers tracked per workspace.
pub const MAX_OPEN_DOCUMENTS: usize = 256;

/// Maximum ontology files scanned per workspace.
pub const MAX_SCAN_FILES: usize = 10_000;

/// Maximum filesystem entries visited during a workspace scan (including non-ontology files).
pub const MAX_SCAN_WALK_ENTRIES: usize = 500_000;

/// Maximum RDF quads parsed from one file.
pub const MAX_TRIPLES_PER_FILE: usize = 5_000_000;

/// Maximum RDF quads loaded into the catalog store for a workspace.
pub const MAX_TOTAL_TRIPLES: usize = 20_000_000;

/// Maximum extracted entities per workspace build.
pub const MAX_ENTITIES: usize = 1_000_000;

/// Maximum SQL or SPARQL query string length.
pub const MAX_QUERY_BYTES: usize = 1_048_576;

/// Maximum rows returned from a SQL virtual-table query.
pub const MAX_SQL_RESULT_ROWS: usize = 100_000;

/// Maximum rows returned from a SPARQL query.
pub const MAX_SPARQL_RESULT_ROWS: usize = 100_000;

/// Maximum nodes in a graph export payload.
pub const MAX_GRAPH_NODES: usize = 2_000;

/// Maximum edges in a graph export payload.
pub const MAX_GRAPH_EDGES: usize = 5_000;
