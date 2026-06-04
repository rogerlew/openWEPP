# Implementation Test Evidence

Status: completed/HOLD
Evidence mode: mixed static-and-ran

Static: HPHYS0277 implemented a typed physical hourly radiation guard in the
SIMIMPL28 hourly forcing publication path.

Ran: focused gates, full H1..H39 diagnostics, and workspace validation were
executed locally. Workspace-wide tests remain HOLD for a known SIMIMPL18/WB11
ET domain violation outside this package.

## Implementation Summary

- `simimpl28_hourly_extraterrestrial_radiation_upper_bound(sdate)` derives the
  allowed hourly upper bound from baseline `radcur.for` solar constant and
  Earth-sun distance lineage.
- `simimpl28_hr_tmp_hour` now rejects finite hourly radiation below zero or
  above the derived physical bound with
  `RuntimeContextSymbolOutOfRange`.
- The guard applies before boundary publication and does not clip, cap,
  renormalize, substitute, or compensate downstream values.
- The bound tolerance is explicit and limited to roundoff:
  relative `1.0e-9` plus absolute `1.0e-12 MJ m^-2`.

## Focused Gates

Ran:

- `cargo fmt --check`
- `tools/release/check_raw_unit_conversions.sh`
- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context --lib`
- `cargo test -p openwepp-hillslope-orchestrator runtime_inputs`
- `cargo test --test clim05_snow_runtime_kernel_contract`
- `cargo test --test hphys0275_boundary_value_dimensional_typing_contract`
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo deny check`
- `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md --path docs/specifications/science-contracts/index.md --path docs/specifications/units/boundary-symbol-unit-registry.md --path docs/work-packages/20260603-hphys0277-climate-radiation-physical-flux-guard-001 --path docs/work-packages/README.md`
- `git diff --check`

Result: passed, except `cargo deny check` reported existing duplicate/unmatched
license warnings while returning success.

## H1/H7/H39 and Full 39 Diagnostics

Ran:

`.venv/bin/python docs/work-packages/20260603-hphys0272-hourly-radiation-unit-closure-001/artifacts/hphys0272_diagnostics.py --run-root /tmp/openwepp-hphys0277-radiation-guard`

Result: completed with `rc=0`.

- Targeted H1/H7/H39 valid traces did not trip the guard.
- Full H1..H39 valid traces did not trip the guard.
- Semantic parity remains diagnostic `0/39`, matching prior snowpack/ET/storage
  residual HOLD posture.

Detailed summaries are recorded in:

- `targeted-h1-h7-h39-radiation-guard-metrics.md`
- `full-39-suite-metrics.md`
- `targeted-trace-status.tsv`
- `hillslope-batch-status.tsv`
- `semantic-status.tsv`

## Workspace HOLD

Ran:

`cargo test --workspace`

Result: failed in
`tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
with known `HKERNEL-WB11-ET-E-003` / `DOMAIN_VIOLATION` SIMIMPL18 failures.
The failure is outside the HPHYS0277 write set and did not involve the new
hourly radiation guard.
