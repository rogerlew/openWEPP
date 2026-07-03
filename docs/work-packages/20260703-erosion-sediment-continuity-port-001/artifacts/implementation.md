# Increment-1 — Implementation Record (single-OFE Wave-1 sediment-continuity solve)

Author: Claude Code (ADR-0035 executor exception), 2026-07-03.
Branch: `worktree-erosion-inc1-wave1-continuity` (not merged to `main`;
awaiting Codex review per the WP non-negotiables).

Evidence classes are labeled per claim: **Static** = read/derived from
source; **Ran** = command executed in this session on this branch.

## What landed

1. **`crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs`**
   (new, 1,936 lines — under the 2,000-line WARN; split intent for
   Increment-2 noted below): the normalized-space single-OFE Wave-1
   sediment-continuity solver, a source-intent port (ADR-0024, baseline
   `dac3c950`) of:
   - `runge.for` — classic RK4 over `dG/dx = Dc·(1 − G/Tc) + θ`
     (`Tc ≤ 0 → θ`), with `k3` reusing `k2`'s shear/`Dc`/`Tc` (same
     midpoint) and the mandatory interrill floor
     `ldnew ≥ ldold + θ·dx` (`runge.for:219`). The `/detcom/` memo is
     reproduced by stateless recomputation — numerically identical
     because `shr`/`Dc` are pure functions of `(xterm, η, τc)`.
   - `erod.for` — the 101-point detachment march (first sub-point
     `dx = x_i − xb`), the case-4 flow-end branch, the `kflag`/`ldrat`
     deposition trigger, the segment-end tail integration, and the
     ≤10-iteration `cross.for` secant solve for the deposition onset
     `xdbeg`, including the `kflag = 5` degenerate-top bracket.
   - `xcrit.for`/`root.for` — all five `mshear` regimes with
     **un-clamped `tauchk`** (the baseline comments the `≥ 0` clamp out
     at `xcrit.for:82`; the deleted `erod19` port's added clamp is
     reconciled back to the baseline — see the falling-cross unit test
     that fails under a clamped `tauchk`).
   - `depc.for`/`depeqs.for`/`depend.for`/`depos.for` — the analytic
     deposition region `G = Tc − D(x)·(x + q*)/φ` with `undflo.for`
     underflow guards, the ≤10-iteration Newton solve for `xdend`
     (increasing- and decreasing-flow branches, `kkkk/xmin` fallback),
     and the grid writer with the `θ ≤ 0` monotonic guard and `G ≥ 0`
     clamp.
   - `route.for` — the per-segment control flow: upper-boundary
     deposition-rate estimate (`|q*| < 0.0011` branch), deposition-at-top
     (`depc → depend → depos` + post-deposition `mshear` dispatch from
     `xdend`), detachment-at-top `mshear` dispatch (`η` only on
     above-critical sub-intervals, `0.0` below), the post-detachment
     deposition tail from `xdbeg`, and the case-4 segment bypass.
   - `param.for`/`xinflo.for` — the normalization layer:
     `η = cntlen·kr·kradjf·shrsol/tcend`, `τcn = tcadjf·shcrit/shrsol`,
     `θ = cntlen·detinr/tcend·(effdrr/effdrn)` (zeroed when
     `qout ≤ qin` or suppressed), `φ = β·Vf/pkro` clamped ±100000,
     `qostar` (all `xinflo` branches incl. the `−1.001` displacement and
     decreasing-flow floor), per-event normalized segment coefficients
     `(a,b,c)/(atc,btc,ctc)` from the static `profil.for` slope fit, the
     `qout = peakro·efflen` discharge, and the frozen-surface /
     θ-suppression activation semantics as explicit typed input flags.
   - `sloss.for` — denormalization
     `dslod = G·effdrn·tcend·width/rspace` (kg per m of width), event
     totals, toe concentration `exported/(runoff·efflen)`, and the
     interrill `irdgdx` surface.
   - `profil.for` — `derive_wave1_slope_segments` (normalized
     slope-segment linear fit), the production seed derivation for the
     static geometry payload.
   - `contin.for:977` — the `passby` event-size gate
     (`runoff ≤ 0.010 m` **and** `peakro ≤ 2.78e-6 m/s` → no sediment
     routing) plus the runoff-day activation gate (`norun` semantics).

