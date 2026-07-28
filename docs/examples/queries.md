# Query cookbook

Runnable examples against an ontology workspace. Replace `/path/to/ontologies` with your project folder (or the [First success](../guides/first-success.md) tutorial directory).

!!! note "Clone vs install"
    `fixtures/` exists only in a git clone. After `cargo install strixonomy-cli`, do not run `strixonomy query ./fixtures` unless you cloned the repo.

```bash
strixonomy query /path/to/ontologies "SELECT * FROM classes"
strixonomy query /path/to/ontologies "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"
```

From a git clone, use `fixtures` instead of `/path/to/ontologies`, or `cargo run --` from the repo root.

## Classes and entities

```bash
strixonomy query /path/to/ontologies "SELECT * FROM classes"
strixonomy query /path/to/ontologies "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"
strixonomy query /path/to/ontologies "SELECT * FROM individuals"
strixonomy query /path/to/ontologies "SELECT * FROM entities"
```

**Expected (`fixtures/`, filtered query):** 1 row with `short_name` = `Person`.

## Properties

```bash
strixonomy query /path/to/ontologies "SELECT * FROM object_properties"
strixonomy query /path/to/ontologies "SELECT * FROM data_properties"
strixonomy query /path/to/ontologies "SELECT * FROM properties"
```

## Annotations and axioms

```bash
strixonomy query /path/to/ontologies "SELECT * FROM annotations"
strixonomy query /path/to/ontologies "SELECT * FROM axioms"
```

## Ontology metadata

```bash
strixonomy query /path/to/ontologies "SELECT * FROM ontologies"
strixonomy query /path/to/ontologies "SELECT * FROM namespaces"
strixonomy query /path/to/ontologies "SELECT * FROM imports"
```

## Diagnostics and validation

```bash
strixonomy query /path/to/ontologies "SELECT code, severity, message, file FROM diagnostics"
strixonomy validate /path/to/ontologies
```

## SPARQL

```bash
strixonomy sparql /path/to/ontologies "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 5"
strixonomy sparql fixtures "PREFIX ex: <http://example.org/people#> PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#> SELECT ?label WHERE { ex:Person rdfs:label ?label }"
```

**Expected (fixtures):** the second query returns **1 row** with a label for `Person` (e.g. `"Person"`).

## Export formats

```bash
strixonomy query /path/to/ontologies "SELECT * FROM classes" --format json
strixonomy query /path/to/ontologies "SELECT * FROM classes" --format csv
```

## CI validation

```bash
strixonomy validate /path/to/ontologies   # exit 0 when no diagnostic errors
```

**Expected (`fixtures/`):** exit code **0** (warnings may be present; errors fail validation).

Full column reference: [sql-reference.md](../sql-reference.md). SPARQL: [sparql-reference.md](../sparql-reference.md). Errors: [errors.md](../errors.md).
