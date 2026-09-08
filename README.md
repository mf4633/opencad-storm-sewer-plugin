# opencad-storm-sewer-plugin

Storm Sewer hydraulics add-on for [Open CAD Studio](https://github.com/HakanSeven12/OpenCADStudio), distributed as a prebuilt dynamic library via GitHub Releases.

Depends only on [`ocs_plugin_api`](https://github.com/HakanSeven12/OpenCADStudio/tree/main/crates/ocs_plugin_api) (API **v5**, built for OCS **v2026.36**) and the in-repo [`stormsewer`](crates/stormsewer) engine crate.

## Status (v0.3.2)

| Area | Status |
|------|--------|
| Engine (`stormsewer`) | Rational, Manning, HGL, LandXML, `.ssn` |
| Analysis | `SS_ANALYZE` (+ surcharge/flood colors), report, profile — free |
| Pro ($29/yr) | **HTML/KaTeX report** (`SS_REPORT_HTML`), pipe sizing (`SS_SIZE`), multi-RP (`SS_MULTIRP`) — `SS_ACTIVATE` with a key from [hydrocomplete.com/stormsewer](https://hydrocomplete.com/stormsewer) |
| Interactive placement | `SS_INLET` / `SS_JUNCTION` / `SS_OUTFALL` (click) + `SS_PIPE` (two structure picks) via `InteractiveCommand` |
| Automation placement | `SS_INLET 100,200 …`, `SS_PIPE 1 2`, coordinate/handle forms for `--serve` |
| Edit / validate | `SS_EDIT`; `SS_VALIDATE` integrity + design-criteria review (velocity, cover, slope, capacity, size progression, flooding) |
| HEC-22 inlets | Grate, curb opening, combination, sag — `SS_PARAMS INLETKIND` |
| Import | LandXML via ribbon file dialog or `SS_IMPORTXML <path>` |
| Catchment tagging | Manual XDATA or LandXML — interactive polyline pick pending richer `HostApi` |

See [PLUGIN.md](PLUGIN.md) for syntax and XDATA schemas.

## Repo layout

```
opencad-storm-sewer-plugin/
├── Cargo.toml              # cdylib plugin crate
├── plugin.toml             # Plugin Manager metadata (sync with MANIFEST in lib.rs)
├── crates/stormsewer/      # headless engine (std-only, WASM-capable)
├── src/
│   ├── lib.rs              # BuiltinPlugin + ribbon
│   ├── dispatch.rs         # SS_* command routing
│   ├── interactive.rs      # InteractiveCommand (viewport + --serve picks)
│   ├── placement.rs        # coordinate/handle placement for automation
│   ├── data.rs             # XDATA schemas + network reconstruction
│   ├── analysis.rs         # engine bridge
│   ├── license.rs          # Pro key activation + cached license file
│   ├── license_cmd.rs      # SS_LICENSE / SS_ACTIVATE text + Pro gate list
│   └── …
├── examples/automate_analyze.py
└── .github/workflows/release.yml
```

## Install (from Open CAD Studio)

**Plugin Manager → marketplace → Storm Sewer → Install**, restart OCS.

Requires Open CAD Studio **v2026.36** (plugin API 5). The host refuses a plugin whose rustc/acadrust fingerprint differs from its own, so each plugin release is built for one OCS release; older releases (v0.2.x, API 2) load on the current host but fail on the first command.

## Build

```bash
cargo build --release
```

Produces `opencad_storm_sewer_plugin.dll` (Windows) / `libopencad_storm_sewer_plugin.so` (Linux) / `libopencad_storm_sewer_plugin.dylib` (macOS). Ship beside `plugin.toml`.

## Release

Tag `v0.2.1` (or later) — CI attaches per-platform binaries + `plugin.toml` to the GitHub Release for Plugin Manager.

## XDATA contract

Domain data lives on DWG entities (round-trips through DXF/DWG):

- `STORMSEWER_STRUCT` — structure markers (inlet / junction / outfall)
- `STORMSEWER_PIPE` — pipe connectivity and hydraulics
- `STORMSEWER_CATCHMENT` — catchment tagging

## Related

- Extensibility epic: [OpenCADStudio#100](https://github.com/HakanSeven12/OpenCADStudio/issues/100)
- Reference plugin: [opencad-example-plugin](https://github.com/HakanSeven12/opencad-example-plugin)
- Source fork history: [mf4633/OpenCADStudio `feature/storm-sewer-module`](https://github.com/mf4633/OpenCADStudio/tree/feature/storm-sewer-module)

## License

GPL-3.0-only