2. **Conservation gates (hard, fail-closed typed errors):**
   - *Publication closure* (INV-SED-010 consistency): the telescoping
     identity `exported − inflow = Σdetachment − Σdeposition`, where the
     totals are the signed per-cell `ΔG` sums (legacy `dslost`
     semantics). Tolerance `1e-9` relative — the identity is exact up to
     float accumulation order.
   - *Continuity flux closure* (INV-SED-001): per-cell
     `|ΔG − trapz(detach + θ)|` over unclamped same-region cells,
     hard-gated at `1e-3` relative to `Σ|ΔG|` (the named
     RK4-vs-trapezoid discretization bound) and **always reported** in
     the state (`flux_closure_residual`/`flux_closure_scale`), per the
     TOL-SED-001 residual-reporting posture.
   - Pointwise invariants: `Dc ≥ 0` with `Dc = 0` when `τf ≤ τc`
     (INV-SED-002), `Tc ≥ 0` clamps + `tcadjf ≥ 0.30` operand guard
     (INV-SED-006), finite normalization denominators (INV-SED-007),
     nonnegative loads, fail-closed operand validation (a zeroed
     positive-required operand is a missing operand).

3. **Runtime wiring (shadow-first):**
   `DirectErosionInputs.wave1_continuity` (typed payload,
   `enabled = false` in `zero()`), `DirectErosionState.wave1_continuity`,
   the solve in `compute_r7d6_erosion`, runoff authority in
   `r7d8_erosion_inputs_with_runoff_authority` (`peakro`, runoff depth,
   and `effdrn = runoff/peakro` — the WB16 `runoff_duration_s` surface,
   `irs.for:725`), the Wave-1 publication projection (INV-SED-010 totals
   + total toe concentration; per-class array zeros deferred to
   Increment-3 per the handoff), and `publication_authority` extended to
   Wave-1-continuity-enabled lanes. The existing EROD13 pointwise
   validator and the EROD14 multi-OFE router are untouched.

## Scope deviation (prominent): production enable held at a declared boundary

The handoff (§3f) said the Wave-1 raw operands "already exist on
`DirectErod13Inputs` — the projection exists." **Static finding:** only
the *struct fields* exist. The production seed constructs
`DirectErod13Inputs::zero()` (`00_builders_and_authority.rs`), r7d8 wires
only `q_runoff/peakro/watdur`, and **no producer anywhere in openWEPP**
derives the remaining operands. The legacy producers are:

| Operand cluster | Legacy producer | openWEPP status |
|---|---|---|
| `shrsol`/`shrend` (rill shear), rill `width`, friction factors | `frcfac.for` + `shears.for` | absent (SC-SED-001 `EROD-BND-002` runtime owner explicitly deferred under the erosion-physics HOLD) |
| `kiadjf`/`kradjf`/`tcadjf` daily adjustments | `soil.for:990-1170` (residue/root/consolidation/freeze-thaw state) | absent — a subsystem-scale port |
| `effint`/`effdrr` (rainfall-excess intensity/duration) | `irs.for` | computed internally by WB16 but not exported |
| `kt`/`kt2`/`ktrato`/`tcend`, `veleff` | `prtcmp.for`/`falvel.for`/`yalin.for`/`trcoef.for` | absent |
| `detinr` (interrill rate + delivery ratio) | `param.for:463-518` | absent |

