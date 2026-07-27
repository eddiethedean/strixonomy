mod discovery;
mod host;
mod lifecycle;
mod manifest;
mod protocol;
mod subprocess;
mod traits;

pub use discovery::{discover_plugins, PluginDiscoveryError, PLUGIN_DIR};
pub use host::{
    manifest_for_builtin, merge_plugin_diagnostics, PluginDescriptor, PluginHost, PluginHostError,
    RunPluginResult,
};
pub use lifecycle::{activation_order, dependents_of, LifecycleError};
pub use manifest::{
    parse_manifest, DiscoveredPlugin, ManifestValidationError, PluginActivation,
    PluginCapabilities, PluginCommandContribution, PluginConfig, PluginContextActionContribution,
    PluginInspectorCard, PluginKind, PluginLifecycleState, PluginManifest, PluginPermission,
    PluginPreferencePageContribution, PluginUiContributions, PluginViewContribution,
};
pub use protocol::{plugin_diagnostic, PluginDiagnosticWire, PluginOutput};
pub use traits::{
    ExporterPlugin, GraphPlugin, GraphProviderResult, QueryPlugin, QueryProviderResult,
    ReasonerPlugin, ReasonerProviderResult, RefactorPlugin, RefactorProviderResult,
    ValidatorPlugin, WorkflowPlugin, WorkflowRequest, WorkflowResult,
};
