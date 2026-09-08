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
| `SS_REPORT` / `SS_PROFILE` | 0.1 | Text report to the command line; HGL profile on the drawing |
| `SS_REPORT_HTML` **(Pro)** | 0.2.3 | KaTeX HTML report to `Documents\StormSewer\` |
| `SS_SIZE` **(Pro)** | 0.1 | Auto-size pipes to capacity/velocity criteria |
| `SS_MULTIRP` **(Pro)** | 0.1 | Multi-return-period table |
| `SS_PARAMS` / `SS_APPLYTC` | 0.1 | Parameters, Tc apply |
| `SS_LICENSE` | 0.3 | Show license status, file path and what Pro unlocks |
| `SS_ACTIVATE <email> <hc_live_ss_…>` | 0.3 | Activate a Pro key (online validation against hydrocomplete.com) |
| `SS_IMPORTXML <path>` | 0.1 | LandXML import (ribbon file dialog too) |
| `SS_CATCHMENT` | — | Manual XDATA / LandXML (interactive polyline pick planned) |

## Free vs Pro

Drawing, LandXML import, `SS_ANALYZE`, `SS_REPORT`, `SS_PROFILE` and `SS_VALIDATE` are free. **Pro ($29/year, one seat)** unlocks the deliverables: `SS_REPORT_HTML`, `SS_SIZE` and `SS_MULTIRP`. Buy a key at <https://hydrocomplete.com/stormsewer>, then:

```
SS_ACTIVATE you@firm.com hc_live_ss_…
SS_LICENSE
```

The key is validated online (`POST /api/licensing/validate`, product `stormsewer`) and cached in `%APPDATA%\HydroComplete\stormsewer-license.json` (`$HOME/HydroComplete/` elsewhere) with its expiry, so later sessions do not need the network. Keys for the HydroComplete Open CAD Studio plugin (`opencad`) or Civil 3D (`civil3d`) are separate SKUs and will not activate this plugin. Running a Pro command without a key prints the purchase URL and the free alternative instead of failing silently.

Every 7 days the plugin silently re-checks the key with the server the next time a Pro command runs (4 s timeout). A rejected key (revoked, expired, refunded) deletes the cached license immediately; if the server is unreachable the cached license keeps working for 30 days after its last successful check, then `SS_LICENSE` reports it and `SS_ACTIVATE` is required again. Network attempts are throttled to one per day while offline.

Debug builds honour `STORMSEWER_PRO=1` as a developer bypass; release builds ignore it.

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