//! Storm Sewer — external add-on for Open CAD Studio.
//!
//! Engine: `crates/stormsewer` (headless hydraulics, no CAD deps).
//! CAD bridge: XDATA on entities + `SS_*` commands via `ocs_plugin_api`.

mod analysis;
mod data;
mod dispatch;
mod params_cmd;
mod sizing;
mod state;
mod style;

use ocs_plugin_api::host::{BuiltinPlugin, HostApi};
use ocs_plugin_api::manifest::PluginManifest;
use ocs_plugin_api::ribbon::{CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef};

pub mod manifest {
    use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};

    pub const PLUGIN_ID: &str = "opencad.storm_sewer";

    pub static MANIFEST: PluginManifest = PluginManifest {
        id: PLUGIN_ID,
        name: "Storm Sewer",
        version: "0.1.0",
        description: "Gravity storm-drain network design and analysis",
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

    fn ribbon_groups(&self) -> Vec<RibbonGroup> {
        vec![
            RibbonGroup {
                title: "Network",
                tools: vec![
                    RibbonItem::LargeTool(tool("SS_INLET", "Inlet", "◉")),
                    RibbonItem::LargeTool(tool("SS_JUNCTION", "Junction", "◎")),
                    RibbonItem::LargeTool(tool("SS_OUTFALL", "Outfall", "▽")),
                    RibbonItem::LargeTool(tool("SS_PIPE", "Pipe\nRun", "╱")),
                ],
            },
            RibbonGroup {
                title: "Analysis",
                tools: vec![
                    RibbonItem::LargeTool(tool("SS_ANALYZE", "Analyze", "⚡")),
                    RibbonItem::LargeTool(tool("SS_SIZE", "Size\nPipes", "⌀")),
                    RibbonItem::Tool(tool("SS_PARAMS", "Params", "⚙")),
                    RibbonItem::Tool(tool("SS_MULTIRP", "Multi-RP", "≋")),
                    RibbonItem::Tool(tool("SS_REPORT", "Report", "📋")),
                    RibbonItem::Tool(tool("SS_PROFILE", "Profile", "▤")),
                ],
            },
        ]
    }
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