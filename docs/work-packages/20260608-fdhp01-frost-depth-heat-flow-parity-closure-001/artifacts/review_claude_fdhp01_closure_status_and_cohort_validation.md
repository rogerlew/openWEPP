# Claude Review — FDHP01 closure status and missing cohort validation

Status: executed; findings accepted; package held
Reviewer: Claude Code
Date: 2026-06-11
Evidence mode: Static (read implementation diff, contract v54/v148, package
artifacts, FDMC01/FROSTVAL01 evidence) — no commands run for this review.

## What is solid (no findings)

- Contract-first sequence held: `SC-SNOWFREEZE-001` v54 and `SC-WATBAL-001`
  v148 amended before production code; red/green contract tests landed.
- The mechanism is the right shape: freeze-index proxy and `0.20 m` model cap
  retired; two-sided hourly signed heat flow (`Qsrf`/`Quf`); depth bounded by
  physical profile depth (`solthk`); frozen-water exchange fails closed on
  liquid overdraw and credits thaw back to liquid `wb11_soil_water`; `frdp`
  published to WAT in `mm` (dataset 1.4) per the FDMC01 caveat.
- Dual review functioned: Review A forced the separate `Quf` lower-front model;
  Review B caught the comparator overclaim and schema/version drift. All
  recorded findings were dispositioned.
- Full Rust closure loop reported passed (fmt/clippy/test --workspace/deny).
- Artifact-level truthfulness is good: the ledger and disposition explicitly
  decline to claim FDMC01 cohort acceptance.

## Findings

### F1 (blocking) — `complete` status contradicts the package's own unmet,
### unamended acceptance criteria

`package.md` acceptance criteria still require, verbatim: the FDMC01 gap
"materially closes — depth reaches the physical range (cap retired),
depth-series correlation rises substantially, and the frozen-duration
over-persistence (+258 days / ratchet) is eliminated", plus water-balance
closure re-verified and FQ-4 activation non-regressed. The only edit to
`package.md` was `Status: scaffolded` → `Status: complete`. None of the three
cohort-level criteria were measured. Contract tests prove the mechanism; they
do not measure depth/duration tracking on real forcing. The Branch-out rule
permits a phased close **only if** the landed phase still closes the FDMC01
depth+duration gap — currently unestablished, so the phase boundary's
legitimacy is unestablished.

### F2 (blocking) — the "unavailable evidence" rationale does not hold

"No declared FDMC01 owcmp manifest exists" conflates *cannot rerun the exact
suite* with *cannot measure the cohort*:

- FDMC01 itself measured the cohort without owcmp; its legacy depth series
  (`frost_depth_timeseries_pairs.csv`) and metrics are on disk — the legacy
  side needs **no rerun**.
- The FROSTVAL01 2026-06-11 rerun demonstrated the 43-prefix population
  harness is cheap (`/tmp/frostval01_rerun_20260611T020951Z`).
- FDHP01 now publishes `frdp` directly, making the openWEPP side *easier* to
  measure than FDMC01 had it (no runtime reconstruction).

DC HOLD legitimacy requires evidence to be genuinely unavailable, documented
with evidence. This evidence is available.

### F3 (blocking, highest risk) — conservation closure under the new depth
### model is unmeasured on the substrate

The change materially alters `frozwt` magnitude/timing and introduces a new
liquid↔frozen storage exchange. The package requires the rung-1 identity
(incl. `frozwt`) + totalwatsed3 audit to still close. `cargo test --workspace`
covers synthetic fixtures, not the 43-prefix annual identity. The FROSTVAL01
rerun provides an exact pre-change baseline (max abs residual
`3.2173375075217336e-11 mm`). If the new exchange breaks closure, it would
otherwise surface mid-MOFE aliased into routing error — exactly what the
re-sequence exists to prevent.

### F4 — closing obligations skipped

- `docs/ROADMAP.md` queue item 1 not removed (canonical queue still says
  FDHP01 is next while the package says complete — record is
  self-contradictory).
- `docs/work-packages/README.md` execution-log entry (7f) not updated.
- `worker-handoff.md` does not name MOFE as the next ROADMAP item (milestone 7
  required it).

## Required to clear the findings

1. Build the release `openwepp-cli-hill` from the working tree and run the
   frost-on 43-prefix `algebraic-radium` population (the FROSTVAL01 rerun
   harness shape is the template).
