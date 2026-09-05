# Line-count governance

Status: `TERMINAL INVENTORY — NO HARD-CEILING VIOLATIONS`

Evidence mode: `Static inventory`

Inventory every changed Rust file at terminal diff. Files at 2,000+ lines need
WARN disposition; nonexempt files at 3,000+ lines require refactor before
closure. Record decomposition evidence and any approved exception.

The terminal pre-refactor inventory found seven changed, nonexempt Rust files
above the 3,000-line hard ceiling. Before creating any split file, the package
write set was amended with one exact same-scope `include!` target for each:

- `open_snow_convergence_tests.rs` (4,327) -> historical-test quarantine;
- `v10_soil_thermal_v2.rs` (3,544) -> unpublished-continuation implementation;
- `v10_soil_thermal_v2_tests.rs` (3,167) -> resident-install tests;
- `snow_stage3_v11_terminal_execution.rs` (3,130) -> terminal event application;
- `surface_liquid_closure.rs` (3,130) -> terminal validation;
- `surface_liquid_ingress.rs` (3,098) -> mass allocation; and
- `owner_finalization.rs` (3,004) -> nitrogen-protocol tests.

The terminal audit after Lane-D and scale-seed correction found two more
touched files that had crossed the ceiling since the seven-file inventory:

- `direct_runtime/runoff.rs` (3,365) -> result/downstream-publication type tail;
  and
- `direct_runtime/03_executor.rs` (3,064) -> executor test module.

The package write set was amended before either extraction. All planned
originals and new files are projected below 3,000 lines. Terminal
evidence must record exact post-format counts, focused behavior checks, source
guards affected by moved text, and WARN dispositions for every remaining
2,000--2,999-line changed file.

## Native-V2 soil-thermal splits — 2026-09-02

Ran: the unpublished-continuation implementation and resident-install test
tail were moved without signature or logic changes into same-module lexical
`include!` files. Post-`rustfmt` counts are:

- `v10_soil_thermal_v2.rs`: 2,877 (`WARN`);
- `v10_soil_thermal_v2_unpublished_continuation.rs`: 669 (`PASS`);
- `v10_soil_thermal_v2_tests.rs`: 2,981 (`WARN`); and
- `v10_soil_thermal_v2_resident_install_tests.rs`: 219 (`PASS`).

The production `WARN` retains the public/native-V2 resident and real-consumer
wiring in its established module while isolating the coherent unpublished
continuation implementation. If that host grows again, the next split seam is
the complete `DirectV10RealConsumerShadow` native-V2 implementation block. The
test `WARN` retains the chronological V39--V48 custody suite; if it grows again,
the next split seam is the complete V43/V44 numerical-projection test family.

Ran validation:

- `cargo test -p openwepp-hillslope-orchestrator --lib unpublished_soil_beginning`:
  `PASS` (6 passed);
- `cargo test -p openwepp-hillslope-orchestrator --lib direct_v10_soil_thermal_v2_v49_tests`:
  `PASS` (8 passed);
- `cargo test -p openwepp-hillslope-orchestrator --lib direct_v10_soil_thermal_v2`:
  `PASS` (40 passed);
- `rustfmt --edition 2024` on all four split files: `PASS`; and
- scoped `git diff --check`: `PASS`.

Static source-guard impact: the package integration guard
`unpublished_soil_continuation_enters_v3_only_as_typed_candidate_beginning`
currently reads only `v10_soil_thermal_v2.rs`; its production-source aggregate
must also read `v10_soil_thermal_v2_unpublished_continuation.rs` before that
guard is rerun. Updating that integration test is outside this bounded split
ownership and remains assigned to the package's integration-test owner.

## Terminal/closure/ingress splits — 2026-09-02

Ran: three cohesive implementation blocks were moved without signature or
logic changes into same-module lexical `include!` files. Post-`rustfmt` counts
are:

- `snow_stage3_v11_terminal_execution.rs`: 2,789 (`WARN`);
- `snow_stage3_v11_terminal_event_application.rs`: 344 (`PASS`);
- `direct_runtime/surface_liquid_closure.rs`: 1,898 (`PASS`);
- `direct_runtime/surface_liquid_closure_terminal_validation.rs`: 1,232
  (`PASS`);
- `direct_runtime/surface_liquid_ingress.rs`: 2,908 (`WARN`); and
- `direct_runtime/surface_liquid_ingress_mass_allocation.rs`: 190 (`PASS`).

The terminal host retains provider/evaluator/replay orchestration and delegates
only the coherent accepted-event application transaction. The ingress host
retains canonical source construction, WB14 execution, and routing while the
new include owns only bounded retained/infiltration mass allocation. Both WARN
hosts are below the hard ceiling; their next coherent seams are the covered
executor preparation family and the one-OFE advance/routing family,
respectively.

Ran validation:

