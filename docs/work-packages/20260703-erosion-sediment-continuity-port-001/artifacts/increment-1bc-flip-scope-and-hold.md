# Increment-1b-C — Flip Scope + Hold Assessment

Author: Claude Code, 2026-07-03. Evidence: **Static** (runtime-surface
audit) + **Ran** (producer tests, full suite on the effint/effdrr add).

This records, defect-shaped, exactly what the production seed flip
requires, why it is a distinct large integration rather than a
single-pass add, and where the confirmed hold sits — so the flip can be
executed no-intervention when sequenced.

## What just landed (this branch)

**1. The faithful `effint`/`effdrr` producer** —
`erosion_effective_intensity` in `direct_runtime/erosion_operands.rs`
(`reid.for`/`grna.for:607`): `effdrr = durre` (Σ excess-interval
durations), `effint = sumint/durre` (mean **rainfall** intensity over
excess periods, snowmelt intervals excluded from `sumint`). This is the
faithful form of the operand the Increment-1b-A `erod16` test
approximated as `runoff/effdrr` — the approximation substituted the mean
*excess* rate for the mean *rainfall* intensity and understated the
interrill `Di = Ki·I·q` driver. 4 unit tests; the operand producer suite
went 16 → 20.

**2. The 1b-B portable adjustment producers** —
`direct_runtime/erosion_adjustments.rs`: `erosion_consolidation_baselines`
(`scon.for` `kicrat`/`krcrat`/`tccrat`, corrected-`thetfc` lineage) and
`erosion_adjustment_factors` (the `soil.for` cropland subfactor chain →
`kiadjf`/`kradjf`/`tcadjf`: cover/root/residue, sealing via `produc`,
slope factor, `0.03` floors, `2.0` cap). Pure, typed, fail-closed; 9 unit
tests including the consolidation-toward-baselines trajectory and the
floor/cap. The **actively-thawing** (`ifrost == 2`) branch is
**fail-closed** — a typed `MissingDirectUpstream` naming the winter
`fcycle` counter — implementing the recommended safe design directly in
the producer.

Codex review round 1 (Medium, fixed on this branch):
`erosion_consolidation_baselines` was not fully fail-closed at its
producer boundary — the `ki`/`kr`/`shcrit` `<= 0.0` check missed NaN
(which would divide to a NaN ratio), and the texture / `thetfc` / `rfg`
inputs were only finite-checked, so out-of-range values were silently
absorbed by the `scon` clamps. Now: the fraction inputs are
domain-validated to `[0, 1]`, the baseline erodibilities are validated
finite **and** strictly positive, and the three output ratios are
re-validated finite. Regressions added for NaN `ki`/`kr`/`shcrit` and the
out-of-range texture/`rfg` domains (adjustment producer suite 8 → 9).

Both are pure producers **not yet consumed** — like the rest of 1b-A/1b-B
they sit behind the disabled seed. Making them observable IS the flip.
This is why the `effint`/`effdrr` "export" is coupled to 1b-C, not
separable: AGENTS.md producer-only evidence cannot close a publication
claim. The stateful parts of 1b-B (the `daydis`/`rfcum` accumulators and
the prior-`ifrost` frost-regime resolution) are runtime wiring, part of
the flip (item 4 below).

## Execution progress (2026-07-03)

**Stage 2 pure stateful logic LANDED** (branch `erosion-inc1c-flip`):
`direct_runtime/erosion_adjustments.rs` —
- `resolve_erosion_frost_regime` + `ErosionIfrostCarry` + `ErosionFrostInputs`
  (`soil.for:866`): resolves the frost regime
  (`Unfrozen`/`FrozenSurface`/`Thawing`) and the new `ifrost` carry from
  the frost/thaw depths, surface-layer water vs field capacity, and the
  prior `ifrost`. The `Thawing` (`ifrost == 2`) regime fail-closes
  downstream (winter `fcycle` block).
- `DirectErosionConsolidationCarry` + `advance_erosion_consolidation`
  (`soil.for`): the persistent `rfcum`/`daydis` carry — `daydis`
  increments when prior `rfcum > 0.01`, `rfcum` accumulates warm-day
  liquid input, tillage scales `daydis` by `(1 - surdis)` and resets
  `rfcum` (forest never tills).
