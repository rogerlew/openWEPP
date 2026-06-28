# Validation Log

Evidence mode: Static/Ran.

## Focused Build And Diagnostic

- Ran: `cargo build -p openwepp-runner --bin openwepp-cli-hill`
  - Result: PASS.
- Ran: `.venv/bin/python tools/snowfreeze_observed/open_surface_ablation_stage_a.py`
  - Result: PASS as diagnostic execution; package disposition is
    `NON-PROMOTION-STAGE-A-GATE-NOT-MET`.
  - Artifact:
    `docs/work-packages/20260627-snowdensity-10-3-16-open-surface-ablation-stage-a-001/artifacts/open-surface-ablation-stage-a.{md,json}`.
- Ran: `cargo test --test snowdensity10_3_16_open_surface_ablation_stage_a`
  - Result: PASS, 3 passed.

## Package Gates

| Gate | Evidence | Status |
|---|---|---|
| Contract-first amendment | `SC-SNOWFREEZE-001` v102, `INV-SNOWFREEZE-073`, `OBL-SNOWFREEZE-P-048` | PASS |
| Candidate reaches real snow partition | Trace model counts for `coe_open_sublimation_stage_a_v1`: `30317` rows across the two open surfaces | PASS |
| Reduce open cap-limited tail | `30 -> 27` | PASS |
| Do not worsen under-persistence | `54 -> 57` | FAIL |
| Literature-range magnitude | total trace sublimation `0.586351 m`; max daily-lane `0.004834 m`, below provisional envelope | PASS |
| Snow-state conservation | max closure residual `5.551e-17 m` | PASS |
| Vapor not routed as liquid | diagnostic `routed_melt_minus_rain_released_minus_snowpack_loss` unchanged within tolerance; unit test asserts routed liquid excludes sublimation | PASS |
| Protected boundaries | diagnostic flags default/cap/schema/fixtures/parser/user/Qwet/frost unchanged | PASS |
| PySnobal C licensing | no local non-GPL-family license metadata found; C source not read | PASS |

## Repository Gates

- Ran: `cargo fmt --check`
  - Result: PASS.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings`
  - Initial result: FAIL, `clippy::too_many_lines` in
    `support_helpers_mod/runoff_reconciliation.rs`.
  - Disposition: split inactive snow coupling validation into
    `inactive_snow_coupling_from_typed`; no suppression.
  - Rerun result: PASS.
- Ran: `cargo test --workspace`
  - Result before clippy refactor: PASS.
  - Final post-refactor rerun: PASS.
- Ran: `cargo deny check`
  - Result: PASS (`advisories ok, bans ok, licenses ok, sources ok`).

## Source Scans

- Ran: `rg -n "qwet|frzftp" crates || true`
  - Result: no production crate hits.
- Ran: `find /workdir/pysnobal -maxdepth 3 ...` and metadata `rg`.
  - Result: only `README.md`, `pyproject.toml`, and `setup.py` metadata files
    found; no local license declaration found in those metadata files.
