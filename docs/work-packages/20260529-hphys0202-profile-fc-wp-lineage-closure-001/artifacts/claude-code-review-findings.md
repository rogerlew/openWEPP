# HPHYS0202 Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-29
Scope: WB13 `ProfileFCStore` / `ProfileWPStore` publication-lineage change and
its supporting tests, contract amendments, and diagnostic evidence.

Evidence classes used below: **Static** (read source / contract / artifact and
reasoned) and **Ran** (command executed). Where a run was produced by the
package executor rather than this reviewer, it is attributed as such.

---

## F-1 — Workspace gates pass (independently re-run)

Ran (reviewer, from `/home/workdir/openWEPP`):

- `cargo fmt --check` → exit 0
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `cargo test --workspace` → exit 0
- `cargo deny check` → exit 0 (only an `unmatched license allowance`
  warning for `Unicode-DFS-2016` in `deny.toml`; non-failing)

The four gate claims in `gate-results.md` reproduce.

## F-2 — Contract-test file is behavioral, not structural

Static + Ran. `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
no longer asserts on source-string presence for the runtime path. It executes
`execute_hillslope_run` end-to-end and asserts on the published
`H5.wat.parquet` `ProfileFCStore`/`ProfileWPStore` columns. The seed-divergence
case mutates fixture bulk density to force `wb13_profile_fc/wp_store_mm` seed
values to diverge from the per-layer aggregate, pre-asserts that divergence
(`> 1e-6`) so the test cannot pass vacuously, then asserts the published value
tracks the aggregate and not the seed. Ran (reviewer):
`cargo test --test hphys0202_profile_fc_wp_lineage_contract` → 3 passed.

## F-3 — Direct WB13 guard probes added; evidence labeling is accurate

Static. Three crate-internal unit tests were added in
`crates/openwepp-runner/src/hillslope/mod.rs` (`#[cfg(test)] mod tests`):
`hphys0202_wb13_fc_seed_guard_is_exercised_by_direct_row_builder_probe`,
`hphys0202_wb13_wp_seed_guard_is_exercised_by_direct_row_builder_probe`,
`hphys0202_wb13_profile_fc_wp_publication_ignores_seed_values_when_valid`.
They call the private `build_simulation_owned_wb13_row` directly, exercise the
reachable `wb13_publication` / `SIMOUT-E-001` seed guards (NaN and negative
seed), and assert published values equal the layer aggregate
(`0.30·0.25·1000`, `0.12·0.25·1000`) rather than injected valid seeds (999/555).
`contract-test-implementation-evidence.md` correctly attributes the
integration-test invalid-state hard-fail to the upstream `wb11_seed` guard
(`HS-SIMPIPE-E-001`), distinct from the WB13 seed guards. No overclaim of guard
coverage was found in the test evidence.

## F-4 — Per-layer WB13 builder guards are unreachable defense-in-depth

