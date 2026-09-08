//! Storm Sewer — external add-on for Open CAD Studio.
//!
//! Engine: `crates/stormsewer` (headless hydraulics, no CAD deps).
//! CAD bridge: XDATA on entities + `SS_*` commands via `ocs_plugin_api`.

mod analysis;
mod data;
mod dispatch;
mod edit;
mod html_report;
#[cfg(test)]
mod integration_tests;
mod interactive;
mod landxml_import;
pub mod license;
mod license_cmd;
mod params_cmd;
mod placement;
mod sizing;
mod state;
mod style;
mod validation;

use ocs_plugin_api::host::{BuiltinPlugin, HostApi};
use ocs_plugin_api::manifest::PluginManifest;
use ocs_plugin_api::ribbon::{CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef};

pub mod manifest {
    use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};

    pub const PLUGIN_ID: &str = "opencad.storm_sewer";

    pub static MANIFEST: PluginManifest = PluginManifest {
        id: PLUGIN_ID,
        name: "Storm Sewer",
        version: "0.3.2",
        description: "Gravity storm-drain network design and analysis (Pro: HTML reports, pipe sizing, multi-RP)",
        api_version: ApiVersion::CURRENT,
        ribbon_order: 50,
        xdata_apps: &["STORMSEWER_STRUCT", "STORMSEWER_PIPE", "STORMSEWER_CATCHMENT"],
        command_prefixes: &["SS_"],
    };
}

use manifest::MANIFEST;

struct StormSewerModule;

fn tool(id: &'static str, label: &'static str, glyph: &'static str) -> ToolDef {
    ToolDef {
        id,
        label,
        icon: IconKind::Glyph(glyph),
        event: ModuleEvent::Command(id.to_string()),
    }
}

impl CadModule for StormSewerModule {
    fn id(&self) -> &'static str {
        "storm_sewer"
    }
    fn title(&self) -> &'static str {
        "Storm Sewer"
    }

    fn ribbon_groups(&self) -> &[RibbonGroup] {
        static GROUPS: std::sync::OnceLock<Vec<RibbonGroup>> = std::sync::OnceLock::new();
        GROUPS.get_or_init(build_ribbon_groups)
    }
}

fn build_ribbon_groups() -> Vec<RibbonGroup> {
    vec![
        RibbonGroup {
            title: "Network",
            tools: vec![
                RibbonItem::LargeTool(tool("SS_INLET", "Inlet", "◉")),
                RibbonItem::LargeTool(tool("SS_JUNCTION", "Junction", "◎")),
                RibbonItem::LargeTool(tool("SS_OUTFALL", "Outfall", "▽")),
                RibbonItem::LargeTool(tool("SS_PIPE", "Pipe\nRun", "╱")),
                RibbonItem::Tool(ToolDef {
                    id: "SS_IMPORTXML",
                    label: "Import\nLandXML",
                    icon: IconKind::Glyph("⬇"),
                    event: ModuleEvent::PluginFileDialog {
                        command: "SS_IMPORTXML".to_string(),
                        title: "Import LandXML pipe network".to_string(),
                        filter_name: "LandXML".to_string(),
                        extensions: vec!["xml".to_string(), "landxml".to_string()],
                    },
                }),
                RibbonItem::Tool(tool("SS_APPLYTC", "Apply Tc", "⏱")),
                RibbonItem::Tool(tool("SS_EDIT", "Edit", "✎")),
            ],
        },
        RibbonGroup {
            title: "Analysis",
            tools: vec![
                RibbonItem::LargeTool(tool("SS_ANALYZE", "Analyze", "⚡")),
                RibbonItem::LargeTool(tool("SS_SIZE", "Size\nPipes", "⌀")),
                RibbonItem::Tool(tool("SS_VALIDATE", "Validate", "✓")),
                RibbonItem::Tool(tool("SS_PARAMS", "Params", "⚙")),
                RibbonItem::Tool(tool("SS_MULTIRP", "Multi-RP", "≋")),
                RibbonItem::Tool(tool("SS_REPORT", "Report", "📋")),
                RibbonItem::Tool(tool("SS_REPORT_HTML", "HTML\nReport", "📄")),
                RibbonItem::Tool(tool("SS_PROFILE", "Profile", "▤")),
            ],
        },
        RibbonGroup {
            title: "Pro",
            tools: vec![
                RibbonItem::Tool(tool("SS_LICENSE", "License", "🔑")),
                RibbonItem::Tool(tool("SS_ACTIVATE", "Activate", "✔")),
            ],
        },
    ]
}

struct StormSewerPlugin;

impl BuiltinPlugin for StormSewerPlugin {
    fn manifest(&self) -> &'static PluginManifest {
        &MANIFEST
    }
    fn ribbon(&self) -> Box<dyn CadModule> {
        Box::new(StormSewerModule)
    }
    fn dispatch(&self, host: &mut dyn HostApi, cmd: &str) -> bool {
        dispatch::handle(host, cmd)
    }
}

ocs_plugin_api::export_plugin!(StormSewerPlugin);
