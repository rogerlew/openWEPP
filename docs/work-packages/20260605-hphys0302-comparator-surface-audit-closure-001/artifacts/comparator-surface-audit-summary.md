# HPHYS0302 Comparator-Surface Audit Summary

Status: executed
Evidence mode: Ran

## Result

- Production edit authorized: `false`.
- `RM` passes as a like-for-like daily WB13/WAT publication surface.
- `Snow-Water` passes as a daily output surface, not producer authority.
- Raw `hrmlt` and post-raw `wmelt` pass only as aggregate cut-point surfaces.
- Term-level melt correction is blocked because paired baseline `amelt`/`bmelt`/`cmelt`/`dmelt` term/state surfaces are absent.

## Counts

- Windows: `9`.
- Surface rows: `45`.
- RM like-for-like rows: `9`.
- Snow-Water output-surface rows: `9`.
- Aggregate cut-point rows: `18`.
- Blocked melt-term rows: `9`.

## Surface Summary

| Surface | Verdict | Rows | Max abs residual mm |
|---|---|---:|---:|
| `RM` | `like-for-like-pass` | 9 | 65.755222 |
| `Snow-Water` | `output-surface-pass` | 9 | 592.358693 |
| `raw_hrmlt` | `aggregate-like-for-like-pass-not-term-authority` | 9 | 48.07294 |
| `post_raw_wmelt` | `aggregate-like-for-like-pass-not-term-authority` | 9 | 69.7689 |
| `melt_terms` | `blocked-missing-baseline-term-surface` | 9 | 0.0 |