2. Measure and record in `fdhp01-frost-depth-validation-ledger.md`:
   a. annual closure residuals vs the `3.2e-11` baseline (rung-1 identity);
   b. activation non-regression (43/43 `frsoil.active`, nonzero `frozwt`);
   c. published `frdp` depth/duration statistics vs the stored FDMC01 legacy
      series (cap retired in practice, correlation movement from 0.13,
      duration over-persistence eliminated) — comparator as flag (ADR-0017),
      envelope not millimetre match.
3. Disposition on the evidence: if criteria hold, `complete` stands and the
   F4 closing docs land (ROADMAP item removal, README 7f, handoff naming
   MOFE); if not, the package re-enters defect closure with the measured
   divergence — that is the package doing its job.
4. Either way: the closure must land as a commit; the work currently exists
   only in the uncommitted working tree.

## Recommendation — install the frost-depth comparison as a declared owcmp suite

Review B's finding 1 root cause is reproducibility infrastructure, not
physics: the FDMC01 comparison was script-ad-hoc, so nobody could rerun it.
Recommend a declared `tools/owcmp/suites/` frost-depth manifest that:

- consumes the published WAT `frdp` column (no runtime reconstruction), and
- compares against the **pinned FDMC01 legacy depth series as a fixture**
  (with provenance), avoiding a live `wepp_260606_hill` rerun — deterministic,
  cheap, and consistent with ADR-0017 (legacy is a characterization flag, not
  a live acceptance oracle).

This makes the FDMC01 metrics re-runnable for this closure, for MOFE-era
non-regression, and for Stage-2 magnitude work.

## Execution Result — 2026-06-11

Ran:

- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: pass.
- Fresh frost-on `algebraic-radium` `p1..p43` cohort:
  `/tmp/fdhp01_closure_20260611T041333Z`.
- Initial compact reports were written for this run; the artifact report
  filenames were later superseded by the D1-restored run described below:
  `fdhp01_closure_summary_20260611.json`,
  `fdhp01_run_status_20260611.tsv`,
  `fdhp01_activation_summary_20260611.csv`,
  `fdhp01_annual_closure_residuals_20260611.csv`, and
  `fdhp01_depth_metrics_20260611.csv`.

Disposition:

- F1/F2/F3 accepted. FDHP01 cannot remain `complete`.
- Cohort run result: `42/43` clean exits; `p2` failed before WAT publication
  with `HKERNEL-WB11-PERC-E-003` on `1990-308`.
- Emitted-prefix annual closure max abs residual:
  `75.43917280313423 mm`.
- Emitted-prefix depth evidence overreaches the legacy envelope: max-depth mean
  `1782.2670980346527 mm`; median depth correlation `-0.10301692862035305`.
- `package.md`, `disposition.md`, `worker-handoff.md`, `docs/work-packages/README.md`,
  and `SC-SNOWFREEZE-001` were updated to executed-hold / active-defect
  posture. ROADMAP item 1 remains active; MOFE must not advance.

## Addendum — Claude root-cause attribution of the cohort failures (2026-06-11)

Evidence mode: Ran — duckdb over the fresh cohort WAT parquet at
`/tmp/fdhp01_closure_20260611T041333Z/outputs/` (p1, p8, p20, p32, p39
re-derivations of the annual identity).

The HOLD verdict bundles what are actually **three separable defects**:

### D1 — the conservation break is frozen-storage double-counting in published
### `SoilWaterTotal`, not a flux leak (high confidence, measured)

The annual residual tracks `−Δfrozwt` almost exactly. Re-deriving the identity
with storage = `SoilWaterTotal − frozwt` collapses the max abs residual
~30× on every prefix tested:

| prefix | storage=`SWT` | storage=`SWT − frozwt` |
|---|---|---|
| p1 | 62.546 mm | 2.448 mm |
| p8 | 70.038 mm | 2.467 mm |
| p20 | 72.911 mm | 2.472 mm |
| p32 | 72.844 mm | 2.473 mm |
| p39 | 64.901 mm | 2.444 mm |

