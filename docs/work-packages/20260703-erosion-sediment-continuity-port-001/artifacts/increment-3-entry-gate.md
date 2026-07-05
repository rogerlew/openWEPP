# Increment 3 (ROADMAP §E.4) — Enrichment + Particle Routing: Execution Entry Gate

Author: Claude Code, 2026-07-05. Evidence: **Static** at authoring (legacy
source recon; anchors verified in `wepp-forest_260430_baseline`). Execution
record appends below. Executor: Claude Code (operator: "proceed with
scaffolding and executing E.4"). Branch: `erosion-e4-enrichment` (off main
`e54ecce2`). Merges the Increment-2 §2d particle-class scope per the
ROADMAP ("folds into E.3" — the chain landed first; this rung makes its
deposition class-faithful).

Gate policy (per `docs/standards/local-ci-gate-selection.md`): focused +
`--profile erosion` gates in-loop; `full` at branch-head merge readiness;
p61 / p102 / DFF-WS3 fixtures are in-scope-required.

## 1. What E.4 is

The Wave-1 solve routes TOTAL load with an effective fall velocity
(`veleff`); the class composition today is the labeled un-enriched
approximation (`GAP-SED-007`: detached `frac` split, plus the E.3 D4
mass-weighted inflow blend). Legacy resolves the composition through
`enrich.for`, called by `route.for` at every deposition-region end and once
terminally per OFE. E.4 ports that machinery:

- **do-10 terminal blend** (`enrich.for:131-139`): at each call with
  `ldtop > 1e-5 ∧ qout > 0`,
  `frcflw_i ← (frcflw_i·lddend + frac_i·rillod + fidel_i·intlod)/ldtop`
  with `intlod = θ·(xtop − xdetst)`, `rillod = ldtop − lddend − intlod`
  (floored 0). The E.3 D4 approximation is this blend's
  `fidel = frac` reduction with deposition ignored — superseded.
- **do-30 deposition-region re-proportion** (`enrich.for:153-…`): per-class
  analytic depositional solution over `[xtop, xbot]` — per-class
  `φ_i = β·fall_i/pkro` with `pkro = (qout − qin)/slplen`, per-class
  transport `aa/bb/cc = ktrato·tcf1_i·{a,b,c}inftc(k)`,
  `gu_i = frcflw_i·ldtop`, the `phiflg` singular guards (±1e5 φ caps,
  `qostar` sign fallback), ratio clamp `((xtop+q*)/(xbot+q*)) ≤ 1` for
  `qostar ≥ 0` — new per-class loads at `xbot` → `frcflw` update.
- **iendfg ER** (`enrich.for:140-152`): at the OFE end, the specific-
  surface-area enrichment ratio
  `enrato = Σ_i frcflw_i·SSA_i / SSA_soil + 0.005` with the legacy SSA
  constants (`sand 0.05`, `silt 4`, `clay 20`, `org 1000/1.73`) — the ER
  **diagnostic** (no feedback into routing). `qout ≤ 0 → frcflw = 0`.

Call points (`route.for`): deposition-region ends at `:235`/`:250`
(mid-segment), `:448` (region ending at the segment boundary), and the
terminal `:473` `enrich(k, 1.0, 1.0, xdetst, ldlast, ldlast, lddend, θ,
iendfg=1)`.

## 2. Required producer extensions (recon: Ran static, 2026-07-05)

