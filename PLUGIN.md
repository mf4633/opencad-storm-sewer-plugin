# Storm Sewer (`opencad.storm_sewer`)

External add-on for gravity storm-drain network design and analysis in [Open CAD Studio](https://github.com/HakanSeven12/OpenCADStudio).

## Commands

| Command | Since | Description |
|---------|-------|-------------|
| `SS_INLET <x>,<y> [invert] [rim] [area] [C]` | 0.2 | Place inlet structure |
| `SS_JUNCTION <x>,<y> [...]` | 0.2 | Place junction |
| `SS_OUTFALL <x>,<y> [invert] [rim]` | 0.2 | Place outfall |
| `SS_PIPE <from> <to> [dia] [n]` | 0.2 | Pipe by structure handles |
| `SS_PIPE <x1>,<y1> <x2>,<y2> [dia] [n]` | 0.2 | Pipe snapping to nearest structures |
| `SS_EDIT <handle> <field> <value> [...]` | 0.2 | Edit structure or pipe XDATA |
| `SS_VALIDATE` | 0.2 | Integrity + design-criteria review (see below) |
| `SS_ANALYZE` | 0.1 | Run analysis (+ surcharge/flood styling) |
| `SS_REPORT` / `SS_PROFILE` / `SS_SIZE` | 0.1 | Report, profile, sizing |
| `SS_PARAMS` / `SS_MULTIRP` / `SS_APPLYTC` | 0.1 | Parameters, multi-RP, Tc apply |
| `SS_IMPORTXML <path>` | 0.1 | LandXML import (ribbon file dialog too) |
| `SS_CATCHMENT` | — | Manual XDATA / LandXML (interactive polyline pick planned) |

### Example workflow (v0.2, no interactive pick)

```
SS_INLET 0,0 104 110 1.0 0.7
SS_OUTFALL 200,0 100 106
SS_PIPE 1 2 1.5 0.013
SS_VALIDATE
SS_ANALYZE
SS_EDIT 1 invert 103.5
```

## `SS_VALIDATE` checks

Two passes, reported as warnings (info) and errors:

**Integrity** — rim ≤ invert, zero contributing area, runoff C out of range,
pipe diameter ≤ 0 / Manning n ≤ 0, dangling pipe handles, incomplete/malformed
XDATA, no structures, structures-without-pipes.

**Design criteria** (on the analyzed network, default municipal thresholds):

| Check | Default | Severity |
|-------|---------|----------|
| Adverse (uphill) slope | slope < 0 | error |
| Suspiciously flat slope | slope < 0.0005 ft/ft | warning |
| Surcharge | design Q > open-channel capacity | error |
| Near capacity | design Q > 85% of full | warning |
| Self-cleansing velocity | V < 2.0 ft/s | warning |
| Scour velocity | V > 10.0 ft/s | warning |
| Minimum cover | rim − (invert + diameter) < 1.0 ft | warning |
| Pipe size reduces downstream | downstream Ø < upstream Ø at a node | warning |
| Surface flooding | HGL above rim | error |

Thresholds live in `stormsewer::design::ReviewCriteria`. The design pass is
best-effort: if the network can't be built/analyzed, the integrity pass already
reports why.

## XDATA schemas

### `STORMSEWER_STRUCT` (on `CIRCLE`)

| Index | Field | Type |
|-------|-------|------|
| 0 | kind | string (`inlet` / `junction` / `outfall`) |
| 1 | invert | real |
| 2 | rim | real |
| 3 | area | real (acres) |
| 4 | C | real |
| 5 | tc | real (minutes) |

### `STORMSEWER_PIPE` (on `LINE`)

| Index | Field | Type |
|-------|-------|------|
| 0 | diameter | real (inches) |
| 1 | n | real |
| 2 | from_handle | handle |
| 3 | to_handle | handle |

### `STORMSEWER_CATCHMENT` (on closed `LWPOLYLINE`)

| Index | Field | Type |
|-------|-------|------|
| 0 | C | real |
| 1 | length_ft | real |
| 2 | slope | real |
| 3 | inlet_handle | handle (0 = auto) |