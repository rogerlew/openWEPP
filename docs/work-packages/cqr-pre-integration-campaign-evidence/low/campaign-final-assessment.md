# Pre-Integration CQR Campaign Final Assessment

Evidence class: **Ran + Static**

Status: `PASS`

Recommendation: `GO-INTEGRATED-VALIDATION`

## Campaign Result

The fixed baseline contained 67 raw CRAP-above-30 identities across 45
production modules. High A removed 13, High B removed 21, Medium removed 19,
and Low removed its 11 eligible identities. Final raw debt is two exact
formatter identities with current dual-reviewed `R-OBSERVABILITY`
dispositions; final actionable debt is zero. No tranche added a new identity.

| Tranche | Start rows/modules | Removed | Final rows/modules | Terminal result |
| --- | ---: | ---: | ---: | --- |
| High A | 67 / 45 | 13 | 54 / 35 | `TERMINAL-PASS` |
| High B | 54 / 35 | 21 | 32 / 25 | `TERMINAL-PASS` |
| Medium | 32 / 25 | 19 | 13 / 12 | `TERMINAL-PASS` |
| Low/Assessment | 13 / 12 | 11 eligible | 2 raw / 2; 0 actionable | `TERMINAL-PASS` |

## Original 45-Module Accounting

`Baseline` is the number of exact rows in the durable 67-row ledger. `Final`
is the raw count; every row in a zero-final module was removed by its named
reviewed checkpoint.

| Module | Baseline | Final | Disposition |
| --- | ---: | ---: | --- |
| `direct_runtime/01_publication.rs` | 1 | 0 | HA-06 PASS |
| `direct_runtime/03_executor.rs` | 1 | 0 | HA-05 PASS |
| `direct_runtime/erosion.rs` | 1 | 0 | HB-01 PASS |
| `direct_runtime/erosion_continuity.rs` | 5 | 0 | HB-04 PASS |
| `direct_runtime/erosion_operands.rs` | 1 | 0 | HB-02 PASS |
| `direct_runtime/erosion_seed.rs` | 1 | 0 | HB-03 PASS |
| `direct_runtime/laned_active.rs` | 1 | 0 | HA-03 PASS |
| `direct_runtime/projection.rs` | 1 | 0 | HA-04 PASS |
| `ofe_routing/cascade.rs` | 1 | 0 | HA-01 PASS |
| `ofe_routing/iwagaki_oracle.rs` | 1 | 0 | M-10 PASS |
| `ofe_routing/kinematic_wave.rs` | 3 | 0 | HA-02 PASS |
| `parsers/climate.rs` | 3 | 0 | M-01 PASS |
| `parsers/frost.rs` | 1 | 0 | L-01 PASS |
| `parsers/gwcoeff.rs` | 2 | 0 | M-02 PASS |
| `parsers/hbp/error.rs` | 1 | 0 | M-04 PASS |
| `parsers/phosphorus.rs` | 1 | 0 | L-02 PASS |
| `parsers/pmetpara.rs` | 1 | 0 | L-03 PASS |
| `parsers/snow.rs` | 2 | 0 | M-03 PASS |
| `parsers/tcr.rs` | 1 | 0 | L-04 PASS |
| `parsers/watershed_structure.rs` | 1 | 0 | HB-05 PASS |
| `parsers/wepp_ui.rs` | 1 | 0 | L-05 PASS |
| `openwepp-landuse-migrate/src/cli.rs` | 1 | 0 | M-13 PASS |
| `openwepp-landuse-migrate/src/convert.rs` | 1 | 0 | M-12 PASS |
| `openwepp-landuse-migrate/src/lib.rs` | 2 | 0 | M-11 PASS |
| `openwepp-legacy-bridge/src/hbp.rs` | 1 | 0 | L-06 PASS |
| `openwepp-legacy-bridge/src/sidecar.rs` | 1 | 0 | L-07 PASS |
| `openwepp-management-schema/src/lib.rs` | 3 | 0 | M-05 PASS |
| `openwepp-meteorology/src/error.rs` | 1 | 1 | L-08 `R-OBSERVABILITY` |
| `openwepp-runner/src/bin/open_wepp_runner.rs` | 1 | 0 | HA-10 PASS |
| `openwepp-runner/src/bin/openwepp-cli-hill.rs` | 1 | 0 | HA-09 PASS |
| `openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 6 | 0 | HB-10 PASS |
| `hillslope/05_runner_execution_and_outputs.rs` | 2 | 0 | HA-08 PASS |
| `day_input_and_helpers/00_builders_and_authority.rs` | 1 | 0 | HA-07 PASS |
| `day_input_and_helpers/00c_day_input_builder_impl.rs` | 1 | 0 | M-08 PASS |
| `intake_lane_setup/runfile_helpers.rs` | 1 | 0 | M-07 PASS |
| `hillslope/snowbench.rs` | 1 | 0 | L-09 PASS |
| `hillslope/snowbench_coe_melt.rs` | 2 | 0 | L-10 PASS |
| `openwepp-runner/src/release.rs` | 1 | 0 | M-09 PASS |
| `openwepp-sim-contract/src/symbols.rs` | 1 | 1 | L-11 `R-OBSERVABILITY` |
| `units_mod/registries.rs` | 1 | 0 | M-06 PASS |
| `openwepp-summary-accumulator/src/lib.rs` | 1 | 0 | HB-06 PASS |
| `kernel/diagnostics.rs` | 2 | 0 | HB-07 PASS |
| `kernel/hourly.rs` | 1 | 0 | HB-08 PASS |
| `lib_mod/network_frame.rs` | 1 | 0 | L-12 PASS |
| `openwepp-watershed-output/src/writers.rs` | 2 | 0 | HB-09 PASS |

The abbreviated paths are unambiguous suffixes of the exact paths in
`cqr-pre-integration-campaign-baseline.md`; that ledger remains the row-level
identity authority. The baseline counts above sum to 67 and the final counts
sum to two.

## Low Package And Commit Ledger

| IDs | Terminal commits |
| --- | --- |
| L-01..L-03 | `47b29492`, `6019a98b`, `ba369f5c` |
| L-04..L-07 | `432b493f`, `cb175ee1`, `279397c6`, `84a0215d` |
| L-08 | dual-reviewed no-action record at `fa50c0be` |
| L-09..L-10 | `aaacd18e`, `e98f7a13` |
| L-11 | dual-reviewed no-action record at `fa50c0be` |
| L-12 and lint closure | `9145d288`, `8e0f7367` |

High A, High B, and Medium package/commit ledgers remain in their terminal
ExecPlans and evidence trees; their transition commits are ancestors of the
Low measurement commit. The final assessment does not rewrite those ledgers.

## Exit Conditions

Every original high and medium row is absent. Every eligible Low row is absent,
and both retained Low rows have exact current-source dual review. Module
records demonstrate their applicable tier, target coverage/floors, obligation
map, and consumers; no accepted finding is deferred. Workspace formatting,
all-target Clippy, full nextest, deny, Markdown, and diff gates pass. The final
coverage/CRAP artifacts are source-bound and show no new row or regression.

There is no CQR follow-up queue because no actionable identity, unresolved
defect, evidence gap, or dirty overlap remains. The exact campaign disposition
is `GO-INTEGRATED-VALIDATION`.