3 unit tests (the three frost branches; accumulate-and-age; tillage
reset). These are the pure state-transition functions Stage 3 threads
into the frame; the `wb14_hourly_rainfall` surface and the frame
threading itself are folded into Stage 3 (they are only meaningful with
their consumer, avoiding a populated-but-unread field). The
Stage-1 rill-width carry is already returned by the assembly.
**Ran:** `cargo nextest run --workspace --profile full` — 1315/1315
(initial state logic). Checkpoint held for focused review before Stage-3
threading; the review is the right sequence for a small state machine
whose bugs hide until amplified across thousands of days.

**Codex review round 1 (2 Medium, fixed on this branch):**
- *Medium — fail-closed:* both `resolve_erosion_frost_regime` and
  `advance_erosion_consolidation` returned directly and silently
  canonicalized NaN via `.max(0.0)` (`f64::NAN.max(0.0) == 0.0`). Both now
  return `Result` and validate finite/nonnegative fields, `ifrost` in
  `0..=2`, and `surdis` in `[0, 1]`. Regressions added.
- *Medium — irrigation split:* `advance_erosion_consolidation` gated the
  whole liquid input on `tave > 0`, which cannot reproduce the legacy
  split (`soil.for:837-845`: sprinkler/none `irsyst <= 1` adds `irdept`
  even when cold; furrow `irsyst == 2` excludes irrigation). The API now
  takes `ErosionRfcumInputs` with separate `precipitation_m` /
  `irrigation_depth_m` / `irrigation_is_furrow`, faithful to the split;
  the forest path passes zero irrigation. Regression asserts the cold
  sprinkler-irrigated and cold-furrow cases.
After the fixes: adjustment producer suite 12 → 15; `cargo nextest run
--workspace --profile full` **1318/1318 passed, 1 skipped**; fmt/clippy
on the touched surfaces and diff-check clean.

**Codex review round 2 (2 Medium, fixed on this branch):** the
fail-closed surface was not fully closed —
- `resolve_erosion_frost_regime` finite-checked but did not reject
  **negative** depths / water / field capacity (a negative depth could
  still pick a plausible regime). Now rejects negatives as a typed
  domain error; regression added for all four fields.
- `DirectErosionConsolidationCarry::seed` still canonicalized an invalid
  initial `daydis` via `.max(0.0)` (NaN / negative `daydi1` → 0). Now
  returns `Result` and validates finite + nonnegative; regression added.
Adjustment producer suite 15 → 16; `cargo nextest run --workspace
--profile full` **1319/1319 passed, 1 skipped**; fmt/clippy on the
touched surfaces and diff-check clean.


**Stage 1 + assembly core LANDED** (branch `erosion-inc1c-flip`):
`direct_runtime/erosion_seed.rs` — `DirectWave1OperandSeed` (per-lane
static operands), `DirectWave1DailyState` (the daily frame surfaces), and
`assemble_wave1_continuity_inputs` (the pure per-day assembly running the
full operand pipeline: rill hydraulics → transport coefficients →
effint/effdrr → interrill delivery → detinr → daily adjustments, thaw
fail-closed). 6 unit tests including the full assembly driven through the
continuity solver to a conserving solve, the faithful-`effint` interrill
driver being live (distinct from the old `runoff/effdrr` proxy), the thaw
fail-closed propagation, the frozen-surface flag, the dry-day activation
gate (below), and the persistent rill-width carry (below). This is the
flip's core; `DirectWave1DailyState` is now the exact spec for what the
Stage-3 runtime population must supply.

**Ran (Stage-1 checkpoint):** `cargo nextest run --workspace --profile
full` — initial core 1310/1310 (4 assembly tests); after the Codex
round-1 fixes below (2 High + 1 Low → 6 assembly tests) **1312/1312
passed, 1 skipped**. `git diff --check` and clippy on the touched
surfaces clean.

