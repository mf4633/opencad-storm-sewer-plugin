use std::collections::HashMap;

use ocs_plugin_api::host::{ensure_plugin_state, HostApi};

use acadrust::EntityType;
use acadrust::Handle;

use stormsewer::network::NodeKind;

use super::analysis;
use super::edit;
use super::landxml_import;
use super::manifest::PLUGIN_ID;
use super::params_cmd;
use super::interactive::{PlacePipeInteractive, PlaceStructureInteractive};
use super::placement;
use super::sizing;
use super::state::StormTabState;
use super::validation;
use super::{data, style};

fn tab_params(host: &mut dyn HostApi) -> stormsewer::params::StormAnalysisParams {
    ensure_plugin_state(host, PLUGIN_ID, StormTabState::default)
        .params()
        .clone()
}

fn entities<'a>(host: &'a dyn HostApi) -> impl Iterator<Item = &'a EntityType> {
    host.document().entities()
}

fn entities_mut<'a>(host: &'a mut dyn HostApi) -> impl Iterator<Item = &'a mut EntityType> {
    host.document_mut().entities_mut()
}

/// Everything after the first token (preserves spaces in file paths).
fn command_arg(cmd: &str) -> Option<&str> {
    let mut parts = cmd.splitn(2, char::is_whitespace);
    parts.next()?;
    parts.next().map(str::trim).filter(|s| !s.is_empty())
}

fn run_validation(host: &mut dyn HostApi, block_on_error: bool) -> bool {
    let report = validation::validate_entities(entities(host));
    report.emit_to_host(host);
    if block_on_error && !report.ok() {
        return false;
    }
    true
}

