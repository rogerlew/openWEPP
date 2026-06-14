# line count governance checklist

Status: checked through M-F-REDO2

Evidence mode: Ran

## M-F-REDO2

Ran `wc -l` over touched Rust files after the M-F-REDO2 implementation and
fixture updates.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 1592 | OK. |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | 1793 | OK. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs` | 524 | OK. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 2124 | WARN; pre-existing 2000-line warning, below 3000. |
| `crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs` | 554 | OK. |
| `tests/integration/cli03_runner_contract_derived_tests.rs` | 1345 | OK. |
| `tests/integration/mofe04_publication_contract_authority_closure_contract.rs` | 79 | OK. |

No touched Rust source file crossed the 3000-line non-exempt threshold.
M-F-REDO2 added only a small signature pass-through to
`scheduler_seed_and_runtime.rs`, but that file remains over the 2000-line
warning threshold and should be split before further scheduler lifecycle logic
is added.

## M-F-REDO-CLONE

Ran `wc -l` over touched Rust files after the M-F-REDO-CLONE implementation
and fixture updates.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/constants.rs` | 462 | OK. |
| `hydrology_phase_infiltration_evap.rs` | 1332 | OK. |
| `hydrology_phase_plant_percolation.rs` | 1313 | OK. |
| `hydrology_phase_storage_reconciliation.rs` | 216 | OK. |
| `runoff_reconciliation.rs` | 579 | OK. |
| `hphys_trace.rs` | 1082 | OK. |
| `per_ofe_internal_wb13.rs` | 506 | OK. |
| `per_ofe_state.rs` test helper | 553 | OK. |
| `publication_wb19_wb12_wb16.rs` | 483 | OK. |
| `mofe01_per_ofe_state_contract.rs` | 378 | OK. |
| `mofe04_publication_contract_authority_closure_contract.rs` | 74 | OK. |
| `wb12_reconciliation_kernel_contract.rs` | 524 | OK. |
| `wb14_infiltration_hyetograph_kernel_contract.rs` | 1252 | OK. |

No touched Rust source file crossed the 2000-line warning threshold or the
3000-line non-exempt threshold.

Global scan still reports pre-existing warnings outside this increment write
set: `coupling.rs` is 3052 lines, `clim06_frost_frozen_soil_kernel_contract.rs`
is 2743 lines, `scheduler_seed_and_runtime.rs` is 2122 lines, and
`openwepp-cli-watershed.rs` is 2012 lines.

## M-F-REDO

Ran `wc -l` over touched Rust files after the M-F-REDO implementation and
fixture updates.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 2122 | WARN; crossed 2000-line warning threshold, below 3000. |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | 1740 | OK. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 1583 | OK. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs` | 1617 | OK. |
| Other touched Rust files/tests | below 1500 | OK. |

No touched Rust source file crossed the 3000-line non-exempt threshold.
M-F-REDO2 should extract from `scheduler_seed_and_runtime.rs` before adding
more scheduler trace logic.

## M-F

Ran `wc -l` over touched Rust files after the final clippy refactor.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2012 | WARN; crossed 2000-line warning threshold, below 3000. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 1567 | OK. |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | 1726 | OK. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs` | 357 | OK. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs` | 527 | OK. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 2115 | WARN; crossed 2000-line warning threshold, below 3000. |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | 1394 | OK. |
| `crates/openwepp-summary-accumulator/src/lib.rs` | 1222 | OK. |
| `tests/integration/cli03_runner_contract_derived_tests.rs` | 1266 | OK. |
| `tests/integration/mofe01_per_ofe_state_contract.rs` | 375 | OK. |

No touched Rust source file crossed the 3000-line non-exempt threshold.
M-F-REDO should prefer extraction over further growth in
`scheduler_seed_and_runtime.rs` and `openwepp-cli-watershed.rs`.

## M-E4-REDO

Ran `wc -l` over touched Rust files.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs` | 346 | OK. |
| `crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs` | 550 | OK. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 1973 | OK; below 2000-line warning threshold. |
| `tests/integration/mofe01_per_ofe_state_contract.rs` | 345 | OK. |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | 1994 | OK; existing near-threshold watch item, not touched by M-E4-REDO. |

