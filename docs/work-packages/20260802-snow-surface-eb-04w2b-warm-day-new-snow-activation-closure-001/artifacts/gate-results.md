# Gate Results

Status: `HOLD` — renewed quick correctness gate failed

Evidence mode: **Ran**

| Gate | Result |
|---|---|
| Pre-implementation EB-04W contract target | expected red: 4 passed, warm-new-snow activation failed |
| Post-correction EB-04W contract target | 5 passed |
| Snowbench consumer closure tests | passed |
| Owning orchestrator crate | 427 passed |
| `cargo nextest run --workspace --profile frost` | 343 passed, 1,895 skipped, 550.468 s |
| targeted warnings-denied clippy | passed |
| assurance catalog validation | 3 reports passed |
| pre-review quick profile at `-j 8` | 2,146 passed, 38 skipped, 2,512.720 s; superseded by corrected diff |
| pre-review critical full profile at `-j 4` | 2,195 passed, 31 skipped, 3,193.096 s; superseded by corrected diff |
| first frozen W2A eight-cell rerun | 8/8 process cells passed; prerequisite-ineligible |
| formatting / `git diff --check` | passed |

The first high-concurrency full attempt was stopped after assurance fault tests
exceeded the execution window under contention; it reported no correctness
failure. The single authorized retry reduced concurrency to four and completed
the exact full profile cleanly. Those runs preceded review-required changes and
do not close the terminal diff.

## Corrected-Diff Commands

All commands ran from `/home/workdir/openWEPP`.

| Exact argv | Exit/result |
|---|---|
| `cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(eb04w2b_storage_guard_enforces_exact_tolerance_and_nonfinite_rejection) \| test(cqr_row4_build_forcing_resolves_warm_precip_and_suppresses_warm_dry_days)'` | `0`; 2/2 passed |
| `cargo nextest run -p openwepp-runner --lib -E 'test(eb04w2b_direct_production_warm_mean_new_snow_reaches_shared_partition) \| test(noncanonical_new_snow_density_fails_closed_at_consumer_storage_boundary) \| test(coe_melt_consumer_fails_closed_on_material_daily_swe_residual)'` | `0`; 3/3 passed |
| `cargo nextest run --test snow_surface_eb04w_accumulation_melt_diagnostics_contract` | `0`; 6/6 passed |
| `cargo nextest run -p openwepp-hillslope-orchestrator -p openwepp-runner` | `0`; 653/653 passed in 147.797 s |
| `cargo nextest run --workspace --profile frost` | `0`; 345/345 passed, 1,896 skipped, 544.838 s |
| `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings` | `0` |
| `cargo run -q -p openwepp-assurance -- validate --all` | `0`; 3/3 reports |
| `cargo nextest run --workspace --profile quick -j 8` | `100`; stopped after deterministic EROD16 failure (`61/231`, hard bound `<=20%`) at 2,285.162 s |
| exact EROD16 test under corrected trigger | `100`; reproduced `61/231` in 25.210 s |
| exact EROD16 test under temporary diagnostic old-trigger reversal | `0`; observed prior `37/227` in 26.556 s; stdout/temporary patch were not retained, so this is supporting observation only |

The old-trigger reversal was diagnostic only, was immediately removed, and did
not retain a log or patch identity; it therefore cannot carry a closure claim.
The independently retained corrected-trigger quick and isolated failures are
sufficient for the HOLD. The working diff retains the contract-corrected snow
behavior. The terminal full
profile and frozen rerun were not executed because the quick prerequisite
failed. `cargo deny check` is not applicable: no manifest, lockfile, or
dependency-resolution input changed.

## Terminal In-Envelope Review Corrections

After re-review clarified the tolerance wording and requested stronger direct
consumer proof, the contract retained all existing cold/snow/frost provider
triggers and the regression was extended through the production fixture, seed,
run frame, and `DirectProductionDayInputBuilder`. The following exact commands
then ran from `/home/workdir/openWEPP`:

| Exact argv | Exit/result |
|---|---|
| `cargo fmt --check` | `0` |
| `cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(eb04w2b_storage_guard_enforces_exact_tolerance_and_nonfinite_rejection) \| test(cqr_row4_build_forcing_resolves_warm_precip_and_suppresses_warm_dry_days)'` | `0`; 2/2 passed |
| `cargo nextest run -p openwepp-runner --lib -E 'test(eb04w2b_direct_production_warm_mean_new_snow_reaches_shared_partition) \| test(noncanonical_new_snow_density_fails_closed_at_consumer_storage_boundary) \| test(coe_melt_consumer_fails_closed_on_material_daily_swe_residual)'` | `0`; 3/3 passed |
| `cargo nextest run --test snow_surface_eb04w_accumulation_melt_diagnostics_contract` | `0`; 6/6 passed |
| `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings` | `0` |
| `cargo run --quiet -p openwepp-assurance -- validate --all` | `0`; 3/3 reports, generation `9e64c4c70ed9a5e77d1d9f1de373ef1ad11b27058d23ff030ec140ecdff36cea` |
| scoped `markdown-doc lint` over contracts, roadmaps, catalog, and package | `0`; terminal run 36 files, 0 errors, 0 warnings |
| terminal `git diff --check` | `0` |

These clarification/test-only changes do not affect the retained corrected
EROD16 failure. No quick/full rerun or W2A result rerun is admitted while that
hard prerequisite remains red.