- orchestrator test compilation: PASS;
- terminal source/authority guards: PASS, `17/17`;
- terminal provider-retention proof: PASS, `1/1`;
- surface-liquid closure module: PASS, `8/8`;
- surface-liquid ingress module: PASS, `43/43`;
- ingress context module: PASS, `10/10`;
- surface-liquid V2 ingress module: PASS, `12/12`;
- constitutive terminal source guard and package litter-capacity source guard:
  PASS, `1/1` each;
- `cargo fmt --all -- --check`: PASS; and
- `git diff --check`: PASS.

## Intermediate changed-file WARN inventory — 2026-09-02

The post-format, pre-performance-correction inventory has no changed Rust file
at or above 3,000 lines. Every changed file in the 2,000--2,999 WARN band is
listed below; this inventory is rerun after the final correction/review diff.

| Lines | File | WARN disposition / next coherent seam |
| ---: | --- | --- |
| 2,399 | `direct_runtime/03_executor.rs` | Core publication stream after the test extraction; split committed-row assembly next. |
| 2,033 | `direct_runtime/laned_active.rs` | Cohesive Lane-D source, route, and closure owner; split public-evidence projection next. |
| 2,627 | `direct_runtime/runoff.rs` | Ordered R4/WB14 computation after type-tail extraction; split WB14 numerical kernel next. |
| 2,654 | `direct_runtime/stage3_committed_publication.rs` | Committed-day owner transaction; split accepted-lane construction next. |
| 2,908 | `direct_runtime/surface_liquid_ingress.rs` | Ingress/WB14 coordinator after mass-allocation extraction; split one-OFE advance/routing next. |
| 2,882 | `land_surface_energy_shadow/real_hydrology_execution.rs` | Native LSE/soil/surface finalization seam; split heterogeneous resource-join assembly next. |
| 2,204 | `land_surface_energy_shadow/v3_tests.rs` | Same-module native V3 behavior tests; split the heterogeneous-join family next. |
| 2,789 | `snow_stage3_v11_terminal_execution.rs` | Terminal provider/replay owner after event-application extraction; split covered-executor preparation next. |
| 2,355 | `v11_covered/carrier_phase.rs` | Typed phase and terminal-event protocol; split receipt construction next. |
| 2,955 | `v11_covered/execution.rs` | Canonical covered execution and custody plumbing; split boundary/receipt assembly next. |
| 2,823 | `v11_covered/owner_finalization.rs` | Accepted owner finalization after nitrogen-test extraction; split complete-owner candidate assembly next. |
| 2,707 | `v9_real_consumer_shadow.rs` | Real-consumer facade/owner state; split V11 install and restart glue next. |
| 2,877 | `v9_real_consumer_shadow/v10_soil_thermal_v2.rs` | Native V2 owner after continuation extraction; split the resident implementation next. |
| 2,981 | `v9_real_consumer_shadow/v10_soil_thermal_v2_tests.rs` | Chronological V39--V48 custody tests; split V43/V44 projection tests next. |
| 2,931 | `openwepp-runner/src/hillslope/03_tests.rs` | Existing runner-test aggregation touched only to register the isolated long-run include; split older test families next. |
| 2,338 | `openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | Real runner execution/output transaction; split manifest/output finalization next. |
| 2,723 | `direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | Typed production-frame/seed authority builders; split validation helpers next. |
| 2,227 | `snow_stage3_v11_production_seed.rs` | Exact owner-sidecar seed construction and scale-fixture expansion; split test-only fixture authority next. |
| 2,864 | `tests03/stage3_runner_qualification.rs` | Real-runner Stage-3/Lane-D qualification; long-run tests are already isolated, and complete-season fixtures are the next split. |
| 2,237 | `openwepp-vegetation/src/v11.rs` | V11 production transaction after full test extraction; split canonical owner/receipt helpers next. |
| 2,304 | `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs` | Contract-derived terminal-event matrix; split chronology and enthalpy families next. |
| 2,822 | `tests/integration/vegetation_boundary_authority_contract.rs` | Contract-derived vegetation authority matrix; split revision-binding and exposure-index families next. |

All WARNs preserve cohesive authority or test families and have a named next
seam. None is an exception to the 3,000-line hard ceiling.

The terminal batch source guard was made whitespace-insensitive while retaining
the exact custody tuple sequence, including the concurrently added
`snow_enthalpy_material_owner`; this preserves its semantic assertion rather
than weakening it.

## Surface-liquid restart physical order — 2026-09-02

The bare persisted-state restore now rejects duplicate record/store and
continuation/OFE identities without imposing lexical string order.
`restore_with_configuration` remains unchanged and continues to enforce native
topology-ranked positional validation plus configuration and state digests.

Ran validation:

- physical 1/9/10/19-OFE restore-with-configuration and
  duplicate/omission/substitution/reorder/stale-digest no-mutation selection:
  PASS, `9/9`;
- full `openwepp-persisted-restart-v1` library suite: PASS, `67/67`; and
- `cargo check -p openwepp-persisted-restart-v1 --tests`: PASS.

