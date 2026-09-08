# Changelog

## 0.3.1 — 2026-09-08

- **Rebuilt for Open CAD Studio v2026.36 (plugin API 5).** v0.3.0 and every
  earlier release declared API 2, which the current host still accepts but can
  no longer drive: the ribbon reply failed to decode and every command came back
  "Unknown command". Now pinned to the host's `ocs_plugin_api` tag commit,
  `acadrust` (cadcodec @5b56571) and rustc fingerprint; `plugin.toml` declares
  `api_version = 5` and `rustc_version`.
- Per-tab parameters moved out of the host's `ensure_plugin_state` (in-process
  only; panics under the out-of-process runner) into a plugin-owned map keyed by
  tab id. `SS_PARAMS`, `SS_ANALYZE`, `SS_VALIDATE`, `SS_REPORT` work again.
- Verified headless on v2026.36 via `--serve`: import → validate → analyze →
  report → profile, Pro gate blocks `SS_REPORT_HTML`/`SS_SIZE`/`SS_MULTIRP`
  without a key and a cached license unlocks them.

## 0.3.0 — 2026-09-08

- **Pro tier ($29/year).** `SS_REPORT_HTML`, `SS_SIZE` and `SS_MULTIRP` now require an
  active Pro key. Drawing, LandXML import, `SS_ANALYZE`, `SS_REPORT`, `SS_PROFILE` and
  `SS_VALIDATE` are unchanged and free.
- New commands `SS_ACTIVATE <email> <hc_live_ss_…>` and `SS_LICENSE`; new **Pro** ribbon group.
- Keys are sold at https://hydrocomplete.com/stormsewer and validated online (product
  `stormsewer`); the result is cached locally with its expiry.
- A gated command run without a key prints the purchase URL and the free alternative.
- Plugin remains GPL-3.0-only; API v2 pin unchanged.

## 0.2.3 — 2026-06-22

- KaTeX HTML reports and expanded HEC-22 inlets.
