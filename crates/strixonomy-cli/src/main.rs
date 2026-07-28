use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use strixonomy_catalog::{CatalogStats, IndexBuilder, OntologyCatalog};
use strixonomy_diff::{
    apply_unsat_diff, catalog_at_git_ref, catalog_at_worktree, diff_catalogs,
    diff_git_refs_with_catalogs, format_diff_json, format_diff_markdown, format_diff_pr_summary,
    format_diff_text, parse_git_range, DiffResult,
};
use strixonomy_docs::{export_workspace, ExportFormat, ExportOptions};
use strixonomy_plugin_builtins::load_plugin_host;
use strixonomy_query::{
    query_catalog,
    sparql::to_json as sparql_to_json,
    sparql_catalog,
    sql::{to_csv as sql_to_csv, to_json as sql_to_json},
};
use strixonomy_reasoner::{
    check_instance, classify, explain, realize, run_dl_query, DlQueryMode, ExplanationRequest,
    ReasonerId, WorkspaceInputLoader,
};
use strixonomy_refactor::{
    apply_refactor_plan_checked, find_usages, preview_cleanup_imports, preview_extract_module,
    preview_flatten_imports, preview_merge_entities, preview_merge_ontologies,
    preview_migrate_namespace, preview_move_entity, preview_rename_iri, preview_replace_entity,
    RefactorPlan,
};