Static. The per-layer `thetfc < 0.0` / `thetdr < 0.0` / `dg <= 0.0` guards
inside `build_simulation_owned_wb13_row`
(`crates/openwepp-runner/src/hillslope/mod.rs:3739–3759`) read the same
`thetfc_####`/`thetdr_####`/`dg_####` symbols already guarded — with identical
predicates plus a stricter combined `ul_store > 0.0` check — at the upstream
`wb11_seed` block (`mod.rs:1682–1728`). Any invalid value trips `wb11_seed`
first, so these per-layer WB13 guards cannot fire through the pipeline. (The
WB13 *seed-symbol* finiteness guards at `mod.rs:3790–3802` are distinct and are
reachable; they are the ones F-3's probes cover.)

## F-5 — Published FC/WP source is uncorrected; legacy source is corrected

Static. The runtime per-layer symbols the change now aggregates are raw:
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:188–268`
emits `thetfc_####` = `layer.fc_rosetta.or(layer.fc_measured)` and
`thetdr_####` = `layer.theta_r_rosetta.or(layer.wp_measured)` with no soil-
physics correction applied. The corrected values are computed in a separate
function `compute_wb13_profile_symbols_from_legacy_seed`
(`02_soil_slope.rs:452–535`) — applying `coca` (entrapped air), `cpm`
(rock-fragment), the `sm20c` tension-curve cap, the `0.99`/`0.83` porosity caps,
and `0.01` floors — and are emitted only as the seed symbols
`wb13_profile_fc/wp_store_mm` (`02_soil_slope.rs:359–360`).

The legacy baseline (`/workdir/wepp-forest_260430_baseline`,
`src/watbalprint.for:60–69`) aggregates `thetfc(i)·dg(i)` and `thetdr(i)·dg(i)`
over `i=1..nsl`, ×1000, using the array values **after** `scon.for:261–339`
mutates them in place with the same correction chain (`cpm`, `sm20c`, `0.83`
cap, `0.01` floors). `src/cwater.inc:55–57` defines `thetfc` = 1/3-bar field
capacity, `thetdr` = 15-bar wilting point.

Consequence: the change publishes `Σ(raw thetfc·dg)·1000`, which omits the
corrections present in both the legacy values and the openWEPP corrected seed.

## F-6 — FC/WP regressed against the predecessor run

Ran (executor's 39-hillslope batch under
`/tmp/hphys0202_20260530T003833Z/parity/`, analyzed by reviewer) vs Static
(committed predecessor disposition).

HPARITY02 disposition
(`docs/work-packages/20260529-hparity02-profile-capacity-storage-lineage-closure-001/artifacts/hparity02_disposition.md:13–15`),
runner consuming the corrected seed (confirmed Static: `git show HEAD:…/mod.rs`
retains the seed-consuming `map_or(Ok(fallback_profile_fc_store_mm), …)`):

- `ProfilePorosityCap`: 0/39 fail
- `ProfileFCStore`: 27/39 fail (12 pass)
- `ProfileWPStore`: 1/39 fail (38 pass)

HPHYS0202 semantic summary
(`/tmp/hphys0202_20260530T003833Z/parity/reports/hillslope_semantic_summary.json`),
runner aggregating raw per-layer symbols:

- `ProfilePorosityCap`: 0/39 fail (still seed-sourced)
- `ProfileFCStore`: 39/39 fail
- `ProfileWPStore`: 39/39 fail

Net: `ProfileWPStore` 38 passing → 0 passing; `ProfileFCStore` 12 → 0.

## F-7 — Magnitude and direction of the FC/WP divergence

Ran (reviewer analysis of executor outputs). Per-column stats for hillslope 1
(`reports/semantic/H1.semantic.json`), constant per-day offset
(`mean_abs_diff ≈ rmse ≈ max_abs_diff`):

| Column | mean_abs_diff (mm) | max_rel_diff | fail_count |
|---|---|---|---|
| ProfilePorosityCap | 0.00016 | 1.5e-7 | 0 / 1461 |
| ProfileFCStore | 206.686 | 0.645 | 1461 / 1461 |
| ProfileWPStore | 87.181 | 0.735 | 1461 / 1461 |

Ran (reviewer, duckdb read of
`hillslope_output/H1.wat.parquet`): candidate `ProfileFCStore` = 320.326,
`ProfileWPStore` = 118.581, `ProfilePorosityCap` = 1092.710. Relative-diff
arithmetic (`|cand−base|/cand`: 206.686/320.326 = 0.645;
87.181/118.581 = 0.735) places the legacy baseline at FC ≈ 113.6, WP ≈ 31.4.
Direction: published FC ≈ 2.8× and WP ≈ 3.8× the legacy values; porosity
matches to 7 digits.

## F-8 — Residual on FC persists independent of the publication source

Static. Under the corrected seed (HPARITY02, F-6), `ProfileFCStore` still failed
27/39 while `ProfileWPStore` failed 1/39. The FC residual is therefore not
explained solely by the corrected-vs-raw publication source. This reviewer did
not re-derive the corrected-seed FC value per hillslope to localize the 27/39
FC residual.

## F-9 — Disposition / gap-matrix framing of the residual

Static. `hphys0202_disposition.md` records decision `HOLD` and states
"publication-lineage authority/test closure is complete, but baseline-
authoritative end-to-end process closure is not yet demonstrated."
`hphys0202-physics-gap-matrix.md` records `GAP-HP202-003` as "diagnostic
semantic residual persists … open (follow-on required)" with status for
`GAP-HP202-001` (publication authority) and `GAP-HP202-002` (test closure)
marked closed. Neither artifact records the F-6 fail-count change relative to
the predecessor run, nor the F-5 corrected-vs-raw source distinction.

## F-10 — Working tree is uncommitted

Static. As reviewed, the HPHYS0202 changes are unstaged in the working tree
(`git status`): modified `Cargo.toml`, `mod.rs`, five contract/index docs, and
the package artifacts; `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
is untracked. `git log` HEAD is `357e470` (the HPHYS0201 execution commit); no
HPHYS0202 commit exists.

## F-11 — Contract authority statements relative to the corrected source

Static. `SC-WATBAL-001.md` (HPHYS0202 amendment) designates per-layer
aggregation `Σ(thetfc_i·dg_i)` / `Σ(thetdr_i·dg_i)` as producer-authoritative
publication and `wb13_profile_fc/wp_store_mm` as "non-authoritative adapter
diagnostics." Per F-5/F-6/F-7, the per-layer symbols carry uncorrected inputs
and the seed carries the scon.for-corrected (legacy-matching, for WP 38/39)
values; the designated authoritative source is the one that diverges from the
baseline the package's provenance section requires tracing to.
