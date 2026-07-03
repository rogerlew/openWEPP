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
