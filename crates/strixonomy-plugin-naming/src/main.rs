use clap::Parser;
use std::path::PathBuf;
use strixonomy_catalog::IndexBuilder;
use strixonomy_plugin::{PluginOutput, ValidatorPlugin};
use strixonomy_plugin_naming::NamingValidatorPlugin;

#[derive(Parser)]
#[command(name = "strixonomy-plugin-naming")]
struct Cli {
    #[arg(default_value = "validate")]
    action: String,
    #[arg(long)]
    workspace: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    if cli.action != "validate" {
        eprintln!("only validate action is supported");
        std::process::exit(2);
    }
    let catalog = match IndexBuilder::new().workspace(&cli.workspace).build() {
        Ok(c) => c,
        Err(e) => {
            let out = PluginOutput { exit_message: Some(e.to_string()), ..Default::default() };
            println!("{}", serde_json::to_string(&out).unwrap());
            std::process::exit(1);
        }
    };
    let plugin = NamingValidatorPlugin::default();
    let diagnostics = plugin
        .validate(&catalog, &cli.workspace)
        .into_iter()
        .map(|d| strixonomy_plugin::PluginDiagnosticWire {
            code: d.plugin_code.unwrap_or_else(|| "missing_label".into()),
            severity: d.severity.as_str().to_string(),
            message: d.message,
            file: d.file.display().to_string(),
            line: d.range.line,
            column: d.range.column,
            entity_iri: d.entity_iri,
        })
        .collect();
    let out = PluginOutput { diagnostics, ..Default::default() };
    println!("{}", serde_json::to_string(&out).unwrap());
}
