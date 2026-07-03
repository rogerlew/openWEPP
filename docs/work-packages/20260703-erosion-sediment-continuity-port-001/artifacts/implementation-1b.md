# Increment-1b — Implementation Record (Wave-1 operand production)

Author: Claude Code (ADR-0035 executor exception), 2026-07-03.
Branch: `origin/erosion-inc1b-operand-production` (local worktree branch
`worktree-erosion-inc1b-operand-production`); not merged to `main`,
awaiting Codex review.

Evidence classes are labeled per claim: **Static** = read/derived from
source; **Ran** = command executed in this session on this branch.

## Summary

**The 1b-A pure-producer subset is COMPLETE and gated.** The full 1b-A
stage as scoped in the entry gate is **NOT** complete: it also includes
the `effint`/`effdrr` runtime export and the activation-flag wiring, which
are runtime-surface integration items (not pure producers) and are held —
see "1b-A remaining runtime surfaces (HELD)" below. This is an
**executed-hold of the pure-producer subset**, not a full-stage
completion; the earlier "DONE + GATED" framing was corrected after Codex
review round 1 (High finding: it overclaimed stage completion while those
surfaces were still test-built/deferred).

**Stages 1b-B and 1b-C are BLOCKED at a confirmed, evidence-backed
stop-condition** (the winter freeze-thaw coupling) and are handed back to
review with the boundary characterized, per the handoff's explicit
instruction ("if a stage's gate cannot close … stop at the declared
boundary, record it defect-shaped, and hand off — do not force"). The
production seed enable is therefore **not flipped** this increment; the
pure producers land behind the still-disabled seed, exactly as the 1b-A
gate specifies ("producers exist but the seed stays disabled").

## 1b-A — what landed (pure-producer subset)

New module `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_operands.rs`:
production, typed, fail-closed source-intent ports (ADR-0024, baseline
`dac3c950`) of the Wave-1 event/transport operand chain —

| Producer | Legacy source | Notes |
|---|---|---|
| `erosion_falvel` | `falvel.for` + `inidat.for:1017` drag tables | Stokes branch + table interpolation; overflow uses the last tabulated Reynolds value |
| `erosion_particle_composition` | `prtcmp.for` | 5-class `frac`/`dia`/`spg`, the `jflag` large-aggregate clay re-entry, mm→m at `:333`; per-class fall velocities |
| `erosion_effective_particle` | `param.for:558-579` | 3-class log-mean `diaeff`/`spgeff` → `veleff` |
| `erosion_shield` | `shield.for` | Shields diagram with the legacy mixed linear/log extrapolation |
| `erosion_yalin` | `yalin.for` | class-weighted transport capacity; the sandy `adjtc` floor (INV-SED-006 `≥ 0.30`) lives inside this routine |
| `erosion_trcoef` / `erosion_transport_coefficients` | `trcoef.for` / `param.for:215-234` | `kt`, `kt2`, `ktrato = kt2/kt`, `tcend = kt·shrsol^1.5` (floor 1e-10) |
| `erosion_rill_hydraulics` (frcfac + shears) | `frcfac.for:218-236` + `shears.for` | cropland rill friction (`frcsol = 1.11`, cover/live subfactors); Gilley width growth `1.13·q^0.303` capped at `rspace`; Chezy depth iteration (tol 5e-6, bounded + fail-closed on non-convergence); `shrsol`/`shrend` at the average and end gradients, floored 1e-6 |
| `erosion_interrill_delivery_ratio` | `param.for:412-459` | cropland `rif`/`drinti`/`intdr` model + the non-cropland `intdr = 1` branch |
| `erosion_detinr` | `param.for:463-518` | `ki·kiadjf·effint·(runoff/effdrr)·intdr·rspace/width`; zero on no-width / no-excess-duration |

**Fail-closed posture:** every producer validates its inputs finite
(NaN → typed `NonFiniteDirectValue`) and rejects invalid domains (negative
erodibility/geometry, non-positive shear/transport, out-of-range texture)
as typed `DirectRuntimeError`, never silent defaults. The legacy
exact-zero branches are preserved **only** for exact zeros and are
distinguished from invalid negative/NaN inputs: `erosion_detinr` returns
`0.0` for exact-zero rill width or excess duration (the legacy
`width.gt.0.0` / `effdrr.gt.0.0` else-branches) but errors on
negative/NaN; `erosion_rill_friction` keeps the exact-zero cover/height
branches but errors on negative/NaN cover inputs. (This uniform posture
was completed after Codex review round 1, Medium finding: the first cut
let `erosion_detinr` and the rill-friction helper treat NaN/negative
inputs as the zero case through plain `> 0.0` comparisons; both now
validate-then-branch, with regression tests.)

**Gate — pure-producer subset (Ran):**
- 16 producer unit tests (`tests_mod/direct_runtime_erosion_operands.rs`),
  each hand-checked against the legacy equation on the McKenzie
  clay-loam texture (Stokes closed form to 1e-15, `prtcmp` unity + clay
  fraction, effective-particle log-mean, Shields on-table + extrapolation,
  yalin sandy-adjustment identity, transport `ktrato`/`tcend`, Gilley
  width growth + `rspace` cap, cropland/non-cropland delivery branches,
  detinr product form + exact-zero cases, and the fail-closed NaN/negative
  domain regressions for `detinr` and rill hydraulics). **16/16 green.**
- The `erod16` fixture-forcing conservation test now consumes the
  **production producers** for the pure-producer surface (its Increment-1
  test-harness operand chain is deleted). It runs
  `forest_high_severity_clay_loam` end-to-end through `openwepp-cli-hill`,
  solves every storm day above the `passby` gate with the production
  particle/transport/hydraulics/detinr operands, and asserts conservation
  + nonzero detachment + toe export on the McKenzie storm population.
  **Green (Ran, ~22 s).** It is **not fully through production producers**:
  `effint`/`effdrr` and the daily adjustment factors are still test-built
  (see the held/blocked items) — this test remains an integration probe
  of the pure-producer subset, not a full-stage production gate.

### 1b-A remaining runtime surfaces (HELD — the reason this is a subset)

The entry gate scopes two more items into 1b-A that are **runtime-surface
integrations, not pure producers**, and are held:

- **`effint`/`effdrr` export** — the rainfall-excess mean intensity /
  duration from the WB14/WB16 excess machinery
  (`grna.for:607`/`reid.for` `sumint`/`durre`: `effdrr = Σ` excess-period
  durations, `effint = Σ dur·rate / effdrr`). WB16 computes an excess
  duration internally (`runoff.rs:792`), but `effint` needs the `sumint`
  intensity integral over the WB14 hyetograph excess intervals — a new
  typed shadow surface on the peak-runoff projection, not an existing
  scalar. Held pending that runtime export. In `erod16` these are the
  documented approximation `effdrr = effdrn`, `effint = runoff/effdrr`.
- **Activation flags** — `beta` (rain-today → 0.5, input available),
  `surface_frozen` (`frdp>0 && thdp<=0`, frost depth/thaw available), and
  `theta_suppressed` (snow cover available **+ the `frara` melt-only
  branch, which is winter-subsystem state — same block as 1b-B**). These
  are a per-day wiring step gated by the same runtime integration.

Because these two items are in-scope-1b-A but not delivered, this branch
is an **executed-hold of the 1b-A pure-producer subset**, and the seed
stays disabled. The remaining runtime surfaces (`effint`/`effdrr` export
+ activation-flag wiring) lift when the runtime integration lands —
tracked in `package.md` under 1b-A.

The seed stays `wave1_enabled = false` +
`DirectWave1ContinuityInputs::zero()`; production outputs are unchanged
this increment.

## 1b-B / 1b-C — STOP-CONDITION (winter freeze-thaw coupling)

**Finding (Static, confirmed against the runtime state):** the
`soil.for` daily erodibility chain that produces `kiadjf`/`kradjf`/
`tcadjf` decomposes into a portable majority and one blocked branch.

Decomposition of `soil.for:820-1170`:

| Sub-factor | Inputs | Direct-runtime availability |
|---|---|---|
| Consolidation `ckiasc`/`ckrasc`/`ctcasc` (via `produc = bconsd·daydis`) + the `scon.for` baselines `kicrat`/`krcrat`/`tccrat` (corrected-`thetfc` lineage) | `daydis`, `rfcum` accumulators + texture/`thetfc` | **PORTABLE** — needs two new daily accumulators (`daydis`, `rfcum`); forest managements have no tillage so `daydis` is monotone. `scon` baselines are static and portable. |
| Cover / live-root / dead-root / buried-residue factors (`ckiacc`, `ckiagc`, `ckialr`, `ckiadr`, `ckrbgb`, `ckradr`, `ckralr`, `ctcarr`) | cover, live/dead root mass, buried residue | **PORTABLE** — the growth / decomposition / residue-partition state surfaces carry the mass symbols (audited: `DirectGrowthStateSurface.root_mass_kg_m2`/`live_biomass_kg_m2`, `DirectDecompositionState`/`DirectResiduePartitionState` residue pools). The precise `rtm15`/`rtm(1..3)`/`smrm(1..3)` symbol mapping needs a decomposition-state audit but the surfaces exist. |
| Interrill slope factor `ckiasa` | `avgslp` | **PORTABLE** — trivial. |
| **Freeze-thaw factors `ckiaft`/`ckraft`/`tcaft`** (the `ifrost == 2` actively-thawing branch: matric-potential `tenkpa`, `fcycle` cycle count, `acyc`) | frost/thaw depth (present) **+ `fcycle` freeze-thaw cycle counter + `fgcycl` gate + `froday`** | **BLOCKED** — the frost/thaw depths are on `DirectFrostRuntimeCarry` (`dfrost_m`/`dthaw_m`), but the freeze-thaw **cycle counter** `fcycle`, its gate `fgcycl`, and the frost-day counter `froday` have **zero occurrences in the direct runtime** (Ran: `grep -rn 'fcycle\|fgcycl\|froday' crates/…orchestrator crates/…runner` → 0). Legacy produces them in the **winter subsystem** (`watdst.for:520` increments `fcycle` on thaw-after-freeze; `wshdrv.for:746`/`contin.for:830` reset on `froday > 50`), which is not part of the erosion surface. |

**Why this blocks the flip (1b-C), not just 1b-B:** on non-frozen days
the freeze-thaw factors are 1.0 (`soil.for:867`) and on fully-frozen days
`eata`/`theta` are already zeroed by the solver's `surface_frozen` path,
so the freeze-thaw factors are inert there. They are load-bearing **only
in the actively-thawing regime** (`ifrost == 2`), where erosion can occur
with adjusted erodibility. A general production seed flip that pinned
these factors at 1.0 on thawing days would be **silently wrong winter
erosion physics** — exactly the provisional-math prohibition. The
validation fixture itself is `ksflag = 0` (frost disabled → the branch is
inert for it), but the seed flip is general and would reach cold-climate
single-OFE runs where it is not.

**Recommended resolution (for the next executor / a winter-side WP):**
port the portable adjustment chain (consolidation + cover/root/residue +
slope) as the `kiadjf`/`kradjf`/`tcadjf` producers, and make the
freeze-thaw factor **fail-closed on the `ifrost == 2` branch** — a typed
error naming the missing winter `fcycle` producer — rather than a silent
1.0. That makes the eventual seed flip *safe by construction*: correct
wherever the thaw branch is inert (frost-off runs, all non-thawing days),
and a loud typed failure (not fabricated physics) wherever it is not,
which surfaces the winter-coupling gap as a reviewable runtime boundary.
Surfacing `fcycle`/`fgcycl`/`froday` from the winter subsystem is the
prerequisite and is **out of the erosion port's write set** — it belongs
to a winter/frost work-package.

This is the same class of honest boundary as Increment-1's "the operand
projection is struct-fields-only" finding: the entry-gate flagged the
`fcycle`/`frara` producers as untraced stop-conditions, and the runtime
audit now confirms they are genuinely absent.

(Note: `frara`, the melt-day θ-suppression operand, is
`rans/rain` from the winter partition (`contin.for:855`) — likewise
winter-subsystem state. It gates `theta_suppressed` on melt-only days;
same boundary, same resolution.)

## Validation (Ran, this branch)

- `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean on
  every touched/new surface (`erosion_operands.rs`, its tests, the
  rewritten `erod16` test, module wiring). The five pre-existing
  `dff_ws3_directional_burn_validation.rs` failures remain (verified on
  `main` in the Increment-1 review) and are untouched.
- `cargo nextest run --workspace --profile full` — result appended at
  the end of this file.
- `cargo deny check`, `git diff --check`, authority-suite anti-evasion —
  results appended.

## Line-count governance

`erosion_operands.rs` is well under the 2,000-line WARN. The Wave-1
solver `erosion_continuity.rs` remains at ~1,950 lines — untouched this
increment; its decomposition is still queued for whichever increment next
edits it substantially (the seed-flip stage will).

## Deliverables status

1. 1b-A **pure-producer subset** (particle/transport/hydraulics/delivery/
   detinr) + unit tests + `erod16` production swap — **DONE, gated
   (executed-hold).**
2. 1b-A **remaining runtime surfaces** (`effint`/`effdrr` export +
   activation-flag wiring) — **HELD** pending the WB14/WB16 excess-surface
   runtime integration (the `frara` sub-branch shares the winter block).
3. 1b-B daily adjustment chain — **portable majority specified; blocked
   at the winter freeze-thaw coupling (fail-closed design recommended).**
4. 1b-C activation — **not started; gated behind items 2+3.** Seed
   remains disabled; production outputs unchanged.
5. This record + `package.md` update; branch pushed for Codex review.
   **Not merged to main.**

## Full-suite result (recorded at close)

**Ran (round-1 state):** `cargo nextest run --workspace --profile full` —
1291/1291 passed, 1 skipped, 533 s (nextest run `1612f765`).

**Ran (final, post-Codex-round-1-fixes):** `cargo nextest run --workspace
--profile full` — **1293/1293 passed, 1 skipped, 572 s** (+2 vs round-1:
the two fail-closed NaN/negative regressions for `detinr` and rill
hydraulics). `cargo deny check` (advisories/bans/licenses/sources ok),
`git diff --check` (clean), and the authority-suite anti-evasion check
(PASS) all green. Clippy clean on every touched surface; the five
pre-existing `dff_ws3` failures are untouched (verified on `main`).