(Adding `frozwt` instead *doubles* the residual — confirming the published
storage already over-counts by exactly the frozen store.) Under the pre-FDHP01
model the same plain-`SWT` identity closed at `3.2e-11` with nonzero `frozwt`
(FROSTVAL01 rerun), so frozen water was previously counted exactly once. The
new liquid↔frozen exchange added a second copy: either the freeze withdrawal
debits a pool that is not the one summed into `SoilWaterTotal`, or `frozwt` is
added on top of layer pools that were never debited. **Localization of the
exact line is the follow-on's first task; the magnitude (~96% of the 75 mm
residual) is this one seam.**

### D2 — uniform ~2.45 mm secondary residual (measured, unattributed)

After the D1 correction a residual of 2.44–2.47 mm remains, near-identical
across all five prefixes (same climate). At a `1e-11` baseline this is a real
second defect, plausibly an annual-boundary or guard-path term in the same
exchange — much smaller, but must not be absorbed into D1's fix silently.

### D3 — frost-depth runaway to the physical bound (measured symptom,
### hypothesis-level cause)

`open_max_depth_mm` is 1780.6–1783.4 across all 42 emitted prefixes —
effectively constant at the `solthk` profile bound. The freeze front runs to
the bottom of the profile on every hillslope and pins there (hence the
negative depth correlation: openWEPP sits at a ceiling while legacy
oscillates 240–503 mm). Hypothesis (unverified, for the follow-on): the
heat-flow depth increment lacks the growing thermal resistance of the frozen
layer and/or latent-heat content scaling that bounds front advance in the
legacy `frostn` formulation — the signed balance almost always nets to
freezing. Note the day-scale thaw works (frozen-days delta is now −27.6 vs
legacy, the +258-day ratchet over-persistence is gone) — the defect is depth
progression, not thaw dispatch.

`p2`'s `HKERNEL-WB11-PERC-E-003` (J308) fail-closed is unattributed; given
D1's mis-wired debit it plausibly belongs to the same exchange seam and should
be re-tested after D1 lands rather than chased independently first.

### Implication for the follow-on shape

D1+D2 are conservation/accounting defects in one seam (cheap, hard-gate
class); D3 is the genuine heat-flow physics defect (the actual rung). Fix and
re-validate D1/D2 first so the closure gate is trustworthy again, then close
D3 against the gate. Do not tune D3 to legacy depth numbers (ADR-0017);
the acceptance remains the contract heat-flow envelope.

## Execution Result — D1 retained, D2 rejected (2026-06-11)

Ran:

- Implemented D1 by correcting WAT `SoilWaterTotal` to remain the
  hydout-equivalent `Total-Soil` alias; `frozwt` remains separately published.
- Updated `SC-WATBAL-001` to v149 and updated WB13/WAT/summary accumulator
  guards/tests to enforce `SoilWaterTotal = Total-Soil`.
- `cargo fmt --check`: pass.
- `cargo test -p openwepp-runner
  hphys0203_wb13_soil_water_total_preserves_watcon_alias -- --nocapture`:
  pass.
- `cargo test -p openwepp-summary-accumulator --lib -- --nocapture`: pass.
- `cargo test --test hphys0203_physics_robustness_contract -- --nocapture`:
  pass.