No touched Rust source file crossed the 2000-line warning threshold or
3000-line non-exempt threshold.

## M-E4

Ran `wc -l` over touched Rust files.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs` | 205 | OK. |
| `crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs` | 464 | OK. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 1523 | OK. |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | 1696 | OK. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 1968 | OK; below 2000-line warning threshold. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs` | 367 | OK. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/mod.rs` | 7 | OK. |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | 1994 | OK; existing near-threshold watch item, not touched by M-E4. |

No touched Rust source file crossed the 2000-line warning threshold or
3000-line non-exempt threshold.

## M-E3

Ran `wc -l` over touched Rust files.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | 1994 | OK; below 2000-line warning threshold. Split before M-E4 if this file grows. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs` | 919 | OK. |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 65 | OK. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs` | 57 | OK. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 1961 | OK; below 2000-line warning threshold. Split before M-E4 if this file grows. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 1508 | OK. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs` | 344 | OK. |

No touched Rust source file crossed the 2000-line warning threshold or
3000-line non-exempt threshold.

## M-E2

Ran `wc -l` over touched Rust files.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | 1819 | OK; below 2000-line warning threshold. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs` | 707 | OK. |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 65 | OK. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs` | 56 | OK. |

No touched Rust source file crossed the 2000-line warning threshold or
3000-line non-exempt threshold.

## M-E1

Ran `wc -l` over touched Rust/test files.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | 1239 | OK. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 1453 | OK. |
| `crates/openwepp-runner/src/hillslope/intake_lane_setup/lane_setup_helpers.rs` | 214 | OK. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs` | 331 | OK. |
| `crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs` | 291 | OK. |
| HPHYS0319/0320 authority tests | 149-231 | OK; stale exact-version pins removed narrowly. |

No touched Rust source file crossed the 2000-line warning threshold or
3000-line non-exempt threshold.

## M-E0

Ran `wc -l` over touched Rust/test/Cargo files and touched contracts/index.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `tests/integration/mofe01_per_ofe_state_contract.rs` | 301 | OK |
| `tests/integration/mofe01_inter_ofe_route_contract.rs` | 41 | OK |
| `Cargo.toml` | 616 | OK |
| `SC-RUNOFFPART-001.md` | 997 | Existing large contract authority; updated narrowly. |
| `SC-WATBAL-001.md` | 2484 | Existing large contract authority; updated narrowly. |
| `SC-SYSTEM-001.md` | 1024 | Existing large contract authority; updated narrowly. |
| `docs/specifications/science-contracts/index.md` | 67 | OK |

No touched Rust source file crossed a line-count threshold. No production Rust
source file was edited.

## M-D

M-D edited work-package artifacts only. No Rust source line-count governance
was triggered.

`mofe-per-ofe-state-architecture.md` is 277 lines after M-D and carries the
required architecture, citation, and M-E breakdown content.

## M-C2

M-C2 edited work-package artifacts only. No Rust source line-count governance
was triggered.

## M-C

M-C edited work-package artifacts only. No Rust source line-count governance was
triggered.

## M-B

Ran `wc -l` over touched production/test Rust files and touched contracts/artifacts.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `03_kernel_support_00_support_helpers.rs` | 417 | OK |
| `hydrology_phase_runoff_reconciliation.rs` | 1255 | OK |
| `state_access.rs` | 1911 | OK, below 2000-line warning threshold |
| `scheduler_seed_and_runtime.rs` | 1890 | OK, below 2000-line warning threshold |
| `publication_wb11_seed.rs` | 557 | OK |
| M-B integration tests touched/added | 41-1175 | OK |
| `SC-WATBAL-001.md` | 2456 | Existing large contract authority; updated narrowly, not a Rust source line-count violation |

No touched Rust source file crossed the 2000-line warning threshold or 3000-line non-exempt threshold.

## M-A

Ran `wc -l` over the three M-A deliverables after edits:

| Artifact | Lines |
| --- | ---: |
| `characterization-openwepp-multi-ofe.md` | 104 |
| `legacy-per-ofe-closure-calibration.md` | 101 |
| `mofe-routing-port-scope.md` | 170 |

The routing scope artifact is longer because it carries file:line citations required by increment M-A. No production source line-count governance was triggered.
