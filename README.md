# opencad-storm-sewer-plugin

Storm Sewer hydraulics add-on for [Open CAD Studio](https://github.com/HakanSeven12/OpenCADStudio), distributed as a prebuilt dynamic library via GitHub Releases.

Depends only on [`ocs_plugin_api`](https://github.com/HakanSeven12/OpenCADStudio/tree/main/crates/ocs_plugin_api) (the host's stable contract crate) and the in-repo [`stormsewer`](crates/stormsewer) engine crate.

## Status (v0.2.0)

| Area | Status |
|------|--------|
| Engine (`stormsewer`) | Rational, Manning, HGL, LandXML, `.ssn` |
| Analysis | `SS_ANALYZE` (+ surcharge/flood colors), report, profile, sizing, multi-RP |
| Drafting (coordinate) | `SS_INLET`, `SS_JUNCTION`, `SS_OUTFALL`, `SS_PIPE`, `SS_EDIT` |
| Validation | `SS_VALIDATE` + warnings on `SS_ANALYZE` |
| Import | LandXML via ribbon or `SS_IMPORTXML <path>` |
| Interactive pick placement | Pending `HostApi` hook ([#100](https://github.com/HakanSeven12/OpenCADStudio/issues/100#issuecomment-4733946258)) |

See [PLUGIN.md](PLUGIN.md) for syntax and XDATA schemas.

## Install (from Open CAD Studio)

**Plugin Manager → Add repository →** `mf4633/opencad-storm-sewer-plugin`, pick a compatible release, **Install**, restart OCS.

Or install from a built checkout into `%APPDATA%/OpenCADStudio/plugins/opencad.storm_sewer/`.

## Build

```bash
cargo build --release
```

Produces `opencad_storm_sewer_plugin.dll` (Windows) / `libopencad_storm_sewer_plugin.so` (Linux) / `libopencad_storm_sewer_plugin.dylib` (macOS). Ship beside `plugin.toml`.

## XDATA contract

Domain data lives on DWG entities (round-trips through DXF/DWG):

- `STORMSEWER_STRUCT` — structure markers (inlet / junction / outfall)
- `STORMSEWER_PIPE` — pipe connectivity and hydraulics
- `STORMSEWER_CATCHMENT` — catchment tagging

## Related

- Upstream discussion: [OpenCADStudio#100](https://github.com/HakanSeven12/OpenCADStudio/issues/100)
- Reference plugin: [opencad-example-plugin](https://github.com/HakanSeven12/opencad-example-plugin)
- Source fork: [mf4633/OpenCADStudio `feature/storm-sewer-module`](https://github.com/mf4633/OpenCADStudio/tree/feature/storm-sewer-module)

## License

GPL-3.0-only — see engine crate headers.