/// Handle any `SS_*` command. Returns true when consumed.
pub fn handle(host: &mut dyn HostApi, cmd: &str) -> bool {
    if !cmd.starts_with("SS_") {
        return false;
    }

    match cmd {
        "SS_VALIDATE" => {
            let _ = run_validation(host, false);
            true
        }
        "SS_ANALYZE" => {
            if !run_validation(host, false) {
                // warnings only — still attempt analyze unless hard errors
            }
            let report = validation::validate_entities(entities(host));
            if !report.ok() {
                return true;
            }
            let params = tab_params(host);
            match analysis::analyze_doc(entities(host), &params) {
                Ok((ents, report, analysis)) => {
                    for e in ents {
                        let _ = host.add_entity(e);
                    }
                    if let Ok(drawn) = data::drawn_network_from_entities(entities(host)) {
                        host.push_undo("SS_STYLE");
                        let (sur, flood) =
                            style::apply_analysis_style(entities_mut(host), &drawn, &analysis);
                        if sur > 0 || flood > 0 {
                            host.set_dirty();
                            host.push_info(&format!(
                                "Styled {sur} surcharged pipe(s), {flood} flooded structure(s)."
                            ));
                        }
                    }
                    host.bump_geometry();
                    host.push_info(&format!("Storm sewer analyzed ({}).", params.summary()));
                    for line in report.lines() {
                        host.push_output(line);
                    }
                }
                Err(e) => host.push_error(&e),
            }
            true
        }
        "SS_REPORT" => {
            let params = tab_params(host);
            match analysis::report_doc(entities(host), &params) {
                Ok(report) => {
                    for line in report.lines() {
                        host.push_output(line);
                    }
                }
                Err(e) => host.push_error(&e),
            }
            true
        }
        "SS_MULTIRP" => {
            let params = tab_params(host);
            match analysis::multi_rp_report(entities(host), &params) {
                Ok(report) => {
                    for line in report.lines() {
                        host.push_output(line);
                    }
                }
                Err(e) => host.push_error(&e),
            }
            true
        }
        "SS_PROFILE" => {
            let params = tab_params(host);
            match analysis::profile_doc(entities(host), &params) {
                Ok(ents) => {
                    for e in ents {
                        let _ = host.add_entity(e);
                    }
                    host.bump_geometry();
                    host.push_info("Storm sewer HGL profile drawn.");
                }
                Err(e) => host.push_error(&e),
            }
            true
        }
        "SS_SIZE" => {
            let params = tab_params(host);
            match sizing::plan_size_updates(entities(host), &params) {
                Ok((updates, report, pending)) => {
                    for line in report.lines() {
                        host.push_output(line);
                    }
                    if pending == 0 {
                        host.push_info("Storm sewer: all pipes already meet sizing criteria.");
                    } else {
                        host.push_undo("SS_SIZE");
                        let applied = sizing::apply_updates(entities_mut(host), &updates);
                        host.bump_geometry();
                        host.set_dirty();
                        host.push_info(&format!(
                            "Storm sewer: applied {applied} pipe diameter update(s)."
                        ));
                    }
                }
                Err(e) => host.push_error(&e),
            }
            true
        }
        "SS_APPLYTC" => {
            host.push_undo("SS_APPLYTC");
            let tc_by_handle: HashMap<Handle, f64> =
                match data::drawn_network_from_entities(entities(host)) {
                    Ok(drawn) => drawn
                        .network
                        .nodes
                        .iter()
                        .zip(drawn.node_handles.iter())
                        .filter(|(node, _)| node.kind != NodeKind::Outfall)
                        .map(|(node, &h)| (h, node.tc_inlet))
                        .collect(),
                    Err(e) => {
                        host.push_error(&e);
                        HashMap::new()
                    }
                };
            let updated = data::apply_tc_map(entities_mut(host), &tc_by_handle);
            if updated > 0 || !tc_by_handle.is_empty() {
                host.set_dirty();
                host.bump_geometry();
                host.push_info(&format!(
                    "Storm sewer: updated inlet Tc on {updated} structure(s)."
                ));
            }
            true
        }
        cmd if cmd == "SS_PARAMS" || cmd.starts_with("SS_PARAMS ") => {
            let rest = cmd.trim_start_matches("SS_PARAMS").trim();
            let state = ensure_plugin_state(host, PLUGIN_ID, StormTabState::default);
            match params_cmd::apply_params(state, rest) {
                Ok(msg) => host.push_info(&msg),
                Err(e) => host.push_error(&e),
            }
            true
        }
        cmd if cmd == "SS_LANDXML" || cmd == "SS_IMPORTXML" => {
            host.push_info(
                "Use the Import LandXML ribbon tool, or run SS_IMPORTXML <path-to-file>.",
            );
            true
        }
        cmd if cmd.starts_with("SS_LANDXML ") || cmd.starts_with("SS_IMPORTXML ") => {
            let Some(path) = command_arg(cmd) else {
                host.push_error("Expected: SS_IMPORTXML <path-to-landxml-file>");
                return true;
            };
            match std::fs::read_to_string(path) {
                Ok(xml) => match landxml_import::import_landxml(host, &xml) {
                    Ok(msg) => host.push_info(&msg),
                    Err(e) => host.push_error(&e),
                },
                Err(e) => host.push_error(&format!("cannot read {path}: {e}")),
            }
            true
        }
        "SS_INLET" => {
            host.start_interactive(Box::new(PlaceStructureInteractive::inlet()));
            true
        }
        cmd if cmd.starts_with("SS_INLET ") => {
            match placement::place_structure(host, NodeKind::Inlet, command_arg(cmd).unwrap_or("")) {
                Ok(msg) => host.push_info(&msg),
                Err(e) => host.push_error(&e),
            }
            true
        }
        "SS_JUNCTION" => {
            host.start_interactive(Box::new(PlaceStructureInteractive::junction()));
            true
        }
        cmd if cmd.starts_with("SS_JUNCTION ") => {
            match placement::place_structure(host, NodeKind::Junction, command_arg(cmd).unwrap_or("")) {
                Ok(msg) => host.push_info(&msg),
                Err(e) => host.push_error(&e),
            }
            true
        }
        "SS_OUTFALL" => {
            host.start_interactive(Box::new(PlaceStructureInteractive::outfall()));
            true
        }
        cmd if cmd.starts_with("SS_OUTFALL ") => {
            match placement::place_structure(host, NodeKind::Outfall, command_arg(cmd).unwrap_or("")) {
                Ok(msg) => host.push_info(&msg),
                Err(e) => host.push_error(&e),
            }
            true
        }
        "SS_PIPE" => {
            host.start_interactive(Box::new(PlacePipeInteractive::new()));
            true
        }
        cmd if cmd.starts_with("SS_PIPE ") => {
            match placement::place_pipe(host, command_arg(cmd).unwrap_or("")) {
                Ok(msg) => host.push_info(&msg),
                Err(e) => host.push_error(&e),
            }
            true
        }
        "SS_EDIT" => {
            host.push_info(edit::usage());
            true
        }
        cmd if cmd.starts_with("SS_EDIT ") => {
            match edit::edit_entity(host, command_arg(cmd).unwrap_or("")) {
                Ok(msg) => host.push_info(&msg),
                Err(e) => host.push_error(&e),
            }
            true
        }
        "SS_CATCHMENT" => {
            host.push_info(
                "SS_CATCHMENT interactive polyline tagging is not in v0.2 yet. \
                 Tag catchments with STORMSEWER_CATCHMENT XDATA on closed polylines, or import via LandXML.",
            );
            true
        }
        _ => false,
    }
}