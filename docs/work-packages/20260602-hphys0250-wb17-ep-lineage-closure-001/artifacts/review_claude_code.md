# Review: Claude Code

Status: complete

Evidence mode: static (source/contract/test read) + recorded-log read

Static:

- Reviewer: Claude Code (`claude-opus-4-8`), invoked by user post-commit.
- Scope: static correctness review of HPHYS0250 commit `ff395be` — initial
  live-canopy assimilation (`runtime_inputs/01_management.rs`), PL scheduler
  activation (`runner/src/hillslope/mod.rs`,
  `hydrology/00_pl_slot_resolution.rs`), growth-state seeding
  (`hydrology/06_growth_state.rs`), the WB15 near-zero roundoff guard
  (`hydrology/03_kernel_support_00_support_helpers.rs`), the growth-transition
  writeback phase (`hydrology/04_kernel_execution.rs`), and SC-EVAP-001 /
  SC-PLANT-001 / SC-WATBAL-001 amendments.
- Baseline cross-check: equations verified directly against
  `/workdir/wepp-forest_260430_baseline/src/initgr.for`, `init1.for`,
  `grow.for`, `growop.for`, `ptgrp.for`.
- Continuity: this review follows up HPHYS0249 `review_claude_code.md` finding
  5 (`Ep` unchanged after the transpiration pathway was rewritten), which
  HPHYS0250 set out to close.
- Gate evidence: read from committed `artifacts/gate-logs/`. Claude Code ran
  no `cargo` commands in this review; test pass/fail statements are attributed
  to the recorded logs, not to a run by the reviewer.

## Confirmations (no action)

1. Initial live-canopy assimilation is a faithful port of `initgr.for:64-99`:
   `vdmt = log(1-cancov)/-bb` (with `bb>0` guard, `vdmt<0 -> 0`),
   `canhgt = (1-exp(-bbb*vdmt))*hmax`, LAI `0.5512/-6.8` for `imngmt==1` else
   `0.2756/-13.6`, and `sumgdd = gddmax*lai/xmxlai`. Shape constants also
   appear in `grow.for`/`growop.for`/`ptgrp.for`.
2. `rtd`/`rtmass` seeding matches `init1.for`: perennial `(rdmax, rtmmax)`
   <- `init1.for:176-183`; annual `(rsr*canhgt, rsr*vdmt)` <-
   `init1.for:348-353`.
3. Contract-first ordering honored. SC-PLANT-001 (v16, `INV-PLANT-025`) and
   SC-EVAP-001 (v11, `INV-EVAP-016`) were amended before production edits and
   cite `init1.for:147-244,334-356` and `initgr.for:63-105`. The
   pre-implementation gate log records a real failure before the fix.
4. The `Ws=1.0` neutral-stress seed is contract-grounded (SC-EVAP-001 final-Ep
   addendum: decomposition runs before same-day ET, so prior stress must be
   neutral), not undocumented scope creep.
5. The WB15 `normalize_non_negative_within_tolerance` helper is a bounded
   roundoff guard: it snaps only `(-WB11_ZERO_THRESHOLD, 0.0)` to `0.0`;
   material negatives still hit the typed `require_flux_range` /
   `require_non_negative_liquid_input` failures. Defensible, not a clamp-away
   of real error.
6. HPHYS0249 finding 5 is confirmed: `Ep` was structurally zero because
   `PlantRootUptake` executed with `rtd=0`. The pathway now activates and
   `Ep` is nonzero.

## Findings

1. Medium — growth-equation constants are embedded inline in
   `01_management.rs`, not in `constants.rs` with `REF-` citations. The values
   `0.5512`, `6.8`, `0.2756`, `13.6`, `0.999`, and the LAI/`vdmt` formulas live
   as literals inside `apply_primary_initial_live_canopy_assimilation`. This is
   a discipline regression from HPHYS0249, which keyed every coefficient to a
   named `constants.rs` entry with a `REF-EVAP-LEGACY-*` reference. The values
   are baseline-faithful, but uncited at the code site and harder to
   contract-trace. Evidence: `01_management.rs` assimilation body;
   `initgr.for:82-90` shape constants.

2. Medium — the `cancov >= 0.999` clamp has no baseline counterpart and is
   undisclosed. `initgr.for` does not clamp `cancov`; openWEPP adds the cap to
   keep `log(1-cancov)` finite (baseline emits `-inf`/`NaN` at `cancov=1.0`).
   The guard is arguably necessary, but it silently caps a physical input
   rather than failing, and is not mentioned in `INV-PLANT-025` or any numeric
   note. Recommend disclosing it as an openWEPP-introduced numeric deviation.
   Evidence: `01_management.rs` `if cancov >= 0.999 { cancov = 0.999; }`;
   `initgr.for:64-71` (no clamp).

3. Medium — the contract-first "red" is weak. The one pre-implementation
   failing test
   (`hphys0250_scheduler_lifecycle_preserves_pl_runtime_sentinel_for_ep_lineage`)
   is a source-string grep that asserts the file text lacks a forbidden
   fragment, not a behavioral physics test. The behavioral PL-activation and
   live-canopy assimilation tests were authored alongside the production
   functions, so they could not have gone red first. The red-green
   demonstration is thinner than HPHYS0249's, where behavioral kernel vectors
   went red->green. Evidence: `gate-logs/pre_impl_hphys0250_contract_tests.log`
   (1 failed string-guard test, 1 passing control); `mod.rs` test body.
   Credit: `pre-implementation-contract-gate.md` carries an explicit
   truthfulness note correcting an earlier overstatement.