## Runoff/executor hard-ceiling splits — 2026-09-02

Ran: the complete direct-runoff carrier/result type tail and the complete
executor test module were moved without signature or logic changes into
same-module lexical `include!` files. Post-`rustfmt` counts are:

- `direct_runtime/runoff.rs`: 2,627 (`WARN`);
- `direct_runtime/runoff_result_types.rs`: 739 (`PASS`);
- `direct_runtime/03_executor.rs`: 2,399 (`WARN`); and
- `direct_runtime/03_executor_tests.rs`: 667 (`PASS`).

The runoff host retains the ordered R4/WB14 computations while the extracted
tail owns only typed inputs, states, downstream operands, shadows, and span
reports. The executor host retains the publication-stream implementation while
the extracted file contains only its existing `#[cfg(test)]` module. Their next
coherent split seams, if either WARN host grows again, are the WB14 numerical
kernel and the committed-publication stream implementation respectively.

Ran validation:

- `cargo check -p openwepp-hillslope-orchestrator --tests`: PASS;
- focused runoff/executor/WAT5/Lane-D selection: PASS, `126/126`;
- include-aware Lane-D public-hourly source guard: PASS, `1/1`;
- `cargo fmt --all -- --check`: PASS; and
- `git diff --check`: PASS.

## Performance-continuation hard-ceiling splits — 2026-09-04

Ran: the final correction diff was inventoried after formatting. Four touched
hosts had crossed the nonexempt 3,000-line ceiling. Their exact same-module
include targets were added to the package write set before extraction, and
only existing item bodies moved:

- `v11_covered/carrier_phase.rs`: 3,103 -> 2,828 lines;
  `carrier_phase/snow_boundary.rs`: 277 lines;
- `v9_real_consumer_shadow.rs`: 3,013 -> 2,816 lines;
  `v9_real_consumer_shadow/live_vegetation_forcing.rs`: 198 lines;
- `direct_runtime/surface_liquid_owner.rs`: 3,015 -> 2,953 lines;
  `surface_liquid_owner/validation_helpers.rs`: 63 lines; and
- `tests03/stage3_runner_qualification.rs`: 3,085 -> 2,917 lines;
  `stage3_runner_qualification/complete_season_reappearance.rs`: 169 lines.

No signature, visibility, equation, branch, error, fixture, assertion, or
call-site changed. `cargo check -p openwepp-runner --tests`, focused carrier
boundary/native-V2 tests `2/2`, `cargo fmt --all -- --check`, and
`git diff --check` passed.

## Terminal complete changed-file WARN inventory — 2026-09-04

The formatted current census covers all 179 existing changed/untracked Rust
files and contains no file at or above 3,000 lines. The table lists all 46
files in the 2,000--2,999 WARN band; every smaller file was also counted but
is not expanded here. Every WARN is not an exception; its
cohesive authority/test ownership is retained and any future growth requires a
new prospectively authorized seam.

| Lines | File | Disposition |
| ---: | --- | --- |
| 2008 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2036 | `crates/openwepp-land-surface-energy/src/owner_envelope.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2040 | `crates/openwepp-persisted-restart-v1/src/snow_stage3_v11_tests.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2084 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2099 | `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_production_tests.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2154 | `crates/openwepp-hillslope-orchestrator/src/v11_covered/canonical_covered_solver.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2204 | `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_tests.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2304 | `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2327 | `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/multi_tile_runtime.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2351 | `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2367 | `crates/openwepp-hillslope-orchestrator/src/canonical_owner_bytes.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2367 | `crates/openwepp-vegetation/src/v11.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2423 | `crates/openwepp-runner/src/hillslope/snow_stage3_v11_production_seed.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2433 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2639 | `crates/openwepp-land-surface-energy/src/solver_covered_evaluation.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2564 | `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_execution.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2619 | `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_owner.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2635 | `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_input_projection.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2668 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2682 | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2683 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/stage3_committed_publication.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2723 | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2767 | `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2816 | `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2822 | `tests/integration/vegetation_boundary_authority_contract.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2874 | `crates/openwepp-hillslope-orchestrator/src/v11_covered/carrier_phase.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2882 | `crates/openwepp-hillslope-orchestrator/src/v11_covered/execution.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2895 | `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2917 | `crates/openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2918 | `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_terminal_execution.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2932 | `crates/openwepp-runner/src/hillslope/03_tests.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2944 | `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/real_hydrology_execution.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2944 | `crates/openwepp-land-surface-energy/src/transaction.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2948 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2948 | `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_restart.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2948 | `crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2953 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2955 | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2959 | `crates/openwepp-runner/src/hillslope/tests03/stage3_long_run_qualification.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2970 | `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2974 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_wb14.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2977 | `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment_receipts.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2981 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2981 | `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2_tests.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2989 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_tests.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
| 2995 | `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_publication_retention.rs` | WARN — cohesive existing authority/test host; terminal count remains below 3,000. |