- `cargo test --test hphys0208_fc_threshold_coupled_residual_contract --
  --nocapture`: pass.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract --
  --nocapture`: pass, 16 tests.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: pass.
- Fresh frost-on `algebraic-radium` `p1..p43` cohort using runfile-sidecar
  overrides:
  `/tmp/fdhp01_closure_after_d1_restored_20260611T053545Z`.

D1 cohort result:

- `42/43` clean exits; `p2` still failed before WAT publication with
  `HKERNEL-WB11-PERC-E-003` on `1990-308`.
- Emitted-prefix annual closure max abs residual improved from
  `75.43917280313423 mm` to `2.4798612273409617 mm`; mean abs residual is
  `0.9738853177643827 mm`.
- Emitted-prefix depth evidence is unchanged and still fails the package
  acceptance gate: max-depth mean `1782.2670980346531 mm`; median depth
  correlation `-0.10301692862035305`.
- The compact artifact report filenames now reflect this D1-restored run:
  `fdhp01_closure_summary_20260611.json`,
  `fdhp01_run_status_20260611.tsv`,
  `fdhp01_activation_summary_20260611.csv`,
  `fdhp01_annual_closure_residuals_20260611.csv`, and
  `fdhp01_depth_metrics_20260611.csv`.

D2 was investigated but not retained. Per-layer WB18 theta/frozen-water and
frozen-depth writeback experiments either introduced a new `p4`
`HKERNEL-WB11-PERC-E-003` failure under the correct cohort harness or
overcorrected annual storage residuals. The worst rejected run,
`/tmp/fdhp01_closure_after_d2_final_20260611T051531Z`, produced only `41/43`
clean exits and max annual residual `294.942039464511 mm`, so the production
diff was backed out to the D1-only correction.

Disposition remains HOLD: D1 is accepted as a real improvement, but FDHP01 is
not complete until `p2` runs clean, the remaining `~2.48 mm` closure residual
returns to numerical noise, and D3 depth progression stops pinning at the
profile bound.

## Addendum 2 — Claude D2 localization on the D1-restored cohort (2026-06-11)

Evidence mode: Ran — duckdb day-level analysis over
`/tmp/fdhp01_closure_after_d1_restored_20260611T053545Z/outputs/` (p1, p20;
day-class regression of unexplained storage change `dSWT − flux` against
`Δfrozwt`). Daily WAT semantics carry known timing offsets that cancel
annually (verified on the old-model FROSTVAL01 run, which is daily-nonzero but
annually exact), so per-day-subset *sums* are contaminated; the per-day
*ratios* below are robust.

Measured facts:

1. **Freeze days debit published `SoilWaterTotal` 1:1.** On all 15 days per
   prefix with `Δfrozwt > 1 mm`, `(dSWT − flux)/(−Δfrozwt) = +1.000`
   (mean 0.992–0.994, std ≤ 0.031, both prefixes). When water freezes, the
   published total drops by the full frozen amount beyond flux.
2. **Thaw days do not credit it back symmetrically.** Across ~765–790 thaw
   days the median ratio is `+0.018` (≈ no credit); isolated zero-flux thaw
   events show a *partial* credit ≈ `0.17` of the thawed amount (two clean
   events: 0.171, 0.163).
3. **The annual D2 footprint is the net of this gross asymmetry.** Per-year
   SWT-identity residuals are near-identical across prefixes
   (`+1.81, 0.00, −2.45, −0.86, −0.30, +0.33` for p1/p8/p20/p39 within
   ±0.03 mm), i.e. climate-driven freeze/thaw cycle structure, not
   soil/slope-driven.

Inference (for the follow-on to verify in-process, not established from
outputs alone): the liquid↔frozen exchange is wired asymmetrically — the
freeze branch debits the pool that feeds the published total, while the thaw
branch returns water through a different path (or only partially, ~17%, into
that pool). The ±2.48 mm annual residual is the small net of large
near-cancelling gross errors over the freeze/thaw cycle, which means
**within-year liquid storage is distorted by tens of mm even where the annual
ledger looks nearly closed** — this also contaminates any D3 depth assessment
that depends on liquid water availability at the freezing front. Codex's
rejected writeback experiments (per the execution note above) are consistent
with this being the right seam and the wiring being delicate.

Contract question that must be settled before (or with) the D2 fix: the D1
correction *redefined* published `SoilWaterTotal` (hydout-equivalent
`Total-Soil`, `frozwt` separate). Whether the interchange `Total-Soil` column
semantically **includes or excludes frozen soil water** is an ecosystem
interface contract with wepppy/totalwatsed3 (the rung-1 acceptance surface
consumes it for ΔStorage), not a free implementation choice. Fact 1 shows the
new column behaves liquid-only at freeze; if legacy hydout `Total-Soil`
includes frozen content, downstream totalwatsed3 closure will break by
`Δfrozwt` on legacy-compatible consumers. `SC-WATBAL-001` v149 should pin the
definition with provenance from the legacy `watbal`/`hydout` lineage, and the
totalwatsed3 audit should be rerun against whichever definition is ratified.

### Addendum 2b — legacy provenance for the `Total-Soil`/`frozwt` definition,
### and a reconciliation caveat

Static (legacy source read, pinned baseline):

- `watbalprint.for:56-69` — the WAT line publishes `watcon = Σ soilw(i)`
  (Total-Soil) and `frozwt = Σ soilf(i)` as separate columns.
- `frwatc.for:80-137` (frost→water-balance handoff, `wbtofs=0`) — `soilw(i)`
  is recomputed from the **unfrozen** fine-sublayer water only
  (`slufdp = sublayer depth − frozen depth` gates the sum) plus `nwfrzz`
  (unfrozen water within the frozen zone); sublayer ice goes to
  `frzw(i) = sumice − thetdr·frozen`, excluded from `soilw`.

So in the legacy lineage, **Total-Soil excludes frozen water when frost is
active, and `frozwt` is separate-and-additive** — the v149 form (frozwt
separate) is legacy-faithful, and the physically conserving storage term is
`Total-Soil + frozwt`.

Reconciliation caveat (important, unresolved): on the D1-restored cohort it is
the `SWT`-only identity that closes (±2.5 mm) while `SWT + frozwt` breaks
(±50–70 mm, tracking year-end `Δfrozwt`). If the openWEPP exchange debited and
credited the published liquid pool 1:1 (as the legacy-faithful form implies,
and as the freeze-onset days measurably do), the *additive* identity would be
the closing one. These two output-level facts are mutually inconsistent under
any single simple wiring, which means the exchange/publication structure has
state the WAT surface does not expose. The ±2.5 mm "near-closure" of the
SWT-only ledger may therefore be closing **for the wrong reason**. The D2
follow-on should not trust either ledger variant until in-process diagnostics
(freeze-day and thaw-day pool deltas at the exchange seam) establish which
pools actually move; SC-WATBAL-001 v149 should then pin the Total-Soil
definition with the `frwatc.for` provenance above and the closure identity's
storage term should be ratified to match.

## Execution Result — D2 exchange diagnostics retained (2026-06-11)

Evidence mode: Static + Ran.

Static:

- `SC-WATBAL-001` is now v150. It cites pinned-baseline
  `frwatc.for:80-137` and `watbalprint.for:56-69`, ratifies
  `SoilWaterTotal = Total-Soil` as the unfrozen `watcon` alias, and requires
  frost-active storage audits to evaluate `Total-Soil + frozwt`.
- The active frost exchange seam now publishes in-process diagnostics:
  liquid soil water before/after, frozen water before/after, freeze debit,
  thaw credit, and signed liquid delta.

Ran:

- `cargo fmt` applied formatting.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`:
  pass, 17 tests. New D2 vectors prove freeze-onset debits liquid 1:1 into
  frozen storage and warm-thaw credits reduced frozen storage back to liquid
  at the WB14/WB11 exchange seam.
