# Worker Handoff

Status: READY. Evidence mode: Static + Ran.

## Next Package

Suggested package id:

- `20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001`

## Objective

Resolve the active row-crop `canhgt` runtime publication/source-authority
blocker exposed by `mn_corn_h4` in the selected-cohort suite.

## Required Starting Evidence

- This package:
  - `artifacts/hold-legitimacy-audit.md`
  - `artifacts/active-suite-run-logs/mn_corn_h4-plain.time.log`
  - `artifacts/selected-cohort-runs/mn_corn_h4/p4.man`
- Source locations:
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`
- Authority:
  - `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
  - `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
  - `docs/contracts/openwepp-management-lanuse-authority-contract.md`

## Constraints

- Do not default `canhgt` to `hmax`, `0`, or an arbitrary positive value.
- Do not relax the Rev-21 fail-closed guard.
- Do not treat H2637-only timing as selected-cohort closure.
- Any implementation must be source-authorized and covered by row-crop
  LAI-positive active Lane D tests.

## Acceptance Sketch

1. Bind the authoritative annual-crop canopy-height source/projection.
2. Prove `mn_corn_h4` active plain reaches publication without weakening the
   guard.
3. Run `mn_corn_h4` active hybrid.
4. Rerun all four selected members plain/hybrid with
   `artifacts/run_active_suite.py`.
5. Re-run summary and D16 promotion/tolerance adjudication.