**Codex review round 1 (2 High + 1 Low, fixed on this branch):**
- *High — dry-day ordering:* `assemble_wave1_continuity_inputs` computed
  the routed operands (rill hydraulics) before any activation gate, so a
  `peakro = 0` day hard-errored on the zero-width guard — the same
  ordering bug as the Increment-1 round-1 finding, reintroduced at the
  assembly layer. Fixed: the assembly now gates via the shared
  `wave1_day_routes_sediment` **before** any operand computation and
  returns the inert payload on non-routed days (matching the legacy
  `contin.for` gate-before-`param` order); the solver gates inactive.
- *High — persistent rill width:* the assembly passed `width_seed = 0.0`
  every day, dropping the Gilley rill-width state that `shears.for` grows
  monotonically between disturbances; a later smaller storm would shrink
  the width and mis-scale `detinr`/sediment. Fixed: `DirectWave1DailyState`
  carries `rill_width_prior_m`, the assembly seeds `shears` with it and
  returns the grown width for the caller to carry forward (the persistent
  update + disturbance reset is Stage-2 wiring).
- *Low — evidence line:* the 1310/1310 full-suite count is now recorded
  here.

**Remaining (runtime wiring):** (a) build the static seed in
`direct_production_typed_erosion_authority` (thread texture/slope-points/
cover); (b) populate `DirectWave1DailyState` per-day in r7d8 from the
frame surfaces below + the `daydis`/`rfcum` accumulators + the
`wb14_hourly_rainfall` surface; (c) call `assemble_wave1_continuity_inputs`
and enable for single-OFE; (d) pass-writer unhardcode + DFF-WS3 HOLD flip
+ byte-stability diff.

## Stage 3 progress (2026-07-03) — frame threading

**Sub-piece 1: static-seed builder LANDED** (branch `erosion-inc1c-flip`):
`DirectErosionInputs` now carries a Boxed `wave1_operand_seed`
(`DirectWave1OperandSeed`, disabled). `direct_production_wave1_operand_seed`
in the runner builds it on **every** lane from the real parsed inputs —
texture (`sand`/`clay`/`silt`/`orgmat`/`rfg` from the parsed soil, percent
→ fraction) → particle classes + `veleff`, `scon` consolidation baselines
(with the WB11-corrected `thetfc`), normalized slope segments
(`derive_wave1_slope_segments`), geometry (`slplen`/`efflen`/`cntlen`/
`avgslp`/`slpend`), and the `hmax`/`flivmx` cover constants. Because the
builder runs on every direct-production fixture, the full suite is its
integration test (the static operand chain resolves on all production
soils/slopes). Seed stays disabled; production outputs unchanged.

**Two enable-time adjudication items (provisional behind the disabled
seed — they only affect an active solve):**
- `is_cropland = false` (non-cropland interrill branch, `intdr = 1`),
  matching the reviewed `erod16` forest fixture. The legacy
  lanuse-as-cropland nuance ([[reference-legacy-all-landuse-run-as-cropland]])
  is an enable-time science item.
- `field_width_m = 1.0` (unit width; single-hillslope per-width sediment
  reporting) pending the hillslope-geometry width source. Only scales the
  denormalized total mass, not the per-width physics.
Both are flagged for adjudication at the Stage-4 enable, before the
magnitude / byte-stability check.

**Remaining Stage 3:** the persistent carries
(`DirectErosionConsolidationCarry`/`ErosionIfrostCarry`/rill-width) on the
lane frame + daily advance, and the per-day `DirectWave1DailyState`
population in r7d8 (cover/roots/residue from the growth/decomposition
states, frost regime, the `wb14_hourly_rainfall` surface) calling
`assemble_wave1_continuity_inputs`.

## Integration-surface audit (2026-07-03, Static) — de-risks the flip

Concrete findings from reading the frame + seed-authority surfaces, so the
flip is executable no-intervention:

- **Byte-stability risk is LOW.** The erosion span is a pure downstream
  consumer: `run_r7d6_erosion_span` reads runoff/frost/cover state and
  writes only `self.erosion` + `erosion_downstream_operands` +
  `erosion_shadow_projection`. It mutates no shared water/growth state, so
  enabling it cannot perturb non-sediment publication surfaces (wat, water
  balance, runoff). The byte-stability gate should therefore pass by
  construction for non-sediment columns; the shadow-run diff is a
  confirmation, not a debugging tool.
