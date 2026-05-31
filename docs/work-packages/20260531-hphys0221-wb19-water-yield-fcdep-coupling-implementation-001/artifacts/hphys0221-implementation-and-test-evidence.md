# HPHYS0221 Implementation and Test Evidence

Status: completed
Evidence mode: Static + Ran

## Scope
- Implement WB19 coupling in production kernel and supporting helpers.
- Update canonical contract authority and contract-derived tests.
- Execute rerun and full workspace gates.

## Production implementation completed
- Added WB19 symbol constants:
  - `wb19_fcdep`
  - `wb19_unsdep`
  - `wb19_watyld`
- Added helper symbol loaders/validation for:
  - `solwpv`
  - `por_####`
  - `wb18_perc_ul_####`
- Updated `run_lateral_transfer` to:
  - apply `solwpv` branch semantics (`2006` vs non-`2006` saturated-layer selection),
  - compute `avpora`, `avfca`, `avcoca`, `watyld`,
  - publish coupled WB19 state outputs,
  - apply non-`2006` `fcdep/unsdep` update with typed hard-fail when
    `watyld <= 0`.

## Ran test and gate evidence
- Targeted kernel/contract tests: pass.
- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass (warning-only duplicates/unmatched-allow list items).

## Rerun evidence
- Run root: `/tmp/hphys0221_20260531T141839Z/parity`
- Status:
  - hillslope batch `39/39` success
  - semantic compare `39/39` success
- Summary file:
  - `/tmp/hphys0221_20260531T141839Z/parity/reports/hillslope_semantic_summary.json`

## Result
- `MEASURE-HP221-003`: pass.
