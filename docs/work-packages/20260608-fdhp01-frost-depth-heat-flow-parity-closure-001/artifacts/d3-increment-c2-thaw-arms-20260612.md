# D3 Increment C2 Thaw-Arms Execution

Status: executed-hold

Evidence mode: Ran

Date: 2026-06-12

Comparator subagent: not used. The parent run executed local DuckDB/CLI cohort
comparisons directly because the user reported GPT-5.3-Codex-Spark weekly quota
exhaustion.

## Scope

Increment C2 implemented the thaw-arm state-machine surface on top of the C1b
capacity/overflow ownership path:

- bottom thaw (`mltbtm`) retreats the lower frost front, releases proportional
  fine-layer ice, preserves frost-at-top geometry for partial layers, and
  routes lower overflow through `watbtm`;
- top thaw (`mlttp`) retreats from the surface, preserves frost-at-bottom
  geometry for sandwich frost, publishes positive thaw depth while frost remains
  below, routes upper overflow through `watpdg`, and sets `fgthwd` on thaw
  through;
- mixed arms keep the legacy directionality: freeze, freeze plus bottom thaw,
  top thaw, and bottom thaw-only;
- the scalar handoff was repaired so the post-frost liquid scalar is derived
  from the owned fine state at egress rather than preserving stale pre-thaw
  scalar storage.

`SC-SNOWFREEZE-001` is now version 62 for these thaw-arm ownership rules.

## Local Gates

| Command / Gate | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_c2 -- --nocapture` | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_d2_contract_frwatc_freeze_exchange_diagnostics_reconcile_liquid_and_frozen_storage -- --nocapture` | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_c1b_overflow_routes_to_watbtm_and_closes_shadow_identity -- --nocapture` | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 33 tests |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass, release binary SHA `0b3fed8561232d0de371a195e3f5a5609121ddd2253713b3fb0139add9ec8a4f`; sidecar SHA `5bfe72bec363900838255730a8cd5ee60144294c1a6640282e774f9e3c2f94a7` |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass, `advisories ok, bans ok, licenses ok, sources ok` |
| `bash tools/release/check_authority_suite_antievasion.sh` | Pass |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | Pass, 2 tests |

## Cohort Evidence

The authoritative C2 run used the hourly `algebraic-radium` run directory:

`/wc1/runs/al/algebraic-radium/wepp/runs`

Run root:

`/tmp/fdhp01_increment_c2_cohort_hourly_fix_20260612T035740Z`

The earlier `/tmp/fdhp01_increment_c2_cohort_fix_20260612T035607Z` run is not
authoritative because its temporary run directory missed the hourly
`wepp_ui.txt` lane and selected the daily lane. The retained run reports
`selected_lane=hourly` and `mode_divergence=false`.

| Gate | Result |
|---|---|
| Clean execution | Pass, `43/43` prefixes |
| WAT outputs | Pass, `43/43` |
| Years 2-6 `Total-Soil + frozwt` closure | Pass, max abs residual `0.0 mm` using the package C1b additive-storage ledger shape |
| Year-7 boundary watch | Pass, max abs residual `0.0 mm` |
| Profile-bound pinning | Pass, `0/43` pinned; minimum margin to profile bound `5.54557792097421 mm` |
| `frozwt/frdp` scalar-signature rejection | Pass, max correlation `0.9441102161636825`, median `0.8831449770567324` |
| Depth envelope | Fail, mean max depth `1793.52198510966 mm`, median `1793.649969637327 mm`, max `1794.4544220790258 mm` |
| Depth correlation | Fail, median `-0.16722397856345997`, range `-0.30843972585040713..0.19389645706206324` |
| Frozen-duration residual | Fail, open-minus-legacy median `111` days, mean `74.48837209302326` days, range `-293..171` days |
| Days above 200 mm | Fail/watch, median `815` days |

Generated cohort artifacts:

- `fdhp01_increment_c2_run_status_20260612.tsv`
- `fdhp01_increment_c2_annual_closure_residuals_20260612.csv`
- `fdhp01_increment_c2_depth_metrics_20260612.csv`
- `fdhp01_increment_c2_frozwt_frdp_ratio_20260612.csv`
- `fdhp01_increment_c2_activation_summary_20260612.csv`
- `fdhp01_increment_c2_execution_summary_20260612.json`

## Disposition

C2 lands as `executed-hold`, not closure. It removes the C1b ambiguity:
bottom/top thaw, sandwich geometry, thaw-through, and overflow routing can run
cleanly across the 43-prefix cohort without reopening D2/p2 or the year-7
boundary item. The package's D3 acceptance gate still fails because frost depth
remains near the physical profile bound and depth correlation remains negative.

The discriminating experiment therefore selects the staged-plan reading that
the remaining defect is freeze-side energy/resistance/front-advance behavior,
not missing thaw-arm storage plumbing. The next implementation scope should
audit and repair the freeze-arm heat-flow resistance/latent-heat path using the
C2 cohort evidence as the boundary condition. It should not loosen C1b/C2
capacity, overflow, publication, or conservation guards.
