fn main() {
    let argv0 = std::env::args().next().unwrap_or_default();
    let name = std::path::Path::new(&argv0).file_stem().and_then(|s| s.to_str()).unwrap_or("");
    if name == "ontocore-lsp" {
        eprintln!(
            "warning: `ontocore-lsp` is deprecated; use `strixonomy-lsp` instead (see docs/migration/v0.27.md)"
        );
    }
    if let Err(e) = strixonomy_lsp::run() {
        eprintln!("strixonomy-lsp error: {e}");
        std::process::exit(1);
    }
}