- `ErosionParticleClass` today carries `dia/spg/frac/fall` ONLY. E.4 adds
  the per-class mineralogy `frcly/frslt/frsnd/frorg` — `prtcmp.for:208-…`
  assigns them per class and our port already derives the intermediates
  (`frcly3`/`frcly4`/`frclyt`) but discards them; expose them per-OFE
  (§4a input authority extends: mineralogy from EACH OFE's own soil).
- Per-class transport fractions `tcf1_i = Ws_i/ΣWs` (`yalin.for:157`):
  our `erosion_yalin` returns only the scalar total — add the per-class
  output (same Yalin evaluation, expose the per-class shares).
- `fidel_i` (interrill-detached composition): non-cropland `fidel = frac`
  (`param.for:452-458`) — exact on the entire enabled scope (no-tillage).
  Carried as an explicit per-class field so the cropland branch is a
  visible extension point, never an implicit assumption.
- SSA of the surface soil (`ssasol`): from the OFE's surface texture +
  organic matter — seed-time per-OFE scalar.

## 3. Design decisions

### D1 — frcflw is solver state, per quantum
The Wave-1 quantum solve gains a per-class flow-composition state
initialized per `route.for:142-160` (inflow present → the intake's exit
fractions; else local `frac`; `qout ≤ 0 → 0`), updated at every
deposition-region end (do-30) and blended at each `enrich` call point
(do-10) using the solver's own `lddend`/`θ`/`xdetst` region bookkeeping.
The publication exit composition and the E.3 intake `exit_fractions`
become the ENRICHED values — the D4 approximation and its labeled
GAP-SED-007 basis are superseded.

### D2 — class-mass conservation becomes enforceable
With per-class loads real, the `INV-SED-008..011` class-conservation
family gets an executable gate: `Σ_i frcflw_i = 1` (unit-sum at every
update, `TOL-SED-005`-class tolerance) and the directional law that a
depositing region shifts the composition toward fines (d50 decreases
monotonically through deposition; coarse classes deplete first —
`φ_coarse > φ_fine`).

### D3 — day/hour aggregation
Each hour quantum exits with its own enriched composition; the DAY exit
composition is the export-mass-weighted blend across quanta.
`GAP-SED-008` NARROWS: the serialized hourly `S_h` stays total-mass
(schema unchanged; a per-class-hourly channel remains a future additive
interchange extension), but the day-level class fractions are now
enriched rather than uniform-detached.

### D4 — ER is a published diagnostic
`enrato` rides the erosion publication operands (`Option<f64>`, exit-lane
day surface) and the HBP/pass surfaces ONLY where a column already
exists — no output-schema additions in E.4.

### D5 — scope boundary
Non-cropland `fidel = frac` is exact for the entire enabled scope.
Cropland `fidel` (the `drinti` interrill-delivery composition) is seeded
fail-closed behind the existing tillage disable — a labeled extension
point, not silent reuse.

## 4. Stage plan

- **4a — producer extensions (pure):** per-class mineralogy exposure in
  `erosion_particle_composition` (against `prtcmp.for:208-…`), per-class
  `tcf1_i` from the Yalin evaluation (`yalin.for:150-160`), `fidel`
  field, per-OFE `ssasol`. Unit tests against legacy equations.
- **4b — solver integration:** frcflw state + the three `enrich` limbs
  (do-10 / do-30 / iendfg-ER) wired at the route region call points;
  unit-sum hard gate; per-quantum exit compositions; day mass-weighted
  aggregation; publication + intake switch to enriched fractions.
- **4c — gates + contracts:** SC-SED-001 amendment (INV-SED-017
  enrichment invariant + ER diagnostic; GAP-SED-007 CLOSED, GAP-SED-008
  narrowed); class-conservation + directional-fining tests (crafted
  depositing quantum: coarse depletes first, d50 decreases, ER > 1);
  p61 (single-OFE enriched exit on depositing days), p102 (chained
  enriched intake fractions), erod16 instrument, DFF-WS3; `--profile
  erosion` in-loop, `full` at branch head.

## 5. Hold criteria

1. The do-30 analytic form cannot be reconciled with our deposition-region
   bookkeeping (region boundaries/qostar basis mismatch) — stop, present.
2. Per-class re-proportion materially breaks the TOTAL-load closure the
   telescoping gate enforces (the class solve must re-normalize to the
   total, never redefine it) — the total remains authority; a material
   conflict is a design stop.
3. Class unit-sum cannot hold within tolerance through chained multi-OFE
   handoffs.