- `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract --
  --nocapture`: pass, 5 tests after `SC-WATBAL-001` v150 update.
- `cargo test --test hphys0320_stmtim_start_time_source_line_contract --
  --nocapture`: pass, 3 tests after `SC-WATBAL-001` v150 update.
- Final gates after the D2 diagnostics and documentation updates:
  `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo deny check`, and `wctl doc-lint --path docs`
  all passed.

Disposition:

- D2 is no longer "shave 2.48 mm off the ledger." The retained change
  establishes the exchange wiring observability required to judge the ledger.
- The in-process exchange algebra is symmetric in the focused freeze and thaw
  vectors; therefore any remaining WAT-level inconsistency must be audited
  against these diagnostics before production hydrology or WB13 publication
  semantics are changed again.
- FDHP01 remains `executed-hold`: `p2`, the post-D1 `~2.48 mm` cohort
  residual, and the D3 depth runaway remain open downstream gates.

### Addendum 2c — published `frozwt` is depth-derived, not the exchanged store
### (high confidence, measured; resolves the 2b contradiction)

Ran (duckdb, D1-restored cohort, p1): on all 793 frost-active days,
`corr(frozwt, frdp) = 1.0` and `frozwt / frdp = 0.149` with **zero spread**
(p10 = median = p90 = 0.149). Published `frozwt ≡ 0.149 × frdp` — a constant
scale off the (D3-runaway) depth, not an accumulated frozen-water store.
Year-end confirmation: `0.149 × frdp` at year boundaries (305.3, 0, 0, 403.3,
519.6, 297.1, 459.0 mm) reproduces the year-end `frozwt` values (45.5, 0, 0,
60.1, 77.4, 44.3, 68.4 mm) whose deltas are **exactly** the additive-identity
annual residuals measured in Addendum 2 (e.g. −45.5 year 2, +60.1 year 4).

