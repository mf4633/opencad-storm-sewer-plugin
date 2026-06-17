# Storm Sewer (`opencad.storm_sewer`)

External add-on for gravity storm-drain network design and analysis in [Open CAD Studio](https://github.com/HakanSeven12/OpenCADStudio).

- **Engine:** `crates/stormsewer` (headless hydraulics)
- **Host contract:** `ocs_plugin_api` (`HostApi`, `BuiltinPlugin`, `export_plugin!`)

## Commands

| Command | v0.1.0 | Description |
|---------|--------|-------------|
| `SS_ANALYZE` | yes | Run hydraulic analysis on drawn network |
| `SS_REPORT` | yes | Print analysis report |
| `SS_PROFILE` | yes | Draw HGL profile |
| `SS_SIZE` | yes | Size pipes from analysis |
| `SS_PARAMS` | yes | Set rainfall / analysis parameters |
| `SS_MULTIRP` | yes | Multi return-period analysis |
| `SS_APPLYTC` | yes | Apply time-of-concentration from catchments |
| `SS_IMPORTXML <path>` | yes | Import LandXML storm network |
| `SS_INLET` / `SS_JUNCTION` / `SS_OUTFALL` / `SS_PIPE` / `SS_CATCHMENT` | pending | Interactive placement (needs `HostApi` hook) |

LandXML import also available via the **Import LandXML** ribbon tool (native file dialog).

## XDATA schemas

Domain data lives on DWG entities so networks round-trip through save/load.

### `STORMSEWER_STRUCT` (on `CIRCLE`)

| Index | Field | Type | Notes |
|-------|-------|------|-------|
| 0 | kind | string | `inlet`, `junction`, `outfall` |
| 1 | invert | real | Structure invert elevation |
| 2 | rim | real | Rim elevation |
| 3 | area | real | Contributing area (acres) |
| 4 | C | real | Runoff coefficient |
| 5 | tc | real | Time of concentration (minutes) |

### `STORMSEWER_PIPE` (on `LINE`)

| Index | Field | Type | Notes |
|-------|-------|------|-------|
| 0 | diameter | real | Pipe diameter (inches) |
| 1 | n | real | Manning's n |
| 2 | from_handle | handle | Start structure |
| 3 | to_handle | handle | End structure |

### `STORMSEWER_CATCHMENT` (on `LWPOLYLINE`)

| Index | Field | Type | Notes |
|-------|-------|------|-------|
| 0 | C | real | Runoff coefficient |
| 1 | length_ft | real | Flow path length |
| 2 | slope | real | Average slope |
| 3 | inlet_handle | handle | Inlet structure (optional) |

## Per-document state

Keyed under plugin id `opencad.storm_sewer` as `StormTabState`:

- `StormAnalysisParams` — IDF, tailwater, min Tc, return periods, etc.