#[derive(Parser)]
#[command(
    name = "strixonomy",
    version,
    about = "Local-first ontology index and query engine (Strixonomy v0.28)"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a minimal ontology file
    New {
        /// Output path (.ttl or .obo)
        path: PathBuf,
        /// Ontology IRI
        #[arg(long)]
        ontology_iri: String,
        #[arg(long)]
        version_iri: Option<String>,
        /// Overwrite the target file if it already exists
        #[arg(long)]
        force: bool,
    },
    /// Scan and index ontology files in a workspace
    Index {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Run a SQL-like query over ontology tables
    Query {
        /// Workspace directory
        workspace: PathBuf,
        /// SQL query string
        sql: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Run a SPARQL query over indexed triples
    Sparql {
        /// Workspace directory
        workspace: PathBuf,
        /// SPARQL query string
        query: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Validate ontology files in a workspace
    Validate {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
    },
    /// Inspect catalog statistics for a workspace
    Inspect {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Apply Turtle patch operations from a JSON file
    Patch {
        /// Turtle document to patch
        document: PathBuf,
        /// JSON file containing an array of patch operations
        patch_file: PathBuf,
        /// Preview changes without writing to disk
        #[arg(long)]
        preview: bool,
    },
    /// Classify ontologies in a workspace with a reasoner profile
    Classify {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
        /// Reasoner profile: el, rl, rdfs, dl, auto
        #[arg(long, default_value = "el")]
        profile: String,
        /// Emit profile-detection warnings
        #[arg(long, default_value_t = true)]
        auto_profile: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Explain unsatisfiability for a class IRI
    Explain {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
        /// Class IRI to explain
        #[arg(long)]
        class: String,
        /// Reasoner profile
        #[arg(long, default_value = "el")]
        profile: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Realize individuals (inferred types) in a workspace
    Realize {
        #[arg(default_value = ".")]
        workspace: PathBuf,
        #[arg(long, default_value = "rl")]
        profile: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Check whether an individual is an instance of a class
    CheckInstance {
        #[arg(default_value = ".")]
        workspace: PathBuf,
        #[arg(long)]
        individual: String,
        #[arg(long)]
        class: String,
        #[arg(long, default_value = "rl")]
        profile: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Protégé-style DL Query (Manchester class expression)
    DlQuery {
        /// Manchester class expression
        expression: String,
        /// Workspace directory
        #[arg(long, default_value = ".")]
        workspace: PathBuf,
        #[arg(long, default_value = "dl")]
        profile: String,
        /// `inferred` (default) or `asserted`
        #[arg(long, default_value = "inferred")]
        mode: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Run ROBOT CLI subcommands (validate, merge, report)
    Robot {
        #[command(subcommand)]
        command: RobotCommands,
    },
    /// Workspace refactoring (rename, migrate, move, extract)
    Refactor {
        #[command(subcommand)]
        command: RefactorCommands,
    },
    /// Export Markdown or HTML documentation for a workspace
    Docs {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
        /// Output directory
        #[arg(long, short = 'o')]
        output: PathBuf,
        #[arg(long, value_enum, default_value = "markdown")]
        format: DocsFormat,
        /// Limit export to a single ontology id / base IRI
        #[arg(long)]
        ontology_id: Option<String>,
        /// Exporter plugin id (default: built-in docs export)
        #[arg(long)]
        plugin: Option<String>,
    },
    /// Semantic diff between git refs, directories, or indexed snapshots
    Diff {
        /// Git range (`main..feature`), single ref vs working tree, or omitted with `--left-ref`/`--right-ref`
        #[arg(value_name = "GIT_RANGE")]
        git_range: Option<String>,
        /// Left git ref or directory path
        #[arg(long)]
        left_ref: Option<String>,
        /// Right git ref, directory path, or `WORKTREE` for working tree
        #[arg(long)]
        right_ref: Option<String>,
        /// Git repository root (defaults to current directory)
        #[arg(long)]
        repo: Option<PathBuf>,
        /// Enrich diff with reasoner unsatisfiability changes (requires resolvable workspace paths)
        #[arg(long)]
        reasoner: bool,
        #[arg(long, value_enum, default_value_t = DiffFormat::Text)]
        format: DiffFormat,
        #[arg(long)]
        breaking_only: bool,
        /// Emit a Markdown summary suitable for pull request descriptions
        #[arg(long)]
        pr_summary: bool,
    },
    /// Discover and manage workspace plugins
    Plugins {
        #[command(subcommand)]
        command: PluginCommands,
    },
    /// Run an external workflow plugin (e.g. owlmake)
    Workflow {
        /// Workspace directory
        #[arg(default_value = ".")]
        workspace: PathBuf,
        /// Plugin id from `.strixonomy/plugins/`
        #[arg(long)]
        plugin: String,
        /// Workflow step (build, qc, release, report)
        #[arg(long, default_value = "qc")]
        step: String,
    },
}

#[derive(Subcommand)]
enum PluginCommands {
    /// List discovered plugins
    List {
        #[arg(default_value = ".")]
        workspace: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Show plugin lifecycle and dependency info
    Info {
        /// Plugin id
        plugin_id: String,
        #[arg(default_value = ".")]
        workspace: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Enable a disabled plugin (activates dependents per activation policy)
    Enable {
        plugin_id: String,
        #[arg(default_value = ".")]
        workspace: PathBuf,
    },
    /// Disable a plugin and cascade-deactivate dependents
    Disable {
        plugin_id: String,
        #[arg(default_value = ".")]
        workspace: PathBuf,
    },
    /// Run a plugin action
    Run {
        /// Plugin id
        plugin_id: String,
        #[arg(default_value = ".")]
        workspace: PathBuf,
        /// Action: validate, export, workflow, reasoner.classify, query.run, refactor.preview, graph.build
        #[arg(long, default_value = "validate")]
        action: String,
        #[arg(long)]
        step: Option<String>,
        /// Query text for `query.run`
        #[arg(long)]
        query: Option<String>,
        /// Focus / root IRI for refactor.preview or graph.build
        #[arg(long)]
        iri: Option<String>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Subcommand)]
enum RefactorCommands {
    /// List usages of an entity IRI across the workspace
    Usages {
        workspace: PathBuf,
        iri: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Rename an entity IRI across Turtle files
    Rename {
        workspace: PathBuf,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Migrate a namespace base IRI across the workspace
    MigrateNamespace {
        workspace: PathBuf,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Move an entity block to another Turtle file
    Move {
        workspace: PathBuf,
        iri: String,
        #[arg(long)]
        to: PathBuf,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Extract selected entities into a new module file
    Extract {
        workspace: PathBuf,
        #[arg(long, value_delimiter = ',')]
        entities: Vec<String>,
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        leave_stub: bool,
        /// Close seed signature under bottom-locality heuristic before extract
        #[arg(long)]
        locality: bool,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Merge one entity into another (rewrite refs to keep IRI, drop merge declaration)
    Merge {
        workspace: PathBuf,
        #[arg(long)]
        keep: String,
        #[arg(long)]
        merge: String,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Replace references to one entity with another (keeps source declaration when target exists)
    Replace {
        workspace: PathBuf,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Merge one or more Turtle ontology files into a target file
    MergeOntologies {
        workspace: PathBuf,
        #[arg(long, required = true)]
        sources: Vec<PathBuf>,
        #[arg(long)]
        target: PathBuf,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Inline imported Turtle axioms into a root ontology and remove owl:imports
    FlattenImports {
        workspace: PathBuf,
        #[arg(long)]
        file: PathBuf,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Remove unused owl:imports (heuristic: no imported entities referenced)
    CleanupImports {
        workspace: PathBuf,
        #[arg(long)]
        file: PathBuf,
        #[arg(long)]
        preview: bool,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Subcommand)]
enum RobotCommands {
    /// Run `robot validate`
    Validate {
        /// Ontology file or directory
        path: PathBuf,
        #[arg(long)]
        robot_path: Option<String>,
    },
    /// Run `robot merge`
    Merge {
        #[arg(long, required = true)]
        inputs: Vec<PathBuf>,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        robot_path: Option<String>,
    },
    /// Run `robot report`
    Report {
        /// Ontology file or directory
        path: PathBuf,
        #[arg(long)]
        report: PathBuf,
        #[arg(long)]
        robot_path: Option<String>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum DocsFormat {
    Markdown,
    Html,
}

#[derive(Clone, Copy, ValueEnum)]
enum DiffFormat {
    Text,
    Json,
    Markdown,
    #[value(name = "pr-summary")]
    PrSummary,
}

#[derive(Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Csv,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { path, ontology_iri, version_iri, force } => {
            write_new_ontology(&path, &ontology_iri, version_iri.as_deref(), force)?;
            println!("Created {}", path.display());
        }
        Commands::Index { workspace, format } => {
            let catalog = build_catalog(&workspace)?;
            print_stats(&catalog.data().stats(), format)?;
        }
        Commands::Query { workspace, sql, format } => {
            let catalog = build_catalog(&workspace)?;
            let result = query_catalog(&catalog, &sql).context("query failed")?;
            print_query_result(&result.columns, &result.rows, result.truncated, format)?;
        }
        Commands::Sparql { workspace, query, format } => {
            let catalog = build_catalog(&workspace)?;
            let result = sparql_catalog(&catalog, &query).context("sparql failed")?;
            match format {
                OutputFormat::Json => println!("{}", sparql_to_json(&result)?),
                _ => print_query_result(&result.columns, &result.rows, result.truncated, format)?,
            }
        }
        Commands::Validate { workspace } => {
            let catalog = build_catalog(&workspace)?;
            let data = catalog.data();
            let mut error_count = 0usize;
            let mut warning_count = 0usize;

            let mut all_diags = data.diagnostics.clone();
            if let Ok(host) = load_plugin_host(&workspace) {
                let plugin_diags = host.run_all_validators(&catalog);
                all_diags.extend(plugin_diags);
            }

            for diag in &all_diags {
                let code = diag.display_code();
                match diag.severity {
                    strixonomy_core::DiagnosticSeverity::Error => {
                        error_count += 1;
                        eprintln!(
                            "ERROR [{code}] {}:{}:{}: {}",
                            diag.file.display(),
                            diag.range.line.unwrap_or(0),
                            diag.range.column.unwrap_or(0),
                            diag.message
                        );
                    }
                    strixonomy_core::DiagnosticSeverity::Warning => {
                        warning_count += 1;
                        eprintln!(
                            "WARN  [{code}] {}:{}:{}: {}",
                            diag.file.display(),
                            diag.range.line.unwrap_or(0),
                            diag.range.column.unwrap_or(0),
                            diag.message
                        );
                    }
                    strixonomy_core::DiagnosticSeverity::Info => {
                        eprintln!("INFO  [{code}] {}: {}", diag.file.display(), diag.message);
                    }
                }
            }

            if error_count > 0 {
                bail!("validation failed with {error_count} error(s), {warning_count} warning(s)");
            }
            println!(
                "OK: indexed {} ontology file(s), {} warning(s)",
                data.stats().ontology_count,
                warning_count
            );
        }
        Commands::Inspect { workspace, format } => {
            let catalog = build_catalog(&workspace)?;
            print_stats(&catalog.data().stats(), format)?;
            let data = catalog.data();
            let mut errors = 0usize;
            let mut warnings = 0usize;
            for diag in &data.diagnostics {
                match diag.severity {
                    strixonomy_core::DiagnosticSeverity::Error => errors += 1,
                    strixonomy_core::DiagnosticSeverity::Warning => warnings += 1,
                    _ => {}
                }
            }
            if errors > 0 || warnings > 0 {
                println!("\nDiagnostics: {errors} error(s), {warnings} warning(s)");
                for diag in data.diagnostics.iter().take(10) {
                    println!(
                        "  [{}] {} — {}",
                        diag.code.as_str(),
                        diag.severity.as_str(),
                        diag.message
                    );
                }
                if data.diagnostics.len() > 10 {
                    println!(
                        "  … and {} more (run `strixonomy validate` for full list)",
                        data.diagnostics.len() - 10
                    );
                }
            } else {
                println!("\nNo diagnostics.");
            }
        }
        Commands::Patch { document, patch_file, preview } => {
            let patch_bytes =
                strixonomy_core::read_file_capped(&patch_file, strixonomy_core::MAX_FILE_BYTES)
                    .with_context(|| {
                        format!("failed to read patch file {}", patch_file.display())
                    })?;
            let ext =
                document.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
            let catalog = IndexBuilder::new()
                .workspace(document.parent().unwrap_or(std::path::Path::new(".")))
                .build()
                .context("failed to index workspace for patch namespaces")?;
            let namespaces = catalog
                .data()
                .documents
                .iter()
                .find(|d| strixonomy_core::paths_refer_to_same(&d.path, &document))
                .map(|d| d.namespaces.clone())
                .unwrap_or_default();
            if ext == "obo" {
                let value: serde_json::Value =
                    serde_json::from_slice(&patch_bytes).context("failed to parse patch JSON")?;
                let transaction = strixonomy_edit::parse_obo_input(value)
                    .context("failed to parse OBO transaction")?;
                let result = strixonomy_edit::apply_transaction_to_path(
                    &transaction,
                    &document,
                    preview,
                    &namespaces,
                )
                .context("patch failed")?;
                println!("{}", serde_json::to_string_pretty(&result)?);
                let has_errors = result.diagnostics.iter().any(|d| d.severity == "error");
                if has_errors || (!preview && !transaction.is_empty() && !result.applied) {
                    bail!("patch failed with {} diagnostic(s)", result.diagnostics.len().max(1));
                }
                if !preview && result.applied {
                    println!("applied");
                }
            } else if matches!(ext.as_str(), "ttl" | "owl" | "rdf" | "owx") {
                let value: serde_json::Value =
                    serde_json::from_slice(&patch_bytes).context("failed to parse patch JSON")?;
                let transaction = strixonomy_edit::parse_turtle_input(value)
                    .context("failed to parse patch transaction")?;
                let result = strixonomy_edit::apply_transaction_to_path(
                    &transaction,
                    &document,
                    preview,
                    &namespaces,
                )
                .context("patch failed")?;
                println!("{}", serde_json::to_string_pretty(&result)?);
                let has_errors = result.diagnostics.iter().any(|d| d.severity == "error");
                if has_errors || (!preview && !transaction.is_empty() && !result.applied) {
                    bail!("patch failed with {} diagnostic(s)", result.diagnostics.len().max(1));
                }
                if !preview && result.applied {
                    println!("applied");
                }
            } else {
                bail!("patch write-back supports .ttl, .obo, .owl, .rdf, and .owx documents only");
            }
        }
        Commands::Classify { workspace, profile, auto_profile, format } => {
            let result = run_classify(&workspace, &profile, auto_profile)?;
            match format {
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&result)?),
                OutputFormat::Text | OutputFormat::Csv => {
                    println!("profile: {}", result.profile_used);
                    println!("consistent: {}", result.consistent);
                    println!("unsatisfiable: {}", result.unsatisfiable.len());
                    println!("inferred_edges: {}", result.inferred.edges.len());
                    println!("new_inferences: {}", result.new_inferences.len());
                    println!("duration_ms: {}", result.duration_ms);
                    for iri in &result.unsatisfiable {
                        println!("UNSAT {iri}");
                    }
                    for edge in &result.new_inferences {
                        println!("INFERRED {} SubClassOf {}", edge.child, edge.parent);
                    }
                }
            }
            if !result.consistent {
                bail!(
                    "classification found {} unsatisfiable class(es)",
                    result.unsatisfiable.len()
                );
            }
        }
        Commands::Explain { workspace, class, profile, format } => {
            let result = run_explain(&workspace, &class, &profile)?;
            match format {
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&result)?),
                OutputFormat::Text | OutputFormat::Csv => {
                    println!("class: {}", result.class_iri);
                    println!("{}", result.text);
                }
            }
        }
        Commands::Realize { workspace, profile, format } => {
            let profile_id = ReasonerId::parse(&profile).map_err(|e| anyhow::anyhow!(e))?;
            let input =
                WorkspaceInputLoader::new(&workspace).load().map_err(|e| anyhow::anyhow!(e))?;
            let result = realize(profile_id, &input).map_err(|e| anyhow::anyhow!(e))?;
            match format {
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&result)?),
                OutputFormat::Text | OutputFormat::Csv => {
                    println!("profile: {}", result.profile_used);
                    println!("individuals: {}", result.individuals.len());
                    for entry in &result.individuals {
                        println!(
                            "{} types=[{}] most_specific=[{}]",
                            entry.individual_iri,
                            entry.types.join(", "),
                            entry.most_specific.join(", ")
                        );
                    }
                }
            }
        }
        Commands::CheckInstance { workspace, individual, class, profile, format } => {
            let profile_id = ReasonerId::parse(&profile).map_err(|e| anyhow::anyhow!(e))?;
            let input =
                WorkspaceInputLoader::new(&workspace).load().map_err(|e| anyhow::anyhow!(e))?;
            let result = check_instance(profile_id, &input, &individual, &class)
                .map_err(|e| anyhow::anyhow!(e))?;
            match format {
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&result)?),
                OutputFormat::Text | OutputFormat::Csv => {
                    println!(
                        "{} instanceOf {} => {} ({}ms)",
                        result.individual_iri,
                        result.class_iri,
                        result.entailed,
                        result.duration_ms
                    );
                }
            }
            if !result.entailed {
                bail!("instance check failed");
            }
        }
        Commands::DlQuery { workspace, expression, profile, mode, format } => {
            let profile_id = ReasonerId::parse(&profile).map_err(|e| anyhow::anyhow!(e))?;
            let mode = match mode.to_ascii_lowercase().as_str() {
                "asserted" => DlQueryMode::Asserted,
                _ => DlQueryMode::Inferred,
            };
            let input =
                WorkspaceInputLoader::new(&workspace).load().map_err(|e| anyhow::anyhow!(e))?;
            let namespaces = collect_workspace_namespaces(&workspace)?;
            let result = run_dl_query(profile_id, &input, &expression, &namespaces, mode)
                .map_err(|e| anyhow::anyhow!(e))?;
            match format {
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&result)?),
                OutputFormat::Text | OutputFormat::Csv => {
                    println!("expression: {}", result.expression);
                    println!("normalized: {}", result.normalized);
                    println!("profile: {}", result.profile);
                    println!("mode: {:?}", result.mode);
                    println!("duration_ms: {}", result.duration_ms);
                    println!("subclasses: {}", result.subclasses.len());
                    for iri in &result.subclasses {
                        println!("  SUB {iri}");
                    }
                    println!("superclasses: {}", result.superclasses.len());
                    for iri in &result.superclasses {
                        println!("  SUPER {iri}");
                    }
                    println!("equivalents: {}", result.equivalents.len());
                    for iri in &result.equivalents {
                        println!("  EQ {iri}");
                    }
                    println!("instances: {}", result.instances.len());
                    for iri in &result.instances {
                        println!("  INST {iri}");
                    }
                    for w in &result.warnings {
                        println!("WARN {w}");
                    }
                }
            }
        }
        Commands::Robot { command } => {
            use strixonomy_robot::{robot_merge, robot_report, robot_validate};
            let output = match command {
                RobotCommands::Validate { path, robot_path } => {
                    robot_validate(robot_path.as_deref(), &path)?
                }
                RobotCommands::Merge { inputs, output, robot_path } => {
                    let input_strs: Vec<String> =
                        inputs.iter().map(|p| p.display().to_string()).collect();
                    robot_merge(robot_path.as_deref(), &input_strs, &output)?
                }
                RobotCommands::Report { path, report, robot_path } => {
                    robot_report(robot_path.as_deref(), &path, &report)?
                }
            };
            if !output.stdout.is_empty() {
                print!("{}", output.stdout);
            }
            if !output.stderr.is_empty() {
                eprint!("{}", output.stderr);
            }
            if output.exit_code != 0 {
                std::process::exit(output.exit_code);
            }
        }
        Commands::Refactor { command } => match command {
            RefactorCommands::Usages { workspace, iri, format } => {
                let catalog = build_catalog(&workspace)?;
                let usages = find_usages(&catalog, &iri);
                match format {
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&usages)?);
                    }
                    _ => {
                        for u in usages {
                            println!(
                                "{}:{}:{} {:?} {}",
                                u.file.display(),
                                u.line.unwrap_or(0),
                                u.column.unwrap_or(0),
                                u.kind,
                                u.context
                            );
                        }
                    }
                }
            }
            RefactorCommands::Rename { workspace, from, to, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let plan = preview_rename_iri(&catalog, &from, &to, &HashMap::new())?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
            RefactorCommands::MigrateNamespace { workspace, from, to, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let plan = preview_migrate_namespace(&catalog, &from, &to, &HashMap::new())?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
            RefactorCommands::Move { workspace, iri, to, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let roots = vec![workspace.clone()];
                let plan = preview_move_entity(&catalog, &iri, &to, &HashMap::new(), &roots)?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
            RefactorCommands::Extract {
                workspace,
                entities,
                out,
                leave_stub,
                locality,
                preview,
                format,
            } => {
                let catalog = build_catalog(&workspace)?;
                let roots = vec![workspace.clone()];
                let plan = preview_extract_module(
                    &catalog,
                    &entities,
                    &out,
                    leave_stub,
                    locality,
                    &HashMap::new(),
                    &roots,
                )?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
            RefactorCommands::Merge { workspace, keep, merge, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let plan = preview_merge_entities(&catalog, &keep, &merge, &HashMap::new())?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
            RefactorCommands::Replace { workspace, from, to, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let plan = preview_replace_entity(&catalog, &from, &to, &HashMap::new())?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
            RefactorCommands::MergeOntologies { workspace, sources, target, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let roots = vec![workspace.clone()];
                let plan =
                    preview_merge_ontologies(&catalog, &sources, &target, &HashMap::new(), &roots)?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
            RefactorCommands::FlattenImports { workspace, file, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let roots = vec![workspace.clone()];
                let plan = preview_flatten_imports(&catalog, &file, &HashMap::new(), &roots)?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
            RefactorCommands::CleanupImports { workspace, file, preview, format } => {
                let catalog = build_catalog(&workspace)?;
                let roots = vec![workspace.clone()];
                let plan = preview_cleanup_imports(&catalog, &file, &HashMap::new(), &roots)?;
                run_refactor_plan(&plan, preview, format, &workspace)?;
            }
        },
        Commands::Docs { workspace, output, format, ontology_id, plugin } => {
            let catalog = build_catalog(&workspace)?;
            if let Some(plugin_id) = plugin {
                let host = load_plugin_host(&workspace)?;
                let export_format = match format {
                    DocsFormat::Markdown => ExportFormat::Markdown,
                    DocsFormat::Html => ExportFormat::Html,
                };
                let mut options = ExportOptions {
                    output_dir: output.clone(),
                    format: export_format,
                    ontology_id: None,
                };
                if let Some(id) = ontology_id {
                    options = options.with_ontology_id(id);
                }
                let result = host
                    .run_export_plugin(&plugin_id, &catalog, options)
                    .with_context(|| format!("plugin export '{plugin_id}' failed"))?;
                if !result.output_paths.is_empty() {
                    println!("Wrote: {}", result.output_paths.join(", "));
                } else {
                    println!("Wrote documentation to {}", output.display());
                }
            } else {
                let export_format = match format {
                    DocsFormat::Markdown => ExportFormat::Markdown,
                    DocsFormat::Html => ExportFormat::Html,
                };
                let mut options =
                    ExportOptions { output_dir: output, format: export_format, ontology_id: None };
                if let Some(id) = ontology_id {
                    options = options.with_ontology_id(id);
                }
                export_workspace(&catalog, options.clone()).context("docs export failed")?;
                println!("Wrote documentation to {}", options.output_dir.display());
            }
        }
        Commands::Diff {
            git_range,
            left_ref,
            right_ref,
            repo,
            reasoner,
            format,
            breaking_only,
            pr_summary,
        } => {
            let diff = run_diff(
                git_range.as_deref(),
                left_ref.as_deref(),
                right_ref.as_deref(),
                repo.as_deref(),
                reasoner,
            )?;
            let output = if pr_summary || matches!(format, DiffFormat::PrSummary) {
                format_diff_pr_summary(&diff)
            } else {
                match format {
                    DiffFormat::Json => format_diff_json(&diff),
                    DiffFormat::Markdown => format_diff_markdown(&diff, breaking_only),
                    DiffFormat::Text | DiffFormat::PrSummary => {
                        format_diff_text(&diff, breaking_only)
                    }
                }
            };
            println!("{output}");
        }
        Commands::Plugins { command } => match command {
            PluginCommands::List { workspace, format } => {
                let host = load_plugin_host(&workspace)?;
                let plugins = host.list_plugins();
                match format {
                    OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&plugins)?),
                    _ => {
                        if plugins.is_empty() {
                            println!("No plugins discovered under .strixonomy/plugins/");
                        }
                        for p in plugins {
                            println!(
                                "{} {} ({}) state={} — validate={} export={} reasoner={} query={} refactor={} graph={} in_process={}",
                                p.id,
                                p.version,
                                p.kind,
                                p.state,
                                p.capabilities.validate,
                                p.capabilities.export,
                                p.capabilities.reasoner,
                                p.capabilities.query,
                                p.capabilities.refactor,
                                p.capabilities.graph,
                                p.in_process
                            );
                        }
                    }
                }
            }
            PluginCommands::Info { workspace, plugin_id, format } => {
                let host = load_plugin_host(&workspace)?;
                let info = host
                    .plugin_info(&plugin_id)
                    .with_context(|| format!("plugin not found: {plugin_id}"))?;
                match format {
                    OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&info)?),
                    _ => {
                        println!("{} {} ({})", info.id, info.version, info.kind);
                        println!("  state: {}", info.state);
                        println!("  activation: {}", info.activation);
                        println!("  enabled: {}", info.enabled);
                        println!("  depends_on: {}", info.depends_on.join(", "));
                        println!("  manifest: {}", info.manifest_path);
                    }
                }
            }
            PluginCommands::Enable { workspace, plugin_id } => {
                let mut host = load_plugin_host(&workspace)?;
                host.enable_plugin(&plugin_id)
                    .with_context(|| format!("enable failed for {plugin_id}"))?;
                println!("enabled {plugin_id} (state={})", host.state_of(&plugin_id).as_str());
            }
            PluginCommands::Disable { workspace, plugin_id } => {
                let mut host = load_plugin_host(&workspace)?;
                host.disable_plugin(&plugin_id)
                    .with_context(|| format!("disable failed for {plugin_id}"))?;
                println!("disabled {plugin_id}");
            }
            PluginCommands::Run { workspace, plugin_id, action, step, query, iri, format } => {
                let catalog = build_catalog(&workspace)?;
                let mut host = load_plugin_host(&workspace)?;
                let result = host
                    .run_plugin_action(
                        &plugin_id,
                        &action,
                        Some(&catalog),
                        None,
                        step.as_deref(),
                        None,
                        query.as_deref(),
                        iri.as_deref(),
                    )
                    .with_context(|| format!("plugin run failed for {plugin_id}"))?;
                match format {
                    OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&result)?),
                    _ => {
                        if let Some(logs) = &result.logs {
                            println!("{logs}");
                        }
                        for diag in &result.diagnostics {
                            println!("[{}] {}", diag.display_code(), diag.message);
                        }
                        if !result.output_paths.is_empty() {
                            println!("output: {}", result.output_paths.join(", "));
                        }
                        if let Some(cols) = &result.columns {
                            println!("columns: {}", cols.join(", "));
                        }
                        if let Some(unsat) = &result.unsatisfiable {
                            println!("unsatisfiable: {}", unsat.join(", "));
                        }
                        if let Some(iris) = &result.affected_iris {
                            println!("affected: {}", iris.join(", "));
                        }
                        if let Some(kind) = &result.graph_kind {
                            println!("graph_kind: {kind}");
                        }
                    }
                }
            }
        },
        Commands::Workflow { workspace, plugin, step } => {
            let host = load_plugin_host(&workspace)?;
            let result = host
                .run_workflow_plugin(&plugin, &step)
                .with_context(|| format!("workflow plugin '{plugin}' failed"))?;
            if let Some(logs) = &result.logs {
                println!("{logs}");
            }
            for diag in &result.diagnostics {
                eprintln!("[{}] {}", diag.display_code(), diag.message);
            }
            if !result.success {
                bail!("workflow step '{step}' failed");
            }
            println!("workflow '{step}' completed");
        }
    }
    Ok(())
}

fn write_new_ontology(
    path: &Path,
    ontology_iri: &str,
    version_iri: Option<&str>,
    force: bool,
) -> Result<()> {
    if path.exists() && !force {
        bail!("file already exists: {} (use --force to overwrite)", path.display());
    }
    // Refuse to create/overwrite through a symlink (dangling or otherwise) — closes TOCTOU
    // / path-jail escapes when the leaf is replaced with a link after an exists() check.
    if let Ok(meta) = std::fs::symlink_metadata(path) {
        if meta.file_type().is_symlink() {
            bail!(
                "refusing to write through symlink: {} (remove the symlink or choose another path)",
                path.display()
            );
        }
    }
    if !strixonomy_owl::is_safe_iri(ontology_iri) {
        bail!("ontology IRI contains characters that cannot be safely written: {ontology_iri:?}");
    }
    if let Some(version_iri) = version_iri {
        if !strixonomy_owl::is_safe_iri(version_iri) {
            bail!("version IRI contains characters that cannot be safely written: {version_iri:?}");
        }
    }
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();
    let contents = match extension.to_ascii_lowercase().as_str() {
        "ttl" => {
            let declaration = match version_iri {
                Some(version_iri) => format!(
                    "<{ontology_iri}> a owl:Ontology ;\n    owl:versionIRI <{version_iri}> ."
                ),
                None => format!("<{ontology_iri}> a owl:Ontology ."),
            };
            format!(
                "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
                 @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n\
                 @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
                 @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n\n\
                 {declaration}\n"
            )
        }
        "obo" => {
            let mut contents = format!("format-version: 1.2\nontology: {ontology_iri}\n");
            if let Some(version_iri) = version_iri {
                contents.push_str(&format!("data-version: {version_iri}\n"));
            }
            contents
        }
        _ => bail!("new ontology path must have a .ttl or .obo extension"),
    };
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true);
    if force {
        opts.create(true).truncate(true);
    } else {
        opts.create_new(true);
    }
    let mut file = opts
        .open(path)
        .with_context(|| format!("failed to create ontology file {}", path.display()))?;
    file.write_all(contents.as_bytes())
        .with_context(|| format!("failed to write ontology file {}", path.display()))
}

fn run_refactor_plan(
    plan: &RefactorPlan,
    preview: bool,
    format: OutputFormat,
    workspace: &Path,
) -> Result<()> {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(plan)?),
        _ => {
            for change in &plan.changes {
                println!("{}: {} byte(s) changed", change.path.display(), change.hunks.len());
            }
            for w in &plan.warnings {
                eprintln!("WARN: {w}");
            }
        }
    }
    let root = workspace.canonicalize().unwrap_or_else(|_| workspace.to_path_buf());
    let files_written =
        apply_refactor_plan_checked(plan, preview, Some(std::slice::from_ref(&root)))?;
    if !preview {
        println!("applied {files_written} file(s)");
    }
    Ok(())
}

fn run_diff(
    git_range: Option<&str>,
    left_ref: Option<&str>,
    right_ref: Option<&str>,
    repo: Option<&Path>,
    reasoner: bool,
) -> Result<DiffResult> {
    let repo_root = repo
        .map(Path::to_path_buf)
        .or_else(|| std::env::current_dir().ok())
        .context("could not determine git repository root")?;

    let (mut diff, base_cat, head_cat) = if let Some(spec) = git_range {
        let (left, right) = parse_git_range(spec).map_err(|e| anyhow::anyhow!(e))?;
        let (diff, base, head) = diff_git_refs_with_catalogs(&repo_root, &left, &right)
            .map_err(|e| anyhow::anyhow!(e))?;
        (diff, Some(base), Some(head))
    } else {
        let left = left_ref.context("provide --left-ref or a git range")?;
        let right = right_ref.context("provide --right-ref or a git range")?;
        diff_from_refs_with_catalogs(left, right, &repo_root)?
    };

    if reasoner {
        if let (Some(base), Some(head)) = (base_cat, head_cat) {
            apply_reasoner_unsat_catalogs(&mut diff, &base, &head)?;
        } else {
            eprintln!("WARN: --reasoner skipped (could not resolve catalogs for both sides)");
        }
    }
    Ok(diff)
}

fn diff_from_refs_with_catalogs(
    left: &str,
    right: &str,
    repo: &Path,
) -> Result<(DiffResult, Option<OntologyCatalog>, Option<OntologyCatalog>)> {
    let left = left.trim();
    let right = right.trim();
    if left.is_empty() || right.is_empty() {
        bail!("empty --left-ref or --right-ref");
    }
    let left_path = Path::new(left);
    let right_path = Path::new(right);
    let left_is_dir = left_path.is_dir();
    let right_is_dir = right_path.is_dir();
    match (left_is_dir, right_is_dir) {
        (true, true) => {
            let left_cat = build_catalog(&left_path.to_path_buf())?;
            let right_cat = build_catalog(&right_path.to_path_buf())?;
            Ok((diff_catalogs(&left_cat, &right_cat), Some(left_cat), Some(right_cat)))
        }
        (false, false) => {
            let (diff, base, head) =
                diff_git_refs_with_catalogs(repo, left, right).map_err(|e| anyhow::anyhow!(e))?;
            Ok((diff, Some(base), Some(head)))
        }
        (true, false) => {
            let left_cat = build_catalog(&left_path.to_path_buf())?;
            let right_cat = resolve_git_catalog(repo, right)?;
            Ok((diff_catalogs(&left_cat, &right_cat), Some(left_cat), Some(right_cat)))
        }
        (false, true) => {
            let left_cat = resolve_git_catalog(repo, left)?;
            let right_cat = build_catalog(&right_path.to_path_buf())?;
            Ok((diff_catalogs(&left_cat, &right_cat), Some(left_cat), Some(right_cat)))
        }
    }
}

fn resolve_git_catalog(repo: &Path, git_ref: &str) -> Result<OntologyCatalog> {
    if strixonomy_diff::is_indexed_catalog_ref(git_ref) {
        anyhow::bail!(
            "ref {git_ref}: indexed catalog (INDEXED) is only available via LSP semantic diff; \
             use WORKTREE or a git commit ref in the CLI"
        );
    }
    if strixonomy_diff::is_worktree_ref(git_ref) {
        catalog_at_worktree(repo).map_err(|e| anyhow::anyhow!(e))
    } else {
        catalog_at_git_ref(repo, git_ref).map_err(|e| anyhow::anyhow!(e))
    }
}

fn apply_reasoner_unsat_catalogs(
    diff: &mut DiffResult,
    base: &OntologyCatalog,
    head: &OntologyCatalog,
) -> Result<()> {
    let profile = ReasonerId::El;
    let base_input =
        WorkspaceInputLoader::new(base.workspace()).load().map_err(|e| anyhow::anyhow!(e))?;
    let head_input =
        WorkspaceInputLoader::new(head.workspace()).load().map_err(|e| anyhow::anyhow!(e))?;
    let base_cls = classify(profile, &base_input, true).map_err(|e| anyhow::anyhow!(e))?;
    let head_cls = classify(profile, &head_input, true).map_err(|e| anyhow::anyhow!(e))?;
    apply_unsat_diff(diff, &base_cls.unsatisfiable, &head_cls.unsatisfiable);
    Ok(())
}

fn build_catalog(workspace: &PathBuf) -> Result<OntologyCatalog> {
    IndexBuilder::new()
        .workspace(workspace)
        .build()
        .with_context(|| format!("failed to index workspace {}", workspace.display()))
}

fn collect_workspace_namespaces(
    workspace: &PathBuf,
) -> Result<std::collections::BTreeMap<String, String>> {
    let catalog = build_catalog(workspace)?;
    let mut namespaces = std::collections::BTreeMap::new();
    for doc in &catalog.data().documents {
        for (prefix, iri) in &doc.namespaces {
            namespaces.entry(prefix.clone()).or_insert_with(|| iri.clone());
        }
    }
    Ok(namespaces)
}

fn load_reasoner_input(workspace: &PathBuf) -> Result<strixonomy_reasoner::ReasonerInput> {
    WorkspaceInputLoader::new(workspace).load().map_err(|e| anyhow::anyhow!(e))
}

fn run_classify(
    workspace: &PathBuf,
    profile: &str,
    auto_profile: bool,
) -> Result<strixonomy_reasoner::ClassificationResult> {
    let profile_id = ReasonerId::parse(profile).map_err(|e| anyhow::anyhow!(e))?;
    let input = load_reasoner_input(workspace)?;
    classify(profile_id, &input, auto_profile).map_err(|e| anyhow::anyhow!(e))
}

fn run_explain(
    workspace: &PathBuf,
    class: &str,
    profile: &str,
) -> Result<strixonomy_reasoner::ExplanationResult> {
    let profile_id = ReasonerId::parse(profile).map_err(|e| anyhow::anyhow!(e))?;
    let input = load_reasoner_input(workspace)?;
    explain(profile_id, &input, &ExplanationRequest { class_iri: class.to_string() })
        .map_err(|e| anyhow::anyhow!(e))
}

fn print_stats(stats: &CatalogStats, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(stats)?),
        OutputFormat::Csv | OutputFormat::Text => {
            println!("ontologies:            {}", stats.ontology_count);
            println!("classes:               {}", stats.class_count);
            println!("object_properties:     {}", stats.object_property_count);
            println!("data_properties:       {}", stats.data_property_count);
            println!("annotation_properties: {}", stats.annotation_property_count);
            println!("individuals:           {}", stats.individual_count);
            println!("axioms:                {}", stats.axiom_count);
            println!("annotations:           {}", stats.annotation_count);
            println!("triples:               {}", stats.triple_count);
            println!("parse_errors:          {}", stats.error_count);
            println!("diagnostic_errors:     {}", stats.diagnostic_error_count);
            println!("diagnostic_warnings:   {}", stats.diagnostic_warning_count);
        }
    }
    Ok(())
}

fn print_query_result(
    columns: &[String],
    rows: &[std::collections::BTreeMap<String, String>],
    truncated: bool,
    format: OutputFormat,
) -> Result<()> {
    let result = strixonomy_query::sql::QueryResult {
        columns: columns.to_vec(),
        rows: rows.to_vec(),
        truncated,
    };
    match format {
        OutputFormat::Json => println!("{}", sql_to_json(&result)?),
        OutputFormat::Csv => {
            print!("{}", sql_to_csv(&result)?);
            // Keep CSV body machine-clean; warn on stderr when the result set was capped (#313).
            if truncated {
                eprintln!("WARNING: Strixonomy query results truncated");
            }
        }
        OutputFormat::Text => {
            if columns.is_empty() {
                println!("(no columns)");
            } else {
                println!("{}", columns.join("\t"));
                for row in rows {
                    let line: Vec<String> =
                        columns.iter().map(|c| row.get(c).cloned().unwrap_or_default()).collect();
                    println!("{}", line.join("\t"));
                }
            }
            if truncated {
                println!("# truncated: true");
            }
        }
    }
    Ok(())
}

/// Format helpers used by unit tests for truncation signaling (#313).
#[cfg(test)]
fn text_truncation_marker(truncated: bool) -> Option<&'static str> {
    truncated.then_some("# truncated: true")
}

#[cfg(test)]
fn csv_truncation_warning(truncated: bool) -> Option<&'static str> {
    truncated.then_some("WARNING: Strixonomy query results truncated")
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn plugins_run_accepts_required_plugin_id_before_optional_workspace() {
        let cli = Cli::try_parse_from([
            "strixonomy",
            "plugins",
            "run",
            "strixonomy.naming-validator",
            ".",
            "--action",
            "validate",
        ])
        .expect("plugins run should parse with plugin_id first");
        match cli.command {
            Commands::Plugins {
                command: PluginCommands::Run { plugin_id, workspace, action, .. },
            } => {
                assert_eq!(plugin_id, "strixonomy.naming-validator");
                assert_eq!(workspace, PathBuf::from("."));
                assert_eq!(action, "validate");
            }
            _ => panic!("expected plugins run"),
        }
    }

    #[test]
    fn text_and_csv_surface_truncation_flag() {
        assert_eq!(text_truncation_marker(true), Some("# truncated: true"));
        assert_eq!(text_truncation_marker(false), None);
        assert_eq!(
            csv_truncation_warning(true),
            Some("WARNING: Strixonomy query results truncated")
        );
        assert_eq!(csv_truncation_warning(false), None);
    }
}
