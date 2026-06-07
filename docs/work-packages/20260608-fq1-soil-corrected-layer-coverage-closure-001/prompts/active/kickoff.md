# FQ1 Kickoff — soil corrected-layer coverage closure (HS-RUNTIME-E-062)

Execution mode: package-end-to-end

Autonomy: execute end-to-end for the declared scope — Milestone-1 localization +
ownership, contract amendment, contract-derived red/green tests, pre-implementation
gate, production correction, 43-of-43 validation, dual review/verification,
disposition, and a defect-shaped handoff — without asking for direction on
intermediate diagnostic steps. Ask only if hard-blocked.

## Item 1 — close defect `FQ1-HS-RUNTIME-E-062-SOIL-CORRECTED-LAYER-COVERAGE`

`37/43` single-OFE hillslopes on `/wc1/runs/al/algebraic-radium` fail closed with
`HS-RUNTIME-E-062` (`CorrectedLayerMappingIncomplete`, surfaced as `CLIHILL-E-011`)
before hydrology — e.g. `p1`: "layer 6 corrected-lineage mapping coverage
incomplete (1100..2000 mm, covered 700 mm)". Make openWEPP's corrected soil-layer
mapping cover the full profile for valid SURGO/disturbed soils so all 43 single-OFE
hillslopes produce `H.wat.parquet`+`H.hbp`, or reclassify a genuinely-invalid soil
as typed invalid input with evidence. Primary surface:
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
(`compute_normalized_corrected_layer_runtime_symbols_from_legacy_seed`,
`map_corrected_layer_runtime_symbols_to_parser_layers`, `:246`), error type in
`00_core_types.rs`.

## Milestone 1 first (symptom-existence + ownership)

Reproduce a blocked prefix; capture parser layer depths, the normalized
corrected-layer increments, and exactly where the `lo..2000 mm` gap arises; name the
mechanism (wrong 2000 mm target vs missing bottom-layer extension vs
restricting-layer handling vs parser layer-depth). **Ownership check:** run
`/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill` on the blocked soils — if
legacy runs them and openWEPP does not, it is an openWEPP defect to fix; if legacy
also fails on a soil, reclassify that soil as invalid input with evidence.

## Conversion rule + acceptance authority

- If root cause is reproduced inside the soil corrected-layer envelope and the
  corrected behavior is `SC-SOIL-001`/legacy-soil-provenance backed, you MUST land
  the contract-first fix (contract → tests → pre-impl gate → code → validate →
  disposition). Do not close HOLD because more investigation is possible.
- Authority is `SC-SOIL-001` + legacy soil-profile lineage, NOT comparator match
  (`wepp_260606_hill` is a flag, ADR-0017).
- Fail closed on genuinely-invalid soils; **do not loosen the coverage guard** to
  silently accept incomplete coverage.

## Hard constraints (protected boundaries)

- Soil parse/runtime mapping only — **no frost, ET, runoff, snow, or
  hydrology-kernel changes** (FQ-3/FQ-4 own those).
- No downstream WB compensation; no silent defaults/clamping.

## Acceptance

- 43/43 single-OFE produce WAT (or invalid-with-evidence); the 6 currently-runnable
  prefixes (`p8,p13,p22,p23,p26,p28`) do not regress (corrected FC/WP symbols
  unchanged within tolerance).
- Red/green contract-derived tests over the blocked-soil shapes + the 6-runnable
  control; `cargo test --workspace`, clippy `-D warnings`, `cargo deny` pass.
- Truthful evidence mode (**Ran** for actual runs).

## Required reading

- `docs/work-packages/20260608-fq1-soil-corrected-layer-coverage-closure-001/package.md`
- `docs/defect_closure_execplans.md`, `docs/decisions/0011/0017/0018`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `AGENTS.md`, `docs/codex_exec_plans.md`
- Blocked taxonomy: `/tmp/frostval01/full/run_status.tsv`
- Soil code: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`,
  `00_core_types.rs`; soil parser under `crates/openwepp-input-contract/src/parsers/`.
- Comparator: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`.
