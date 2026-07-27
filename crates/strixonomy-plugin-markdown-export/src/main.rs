use clap::Parser;
use std::path::PathBuf;
use strixonomy_catalog::IndexBuilder;
use strixonomy_docs::ExportOptions;
use strixonomy_plugin::ExporterPlugin;
use strixonomy_plugin::PluginOutput;
use strixonomy_plugin_markdown_export::MarkdownExportPlugin;

#[derive(Parser)]
#[command(name = "strixonomy-plugin-markdown-export")]
struct Cli {
    #[arg(default_value = "export")]
    action: String,
    #[arg(long)]
    workspace: PathBuf,
    #[arg(long, default_value = ".strixonomy/plugin-out")]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    if cli.action != "export" {
        eprintln!("only export action is supported");
        std::process::exit(2);
    }
    let catalog = IndexBuilder::new().workspace(&cli.workspace).build().expect("index");
    let plugin = MarkdownExportPlugin;
    let options = ExportOptions::markdown(&cli.output);
    match plugin.export(&catalog, &cli.workspace, options) {
        Ok(_) => {
            let out = PluginOutput {
                output_paths: vec![cli.output.display().to_string()],
                ..Default::default()
            };
            println!("{}", serde_json::to_string(&out).unwrap());
        }
        Err(e) => {
            let out = PluginOutput { exit_message: Some(e.to_string()), ..Default::default() };
            println!("{}", serde_json::to_string(&out).unwrap());
            std::process::exit(1);
        }
    }
}