- **Most daily operand inputs are already frame-reachable at erosion-span
  time** (`DirectDayFrame`): `wb14_hourly_excess_m[24]` (hourly excess),
  `frost_runtime_carry` (`dfrost_m`/`dthaw_m` → `surface_frozen` + the
  frost-regime split), `winter_column` (snow depth → `theta_suppressed`
  snow-cover part), the `decomposition`/`residue_partition`/`growth`
  states (cover/root/residue masses → 1b-B adjustment factors), and
  `peak_runoff_shadow_projection` (peakro/runoff). So the per-day assembly
  reads existing frame state — no new plumbing for these.
- **Genuine new plumbing (the real work):**
  1. **Seed-authority signature.** `direct_production_typed_erosion_authority`
     currently receives only the WB11 soil projection (thetfc/thetdr, **no
     texture**), the slope projection (slplen/avgslp, **no points**), and
     peak-runoff. The static seed needs surface-layer **texture/rfg** (from
     the parsed `SoilProfile` or a new texture projection), the **slope
     points** (for `derive_wave1_slope_segments`), and the **cover
     surfaces** (from the PL management projection). Threading these to the
     seed authority is the first change.
  2. **`daydis`/`rfcum` accumulators.** New **persistent daily state** (no
     existing analog carries it) — add to the lane frame, initialize from
     the management `daydi1` (fire-reset for the disturbed fixture),
     update each day (`rfcum += precip+irr` when `tave > 0`;
     `daydis += 1` when `rfcum > 0.01`). Precedent exists (frost/snow
     carries, decomposition mass evolve daily), but it is a real threaded
     accumulator.
  3. **Raw-rainfall surface for `effint`.** `wb14_hourly_excess_m` gives
     the hourly excess, but `effint`'s `sumint` needs the raw **rainfall**
     rate per interval. Add a parallel `wb14_hourly_rainfall_m[24]` (the
     WB14 loop already has `interval.intensity_m_s`; bin it like the
     excess) so `erosion_effective_intensity` can be evaluated from the two
     hourly arrays. Isolated additive surface.
  4. **Frost-regime resolution.** Map `frost_runtime_carry` + the layer-1
     water-vs-`thetfc` test + prior-`ifrost` (new 1-bit carry) to
     `ErosionFrostRegime` (`Unfrozen`/`FrozenSurface`/`Thawing`). The
     `Thawing` case fail-closes (1b-B) — see the hold below.

## Why the flip is a distinct large integration

The Wave-1 operands are **per-OFE-day** (they flow from the daily
`peakro` → `qshear` → `shears` → `kt`/`tcend`, and from daily
excess/cover/frost state). The current `DirectWave1ContinuityInputs` is
seeded once at day-zero with `::zero()`. Flipping the seed requires a new
**per-day operand-production step**:

1. **Static erosion seed** (built once per lane from parsed
   soil/slope/management): texture → particle classes + effective
   particle + `veleff`; `kr`/`ki`/`shcrit`; `rspace`; the normalized
   slope segments (`derive_wave1_slope_segments`); the rill cover
   surfaces (`rilcov`/`canhgt`/`hmax`/`flivmx` — already on the PL
   projection, read by the WB16 ealpha producer); the `scon.for`
   consolidation baselines (`kicrat`/`krcrat`/`tccrat`, corrected-`thetfc`
   lineage). New struct on the erosion seed authority.
2. **Per-day operand computation** (in `r7d8_erosion_inputs_with_runoff_authority`,
   which already runs per-day with `peakro`): call the 1b-A producers
   with the daily `peakro` + the daily excess intervals (for
   `effint`/`effdrr`) + the daily frost/snow/cover state to fill
   `DirectWave1ContinuityInputs` (`shrsol`/`shrend` → `ktrato`/`tcend`,
   `detinr`, the activation flags, and the 1b-B adjustment factors).
3. **effint/effdrr threading** — the `ErosionExcessInterval[]` the
   producer needs is the WB14 per-interval (duration, rainfall_rate,
   excess) triple. WB14 computes per-interval excess in its loop
   (`runoff.rs:1480-1526`) but currently retains only the aggregate +
   hourly bins. The flip must surface the per-interval triples (or the
   computed `effint`/`effdrr`) from WB14 to the erosion inputs — a new
   typed shadow surface on the runoff/peak projection.
