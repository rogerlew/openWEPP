# INIMPL17 Merge Conflict Log

Evidence mode: `Ran` + `Static`

## Summary

Wave 2 integration cherry-picks were executed in canonical order after intake
readiness was confirmed. Two content conflicts occurred and were resolved.

## Conflict Entries

| timestamp_utc | worker | worker_commit | integration_commit | file | conflict_class | resolution | status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 2026-05-21T19:40:00Z (approx) | `INIMPL13` | `5b9a578` | `825a5fd` | `crates/openwepp-input-contract/src/parsers/mod.rs` | shared-export ordering/content conflict | Removed conflict markers; preserved existing `irrigation_depletion` export and added `irrigation_fixeddate` export. | resolved |
| 2026-05-21T19:40:00Z (approx) | `INIMPL14` | `dcf8784` | `125c264` | `crates/openwepp-input-contract/src/parsers/mod.rs` | shared-export ordering/content conflict | Removed conflict markers; preserved existing exports and added `frost` export. | resolved |

## Non-Conflict Integrations

| order | worker | worker_commit | integration_commit | result |
| --- | --- | --- | --- | --- |
| 1 | `INIMPL11` | `47c27bc` | `d171b45` | clean cherry-pick |
| 2 | `INIMPL12` | `ab650c3` | `ac5ab46` | clean cherry-pick |
| 5 | `INIMPL15` | `977c3d4` | `6c38613` | clean cherry-pick |
| 6 | `INIMPL16` | `2e63b42` | `ec34cde` | clean cherry-pick |

## Post-Resolution State

- Final `parsers/mod.rs` export list includes:
  - `climate`, `frost`, `irrigation_depletion`, `irrigation_fixeddate`,
    `management`, `pmetpara`, `slope`, `snow`, `soil`, `wepp_ui`.
- Formatting normalized via `rustfmt` and verified by `cargo fmt --check`.