Enabling Wave-1 in the production seed without these would require
fabricated operands — forbidden ("no provisional/proxy sediment math in
the production path", root `AGENTS.md` fail-closed rules). Fail-closing
on them instead would hard-error every runoff day of every single-OFE
run — a regression of the whole green suite.

**Disposition:** the solver, wiring, activation semantics, publication
path, and conservation gates are fully live and exercised through the
real runtime span (tests drive `run_r7d6_erosion_span`, the actual
production consumer). The production seed keeps
`wave1_enabled = false` + `DirectWave1ContinuityInputs::zero()` with the
gap documented at the seed site (`00_builders_and_authority.rs`).
**Operand production is Increment-1b** — flipping the seed to a populated
payload activates the solver with no further orchestrator changes.
Production outputs are unchanged this increment (the DFF-WS3 sediment
HOLD assertions still pass in the full suite — Ran).

Also recorded for Increment-1b: the pass-parquet row builder
(`build_hillslope_pass_row_from_direct_publication`) currently hardcodes
`tdet/tdep/sedcon` to zero (the WS-3 hold at the writer level). The HBP
`EVENT` payload builder already consumes
`erosion.hbp_total_detachment_kg`/`hbp_total_deposition_kg` from the
publication rows, which the Wave-1 projection now populates. The writer
unhardcode must distinguish Wave-1-continuity-sourced totals from the
Wave-2 placeholder-seeded router totals (which must NOT publish).

## Source-intent fidelity notes (traps + reconciliations)

Handled per the handoff §6 list — all **Static** against the baseline:
1. RK4 `k3` reuses `k2`'s `Dc/Tc` (structural, same midpoint values).
2. Interrill floor implemented and unit-tested (exact clamp equality).
3. `tauchk` un-clamped; unit test proves the clamped variant would
   misplace the falling-cross crossing on a negative-`tauchk` profile.
4. `mshear` dispatch passes `η` only on above-critical sub-intervals —
   both the detachment-at-top and detachment-after-deposition variants.
5. Whole solve in normalized `x ∈ [0,1]`; denormalization only in the
   `sloss` totals layer.
6. `veleff`/`kt`/`kt2` traced: **no production producer exists** (the
   Increment-1b finding above); typed required operands, fail-closed.

Deliberate, documented deviations from legacy (all defined-behavior
replacements for legacy UB/quirks, none reachable on the single-OFE
`qostar ≥ 0` path):
- `depeqs.for:56` mutates the caller's grid point (`xinput(i)`) by
  reference at the flow-end singularity; the port shifts the evaluation
  point locally without mutating the grid (unreachable for
  `qostar ≥ 0`; revisit with the Increment-2 decreasing-flow scope).
- `erod.for`/`depos.for` with `ibeg = 102` read/write out of bounds in
  Fortran (adjacent-plane memory); the port no-ops (dl/ldlast keep their
  onset-iteration values) — fail-safe defined behavior.
- `xcrit.for` leaves `xc1` stale when the rising-convex root falls
  outside the segment (mathematically unreachable); the port defaults to
  the segment start.
- `root.for` would NaN on a negative discriminant; the port returns a
  typed `DirectDomainViolation` (fail-closed; unreachable domain).
- The dead `kflag = 1` store at `erod.for:344` is dropped with a comment
  (every continuing path re-flags `kflag = 2`).
- `runge.for`'s `exp(0.666667·log x)` and `shear.for`'s
  `x^0.66666667` (+ 0.0001 floor) are kept as **two distinct**
  shear evaluations, exactly as legacy.
- The legacy M2-IF onset bracket pairs the rewound `xlast` with the
  load computed at `xe` (position/load mismatch in the first secant
  iterate, `erod.for:373-384`); reproduced as-is.

## Validation

**Ran** on this branch (worktree off `main` @ `3af193ad`):
- `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean on
  every touched/new file. Five pre-existing failures remain in
  `tests/integration/dff_ws3_directional_burn_validation.rs`
  (float `assert_eq` + case-sensitive-extension), **verified present on
  `main`** (Ran: clippy on the main checkout) and left untouched per
  write-set discipline.
- `cargo nextest run --workspace --profile full` — result recorded below
  in "Full-suite gate".
- `cargo deny check` — advisories/bans/licenses/sources ok.
- `git diff --check` — clean.
- `bash tools/release/check_authority_suite_antievasion.sh --base-ref
  main --head-ref HEAD` — PASS.
- SC-unit lint — runs inside the full nextest profile
  (`hphys0279_sc_unit_compliance_lint_contract`).

**Unit tests** (Ran, 13/13 green,
`tests_mod/direct_runtime_wave1_continuity.rs`): RK4 vs analytic
constant-coefficient solution (≤1e-9 over the 101-point march); interrill
floor exact clamp; all five `mshear` regimes with hand-solved crossings;
un-clamped-`tauchk` falling cross; `depc`→`depeqs` onset-rate recovery;
`depend` zero-crossing within the legacy residual bound; full-driver
detachment→deposition transition on a crafted concave profile with
hand-checked `param.for` normalization (η=0.2, τcn=0.25, θ=0.005,
φ=1000); conservation round-trip (both gates + reported residuals);
activation gating (no-runoff, `passby`, disabled); fail-closed operands
(missing `ktrato`, `tcadjf < 0.30`, non-tiling segments); `profil.for`
segment-fit; frame-level span test (real `run_r7d6_erosion_span`
consumer publishes nonzero totals through the shadow projection);
missing-peak-upstream typed error.

**Fixture-forcing integration test** (Ran, green,
`tests/integration/erod16_wave1_continuity_fixture_conservation.rs`):
runs `forest_high_severity_clay_loam` (single-OFE, **McKenzie Bridge
OR** — the handoff-required class; NOT the erosion-inert MORAN-WY p313)
end-to-end through `openwepp-cli-hill`, reads every storm day above the
legacy `passby` gate back from the published pass parquet, and solves
each with operands built from the fixture's own soil (`ki = 1.5e6`,
`kr = 6e-5 s/m`, `shcrit = 0.5 Pa`, clay-loam texture → `prtcmp`
particle classes), slope profile (`profil` fit, 43% average grade), and
real per-day `runvol`/`peakro` forcing. Gate assertions: every storm day
solves fail-closed-clean; the conservation identity holds on every
active day; **the fixture generates nonzero detachment and exports
sediment at the toe** (the McKenzie-class activation proof). Magnitudes
are not asserted (ADR-0017).

The test's operand-construction chain is a **test-harness Static port**
of `prtcmp`/`falvel`/`shield`/`yalin`/`trcoef`/`shears` with four
labeled assumptions (adjustment factors at the `inidat.for:424`
initialization value 1.0; bare-burn `frcsol = frctrl = 1.11`;
`rspace = 1 m` + Gilley width; `effdrr = effdrn`, `effint = qi`). These
live only in the test file; the production producers are Increment-1b
scope and will be ported from the `.for` sources directly.

## Full-suite gate

`cargo nextest run --workspace --profile full` — **Ran twice.** The
first run failed exactly one test:
`r7b_constructor_type_size_layout_is_bounded`
(`DirectDayConstructorInputs <= 4096` — the inline
`DirectWave1ContinuityInputs` payload added ~224 B to the constructor).
Fix: the payload is `Box`ed on `DirectErosionInputs` and the state is
`Option<Box<DirectWave1ContinuityState>>` (+8 B each at frame scope; the
box is only cloned on days where an erosion wave is enabled, because the
pre-r7d8 flag check short-circuits first — single-OFE production days
never clone it). The final full-suite result is recorded at the end of
this file.

## Increment-1b queue (operand production — next entry gate)

1. `prtcmp`/`falvel`/`yalin`/`trcoef` production port (particle classes,
   `veleff`, `kt/kt2/ktrato/tcend`) — soil-texture inputs already parsed.
2. `frcfac` + `shears` (+ `qshear = qout·rspace` from `xinflo`) — needs
   the cover/roughness surfaces (`rilcov`, `rrc`, `canhgt` — already on
   the PL projection) and closes `EROD-BND-002`.
3. `irs` `effint`/`effdrr` exposure from the WB14/WB16 hyetograph
   machinery (effdrr already computed internally at `runoff.rs:792`).
4. `soil.for` daily `kiadjf/kradjf/tcadjf` adjustment chain (residue,
   roots, consolidation `produc`, freeze-thaw) — the largest piece;
   candidate for its own increment.
5. `detinr` assembly (`param.for:463-518`, interrill delivery ratio).
6. Seed flip in `direct_production_typed_erosion_authority` + pass-row
   writer unhardcode (Wave-1-sourced totals only) + the DFF-WS3 HOLD
   assertion flip.
7. `erosion_continuity.rs` is at 1,936 lines (WARN at 2,000): split the
   deposition-region kernels into a sibling module when Increment-2
   touches this file.

## Full-suite result (recorded at close)

**Ran:** `cargo nextest run --workspace --profile full` on the final
branch state — **1275/1275 passed, 1 skipped, 568 s** (nextest run
`9656c99b`). All gates green: fmt, clippy (touched surfaces; the five
pre-existing `dff_ws3` failures verified on `main` and left untouched),
nextest full, `cargo deny`, `git diff --check`, authority-suite
anti-evasion, SC-unit lint (in-suite).