4. **1b-B daily adjustment chain** — new **persistent daily state**
   (`rfcum`, `daydis` accumulators; forest = no tillage → `daydis`
   monotone) threaded through the run/lane/day frames, plus the
   `soil.for:820-1170` subfactor math (cover/root/residue from the
   growth/decomposition surfaces, sealing via `produc`, slope factor).
   **Fail-closed on the `ifrost == 2` thaw branch** (the winter hold —
   see below).
5. **Enable** for `contributor_ofe_count == 1` in
   `direct_production_typed_erosion_authority`.
6. **Pass-parquet writer unhardcode** — `tdet`/`tdep`/`sedcon` are hard
   zeros in `build_hillslope_pass_row_from_direct_publication`; unhardcode
   **only for Wave-1-continuity-sourced totals** (the Wave-2
   placeholder-seeded router must not publish).
7. **DFF-WS3 HOLD flip** — the sediment `== 0` assertions become the
   directional ordering law (high burn ≥ unburned; direction only,
   ADR-0017).
8. **Byte-stability gate** — every non-sediment production surface (wat
   parquet, water balance, runoff) must be **byte-identical** to the
   pre-flip baseline across the full 2192-day run. This is the
   highest-risk item and requires a shadow-run diff before the flip
   commit.

Items 1–8 span the runoff producer, the erosion seed authority, the
day-input path, the frame state (new persistent accumulators), the
publication writer, and two integration tests — with a full-run
byte-stability proof. That is a multi-file stateful integration of the
size the entry gate staged as its own increment, not a single-pass add.

## The confirmed HOLD (winter freeze-thaw coupling)

Unchanged from `implementation-1b.md` and re-confirmed: the 1b-B
freeze-thaw factors (`ckiaft`/`ckraft`/`tcaft`) are `1.0` on non-frozen
days and moot on fully-frozen days (the solver's `surface_frozen` path
zeros `eata`/`theta`), but load-bearing on the **actively-thawing**
branch (`ifrost == 2`), which needs the winter `fcycle` freeze-thaw
cycle counter — **0 occurrences in the direct runtime**, produced only in
the legacy winter subsystem (`watdst.for:520`/`wshdrv.for:746`).

**Recommended flip design (safe by construction):** port the portable
adjustment chain (consolidation + cover/root/residue + slope) and
**fail-closed the `ifrost == 2` branch** with a typed error naming the
missing winter `fcycle` producer. Then the flip is correct wherever the
thaw branch is inert (frost-off runs like the McKenzie fixture, and all
non-thawing days) and a loud typed failure — never fabricated 1.0 — where
it is not. The `theta_suppressed` `frara` melt-branch
(`contin.for:855`, `rans/rain`) is the same winter block.

**Suite-safety unknown (the gating risk for closing the flip):** if any
single-OFE frost-on fixture in the suite hits an `ifrost == 2` storm day,
the fail-closed branch will hard-error and the flip cannot close without
the winter `fcycle` producer (a winter-subsystem WP, out of the erosion
write set). This must be checked empirically during the flip: if it
fires, the flip HOLDS at the winter boundary; if not (e.g. all single-OFE
fixtures are frost-off or non-thawing), the flip closes with the
fail-closed guard latent.

## Disposition

The `effint`/`effdrr` faithful producer is landed and gated (extends the
1b-A pure-producer subset). The flip (items 1–8) is the remaining
integration; it is **held** here rather than forced in a single rushed
pass, because (a) the winter `fcycle` coupling is a confirmed hard
boundary inside item 4, and (b) items 2/3/4/8 are a stateful multi-file
integration whose byte-stability gate (item 8) cannot be honestly closed
without a full shadow-run diff. Both are genuine hold criteria under the
"proceed autonomously unless hold criteria is reached" directive.

The flip is now specified no-intervention: a focused effort can execute
items 1–8 in order, with the winter fail-closed guard making the enable
safe by construction and the byte-stability diff as the close gate.