4. Low — `solthk` root-depth cap omitted at seed time. Both `rtd` seed branches
   drop the `rtd > solthk -> solthk` cap that `init1.for:179-181,350-351`
   applies. Functionally recovered at uptake
   (`effective_root_depth = root_depth.min(profile_depth)` in
   `run_plant_root_uptake`), and the residual ledger already flags it for a
   future initial-state package. Non-blocking.

5. High (continuation) — `Ep` magnitude is still wrong and aggregate storage is
   regressing. Activating the pathway moved `Ep` fail-count only
   `56834 -> 56230` (~1%, mean abs `1.739 -> 1.683`) and pushed
   `Total-Soil`/`SoilWaterTotal` worse (`mean abs 131.29 -> 168.13`). The
   pattern across HPHYS0249 -> HPHYS0250 is consistent: each more-correct WB17
   withdrawal worsens aggregate storage. This implies the storage base
   (snow/runoff water inputs) is the dominant defect and ET correctness alone
   cannot close `Ep` or storage. The next target should be the
   snow/runoff/storage input lineage, not further WB17 ET refinement.
   Evidence: `full-39-suite-metrics.md` / `residual-ledger.md`
   (`Ep 0/39 56230`, `Total-Soil/SoilWaterTotal` regression note "now affected
   by real Ep withdrawal").

## Notes for disposition owner (Codex)

- Findings 1-2 are auditability/disclosure gaps, not physics errors: name the
  growth constants in `constants.rs` with `init1`/`initgr` references and
  disclose the `0.999` clamp. Both are localized.
- Finding 3 is a process observation; the implementation is correct, but the
  contract-first evidence chain would be stronger with a behavioral
  pre-implementation failure for the PL-activation/live-canopy lineage.
- Finding 4 is already tracked; monitor only.
- Finding 5 is the highest-value continuation signal and is consistent with the
  package's own honest `HOLD`. The cross-package trend (correct ET worsening
  storage) is the strongest evidence that the next package should target the
  snow/runoff/storage inputs rather than WB17.
- Disposition `HOLD` at `0/39` is consistent with this review; no overclaim
  observed.

## Codex Disposition

Status: complete

Evidence mode: static + ran

Disposition owner: Codex

Date: 2026-06-02

### Finding 1 - Growth Constants Inline

Disposition: fixed.

Static:

- `apply_primary_initial_live_canopy_assimilation` now uses named constants
  from `constants.rs` for annual/perennial LAI coefficients and canopy-cover
  maximum.
- Relevant constants: `PL_GROWTH_ANNUAL_LAI_A`,
  `PL_GROWTH_ANNUAL_LAI_B`, `PL_GROWTH_PERENNIAL_LAI_A`,
  `PL_GROWTH_PERENNIAL_LAI_B`, and `PL_GROWTH_CANCOV_MAX`.

### Finding 2 - Undisclosed `cancov >= 0.999` Cap

Disposition: fixed.

Static:

- `SC-PLANT-001` is updated to version `17`.
- The contract now states that `CANCOV_MAX=0.999` is an openWEPP
  finite-domain guard for initial live-canopy assimilation because baseline
  `initgr.for` does not guard the `cancov=1.0` logarithm singularity.
- The contract constants table now labels the cap as a numeric deviation from
  unguarded baseline singularity handling and cites
  `REF-PLANT-LEGACY-INITGR`, `REF-PLANT-LEGACY-GROW`, and
  `REF-PLANT-PHYS-BOUNDS`.

### Finding 3 - Weak Contract-First Red

Disposition: acknowledged; no retroactive evidence rewrite.

Static:

- The review is correct that the HPHYS0250 pre-implementation red evidence was
  weaker than ideal for behavioral PL activation/live-canopy vectors.
- HPHYS0251 subsequently used stronger contract-first red/green evidence for
  the continuation SWU lineage; this does not change HPHYS0250 history.

### Finding 4 - `solthk` Root-Depth Cap at Seed Time

Disposition: monitor / carry-forward.

Static:

- No production change made in this disposition pass.
- The runtime root uptake path still caps effective root depth against profile
  depth before uptake.
- Seed-time initial-state parity remains appropriate for a future
  initial-state package if needed.

### Finding 5 - `Ep` Magnitude and Aggregate Storage Regression

Disposition: carried forward and sharpened by HPHYS0251.

Ran:

- HPHYS0251 full-suite metrics remained `0/39` semantic pass.
- `Ep` mean abs diff mean moved from `1.68341` to `1.70276`.
- `Total-Soil`/`SoilWaterTotal` mean abs diff mean moved from `168.131` to
  `170.349`.

Static:

- The continuation recommendation is now storage-lineage focused rather than
  further WB17/SWU tuning: diagnose `wb18_perc_theta_####`,
  `wb11_soil_water`, `watcon`, WB18/WB19 mutation timing, and WB13 aggregate
  publication before post-WB19 root uptake.

### Validation

Ran:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