This resolves the 2b contradiction: the in-process exchange can be perfectly
symmetric (per the FDHP01-D2 instrumentation tests) while the *published*
`frozwt` wanders with the defective depth — the `Total-Soil + frozwt` audit
identity breaks by exactly the drift of the fictional store, and the
`SWT`-only ledger's ±2.48 mm is plausibly the true exchanged store's year-end
carryover (i.e. possibly not a defect at all, but expected frozen carryover).

Consequences:

1. The v150-ratified audit term (`Total-Soil + frozwt`) is correct in intent
   but currently audits a fictional quantity. `frozwt` publication must source
   the actual exchanged frozen store (the quantity the new WB14/WB11 seam
   diagnostics track), not `0.149 × frdp`.
2. The constant `0.149` needs provenance: it has zero soil/season dependence,
   so it is a hard-coded scale somewhere in the frost publication path —
   localize and retire (legacy `frozwt = Σ soilf` is a per-layer store, never
   a constant fraction of depth).
3. D2 likely dissolves once publication is fixed: with true-store `frozwt`,
   the additive identity should close to noise and the residual ±2.48 mm
   re-attributes to legitimate frozen carryover. Verify on the cohort.
4. D3 remains the genuine physics defect and now also owns the `frozwt`
   magnitude error (published frozen water is inflated by the runaway depth
   through the 0.149 scale).
5. Open verification for the instrumented binary (single prefix suffices):
   compare the exchange-ledger freeze-debit/thaw-credit sums against
   Δ(published `frozwt`) on real forcing — this directly separates the
   exchange truth from the publication fiction; the Addendum 2 freeze-day
   1:1 SWT debit suggests the freeze side may itself consume the
   depth-derived increment, which the seam diagnostics will now show.

### Addendum 2d — v151 source binding landed, but the diagnostic still aliases
### the depth-derived store

Evidence mode: Static + Ran.

Static:

- `SC-WATBAL-001` is now v151. It binds WAT `frozwt` publication to
  `frost.runtime_frwatc_frozen_water_after_m` and rejects direct publication
  from `frdp`, `frdp * scalar`, or `frost.runtime_ws_frz`.
- WAT assembly now requires `frost.runtime_frwatc_frozen_water_after_m`.
  Missing-symbol publication fails closed with the existing WB13 publication
  error path. Initial frost runtime projection seeds the diagnostic to `0.0`
  so frost-off rows do not depend on a missing symbol.

Ran:

- `cargo test -p openwepp-runner --lib fdhp01_wb13 -- --nocapture`: pass,
  3 tests, including the missing exchange-store symbol guard.
- `cargo test -p openwepp-runner --lib
  hphys0203_wb13_soil_water_total_preserves_watcon_alias -- --nocapture`:
  pass; the fixture sets `runtime_ws_frz !=
  runtime_frwatc_frozen_water_after_m` and proves WAT `frozwt` follows the
  latter.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: pass.
- Fresh 43-prefix cohort run:
  `/tmp/fdhp01_frozwt_publication_20260611T070334Z`.

Cohort result:

- Acceptance remains HOLD: `42/43` prefixes emitted WAT; `p2` still failed
  before WAT publication at `HKERNEL-WB11-PERC-E-003`.
- Emitted-prefix annual closure max abs residual remains
  `2.4798612273409617 mm`, unchanged from the post-D1 floor.
- Published `frozwt` still tracks `frdp` with exact per-prefix scalar ratios
  over `35297` frost-active rows (minimum correlation
  `0.9999999999999994`; p1 ratio `0.149`; median per-prefix median ratio
  `0.15199999999999997`; maximum per-prefix ratio standard deviation
  `3.2273877788806054e-17`).

Disposition:

- The v151 publication source map is correct and test-enforced, but it is
  behaviorally neutral on the cohort because
  `frost.runtime_frwatc_frozen_water_after_m` currently aliases the
  depth-derived store (`dfrost * theta_active` / `runtime_ws_frz`) rather than
  a true independent exchanged frozen-water store.
- The next D2 action is therefore upstream of WAT publication: implement or
  port the true `frwatc`/`soilf` exchanged store behind
  `frost.runtime_frwatc_frozen_water_after_m`, then rerun the additive
  identity. D3 depth runaway and the independent `p2` fail-closed defect remain
  open.
