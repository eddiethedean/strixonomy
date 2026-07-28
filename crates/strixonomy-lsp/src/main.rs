fn main() {
    if let Err(e) = strixonomy_lsp::run() {
        eprintln!("strixonomy-lsp error: {e}");
        std::process::exit(1);
    }
}
