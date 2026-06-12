# Frost Heave / Migration-Water Heat — Frozen-Fringe Impedance Formulation

## Status

- `state`: **backlog (concept)** — not promotable before the FDHP01 D3 frost
  closure (increment De certification) and the snow density/depth-split
  disposition (F4). Explicitly deferred behind MOFE unless thaw-season
  erodibility evidence promotes it earlier.
- `date`: 2026-06-12 (created, Claude Code)
- `relates`:
  [ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md),
  [ADR-0017](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md),
  `SC-SNOWFREEZE-001` (`INV-SNOWFREEZE-006`, eqn 3.8.4 lineage),
  [frost-depth heat-flow parity](20260607-frost-depth-model-heat-flow-parity.md) (parent arc),
  [snow Stage-2 science review](20260605-snow-code-deferred-science-review.md) (sibling)
- `provenance package`:
  `docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/`
  (staged plan Dd outcome + F5 block; this item extracts the qwet dead-code
  finding into its own future-work record)

## Summary

Legacy WEPP's frost model *documents* a migration-water heat term (`qwet` —
the `L · Kw · (P/Zuf)` middle term of CRM eqn [3.8.4]: capillary water
migrating to the freezing front releases latent heat that must be removed
before the front can advance; the same flux is the frost-heave water source).
The term is **dead code in the pinned baseline**: it was designed in 2008
with an external physics advisor, progressively defanged during the
2008–2010 validation era, and finally disabled by a one-line constant —
without recorded rationale. Code analysis shows the formulation was
*untunable as written* because it omits frozen-fringe impedance. openWEPP
correctly does not port it live (FDHP01 De posture). This item holds the
full context and the contract-first path to a *correct* formulation, for
whenever frost heave / migration physics is judged load-bearing.

## Historical context (the archaeology)

Reconstructed 2026-06-12 from the pinned baseline
(`/workdir/wepp-forest_260430_baseline`, commit `dac3c950…`); no written
rationale exists in the source, the repo history (begins at the 2026
import), dev notes, or located literature.

1. **January 2008** — consultation with Kunio Watanabe (Mie University,
   visiting WSU), recorded in `frzng.for` comments: soil freezing-point
   depression −0.01 to −0.25 °C; via generalized Clausius–Clapeyron
   (`dP/dT = L/(T·ΔV) ≈ −13.1 MPa/°C`), frozen-front pressure potential in
   the range **−20 to −160 m**; "Here, we select to use −100 m."
2. **February 2008** — `frzng.for` authored (Shuhui Dun, WSU; verified
   Joan Wu) with the `qwet` machinery implemented: Saxton–Rawls potential
   and unsaturated conductivity of the layer below the front (`saxfun`),
   Darcy flux toward the front, two visible taming devices (see analysis).
3. **Undated dial-down trail** (comment archaeology, `frzng.for:393-394`):
   documented physics `−100 m` → tried `frzftp = -50` (now commented with
   the `cd` editor marker) → shipped **`frzftp = 0.0`** (active).
4. **2010** — Dun et al. publish the improved frost subroutines
   (Trans. ASABE 53(5):1399–1411), validated against Pullman WA and Morris
   MN plots (v2006.5 → v2010.1). A Minnesota validation site would surface
   the desiccation/stalled-frost pathology of an active migration term
   immediately; zeroing `frzftp` is the one-line fix. (Inference — the
   paper text has not been checked for whether the published model claims
   the term active; flagged as an open verification below.)
5. **Pattern match**: this is the same legacy lifecycle as the `ksflag`
   forest-frost disable (provisional change, no recorded why). The author
   cited prior agent-memory labels (`legacy-contract-vs-bug` / "routines
   disabled to work around bugs") for context, but the durable governance
   reference is ADR-0017's distrust posture.

## Legacy code analysis (Static, file:line against the pinned baseline)

### The dead gate

- `frzng.for:373-377`: if the layer below the front is frozen, `qwet = 0`
  (correct physical guard).
- `frzng.for:393-394`: `cd frzftp = -50` (commented) / **`frzftp = 0.0`**
  (active).
- `frzng.for:396-403`: `saxfun` returns the water potential `wtpm` of the
  layer below the front. `saxfun.for:34` documents `varwtp` as "soil water
  potential in meter"; the normal calculation assigns `varwtp = -wtpkpa / 10`
  (`saxfun.for:123-124`), and the error fallbacks return `−150.0`
  (`saxfun.for:72-79, 88-93`), establishing the negative-potential (suction as
  negative metres) convention. A moist layer sits near `−3 m`; dry near
  `−150 m`.
- `frzng.for:410`: activation condition `if ((frzftp .lt. wtpm) .and.
  (frdp > 0.001))` — i.e. `0.0 < wtpm`. With `wtpm` always negative, the
  condition is **always false**: `qwater = 0`, `qwet = 0`, permanently.
  With the documented `−100` (or tried `−50`) the condition activates for
  any moist layer — so `0.0` is not a parameter choice, it is an off
  switch.

### Why it could not be tuned (the explosion arithmetic)

The flux is plain Darcy over a *fine layer* (1–2 cm, `dg/nfine`):

```
qwater = kunsat · (wtpm − frzftp) / 2 / dz_fine        (frzng.for:411-414)
```

- With `frzftp = −100 m`, `wtpm = −3 m` (moist), `dz_fine = 1.5 cm`:
  raw head gradient ≈ `97 / 0.015 ≈ 6,500`; after the code's `/2.0` taming
  factor, the effective gradient is ≈ `3,200`. For moist silt loam
  (`kunsat ~ 10⁻⁷ m/s`), flux ≈ `3.2×10⁻⁴ m/s ≈ 1.16 m/hour` of liquid
  water toward the front — tens of thousands of times above mm/day-scale
  physical frost-heave migration rates.
- The code's own taming devices concede the problem:
  `/2.0` with the comment "we are using the maximum hydraulic gradient …
  migration rate slows down due to water depletion" (`frzng.for:405-414`),
  and an hourly **supply cap** that limits extraction to draining the
  adjacent fine layer to wilting point within the hour
  (`frzng.for:419-425`).
- Even at the cap: draining `(θ − θwp)·dz_fine ≈ 2.25 mm/hour` of water
  gives `qwet = L·flux ≈ 3.35×10⁸ × 2.25×10⁻³/3600 ≈ 209 W/m²` — roughly
  **50× a realistic midwinter `qhtout`** near 4 W/m² (about 35–105× across a
  2–6 W/m² under-snow range). With any
  negative `frzftp` the term pegs at its cap every hour: the front can
  never advance, and the profile below desiccates at ~50 mm/day with the
  water teleported to the front as ice (runaway heave, spring water-balance
  chaos). Halving the gradient (`−50`) changes nothing because the cap, not
  the gradient, is binding. **The only stable setting is off.**

### The missing physics

The formulation uses the **unfrozen** soil's `kunsat` with the full
Clausius–Clapeyron front potential. In real freezing soils the rate limiter
is the **frozen fringe**: the partially frozen zone between the warmest ice
lens and the freezing front, whose hydraulic conductivity collapses by many
orders of magnitude as pore ice saturates. Omitting fringe impedance while
keeping the full front suction guarantees non-physical fluxes — no constant
rescue (`/2`, supply caps, halved `frzftp`) can fix a missing state-dependent
resistance term.

### Related but distinct live code

- `qdry` (lower-front heat) and the `tmpbl` seasonal wave are live and are
  FDHP01 De's subject — do not conflate.
- `watdst` mode `1` ("around frozen front", `watdst.for:20-25`) is the
  redistribution path `frzng` would use for migration water
  (`frzng.for:624-630`) — currently effectively idle for heave because
  `qwater = 0`, but live for ordinary redistribution.
- `amtfrz` / heave bookkeeping (`cwint.inc:50-64`) exists as diagnostics.

## openWEPP experiments and diagnosis (the FDHP01 D3 arc, 2026-06-11/12)

How this surfaced: the D3 staged increments left a 2× depth overshoot and
+500-day duration over-persistence after the frost state machine, in-hour
freeze feedback (Db), seasonal `qdry` wave (Dc1), and forced legacy snow
(Dd) were all in place.

- **Dd (forced legacy snow, `bc47e426`)**: depth improved 1146 → 857 mm
  mean max but stayed 0/43 inside the legacy 240–503 mm envelope → snow
  insulation (F4) is partial, residual is frost-side.
- **Trajectory discriminator (Ran, `H1.winter.dat` hourly vs openWEPP
  forced-snow WAT)**: divergence is at *onset* — openWEPP carries 199 mm of
  frost by mid-December of winter 1 while legacy shows 0 and peaks at
  50 mm that winter. Early-season divergence precedes deep snow.
- **qwet hypothesis raised**: legacy's near-zero advance under early thin
  snow suggested a strong opposing term at the front; eqn 3.8.4's
  migration heat fit perfectly — *if it were live*.
- **qwet hypothesis killed (Static)**: `grep qwet` in openWEPP returns
  nothing (never ported — correctly), and the legacy term is dead by the
  `frzftp = 0.0` gate + negative-potential convention above. The actual
  onset/depth/duration residual was then attributed to **F5** (openWEPP's
  `qdry` using the 0.2 dry-fallback conductivity instead of legacy's
  content-dependent harmonic mean ≈ 1.0–1.5 W/m/K — `coupling.rs:1070-1082`
  vs `frostn.for:430-458`), handled by FDHP01 increment De.
- **Diagnosis recorded here**: legacy frost parity does **not** require
  migration heat — legacy itself runs without it. Any future heave
  implementation is *new physics relative to the operative baseline*, not
  parity work, and must be judged against external authority (ADR-0017:
  the comparator cannot flag a term legacy doesn't compute).

## Why this might matter (promotion case)

- **Thaw-season erodibility**: frost heave loosens surface soil and ice-lens
  melt produces saturated, low-strength layers — the dominant winter erosion
  mechanism in the Palouse-type regimes WEPP targets. The legacy model
  captures some of this via frozen-soil erodibility adjustments without
  explicit heave; an openWEPP heave state would make that mechanistic.
- **Soil-water redistribution fidelity**: real overwinter profiles dry below
  the front and wet at the front; openWEPP currently has no mechanism for
  this signature (affects spring soil-moisture initial conditions).
- **Counter-case**: legacy ships without it; no WEPP calibration depends on
  it; FDHP01-class closure was achievable without it. Cost is high
  (fringe-impedance state, new conservation surfaces). Hence backlog, not
  queue.

## Roadmap — contract-first frozen-fringe impedance formulation

Gated, in order; each stage is a separate work package per the FDHP01
staged-increment lessons (shadow state first; conservation hard stops;
diagnostic before implementation):

1. **Authority selection (science review, operator-steered).** Candidate
   external authorities to adjudicate — fringe-impedance factor approaches
   (e.g. the exponential ice-content impedance used in SHAW-lineage and
   Lundin-type models: `K_fringe = K_unfrozen · 10^(−E·θ_ice)`), segregation
   potential theory (Konrad & Morgenstern), or rigid-ice models
   (O'Neill–Miller class). Selection criteria: hourly-step compatibility,
   parameter availability from existing soil inputs, and bounded-flux
   guarantees (the formulation must be provably non-explosive — derive the
   flux bound symbolically before any code). **Verify against the Dun et
   al. 2010 paper text** whether the published v2010.1 model claimed the
   migration term active (open question from the archaeology).
2. **Contract amendment.** `SC-SNOWFREEZE-001`: new invariant family for
   migration flux (bounded by fringe impedance; mass-conserving: liquid
   debited from the source fine layer, ice credited at the front, heave
   recorded), explicit statement that eqn 3.8.4's middle term is
   **inactive in the pinned baseline** (provenance: this document) so the
   comparator is not used as authority for it; new GAP for implementation.
3. **Red tests before code** (FDHP01 pattern): (a) flux-bound fixture —
   migration ≤ literature mm/day envelope across the full
   moisture/temperature domain (the `1.2 m/hour` legacy arithmetic above is
   the anti-fixture); (b) conservation round-trip — migrated-then-frozen
   water debits the source layer and survives thaw release through the
   C1b/C2 overflow paths; (c) desiccation guard — sustained migration
   cannot draw the source below wilting point (typed, not silent-capped);
   (d) heave bookkeeping (`amtfrz` alias) published as diagnostic.
4. **Shadow-state increment** — compute migration flux and `qwet`
   diagnostically (driving nothing), validate magnitudes against the
   selected authority on the algebraic-radium cohort winters; bit-identical
   output gate (FDHP01 increment-A pattern).
5. **Coupled increment** — `qwet` enters the freeze-arm energy balance and
   migration water enters the front's ice mass via the `watdst` mode-1
   seam; conservation hard stop at the prevailing texture floor; cohort
   depth/duration must not regress the De-certified envelope.
6. **Validation surface** — no legacy comparator for this term (it is off
   in the baseline); validation is against the external authority's
   published heave rates/profiles and the level-4/5 physics-suite scheme
   (author-cited agent-memory label: `correctness-reanchoring-scheme`), plus
   non-regression of all FDHP01 gates.

## Dependencies / sequencing

- **Hard prerequisite**: FDHP01 D3 closure (increment De certification) —
  the freeze/thaw energy budget must be settled before adding a new energy
  term, or its error will alias into this one.
- **Prerequisite**: F4 snow density/depth-split disposition (the insulation
  context heave operates under).
- **Default position**: behind MOFE and the snow Stage-2 review in the
  ROADMAP deferred tier; promote only with a sizing/evidence gate (e.g. a
  thaw-season erodibility characterization showing the missing mechanism is
  load-bearing for a target use case).

## References

- CRM Ch. 3.8, eqn [3.8.1]–[3.8.4] (frost energy balance; 3.8.4 middle term
  is the migration heat).
- Dun, S., J.Q. Wu, D.K. McCool, J.R. Frankenberger, D.C. Flanagan (2010).
  Improving frost-simulation subroutines of the WEPP model. Trans. ASABE
  53(5):1399–1411. https://doi.org/10.13031/2013.34896 — indexed as
  **R-24** in `references/annotated_bibliography.md`; closed access,
  full-text acquisition pending (the eqn 3.8.4 verification question above
  is parked on that acquisition). (Validation sites Pullman WA, Morris MN;
  v2006.5 → v2010.1.)
- Saxton, K.E., W.J. Rawls (2006). Soil water characteristic estimates by
  texture and organic matter for hydrologic solutions. SSSAJ 70:1569–1578
  (`saxfun.for` lineage).
- Frozen-fringe / impedance literature for stage-1 adjudication (candidate
  set, to be verified during authority selection): Konrad & Morgenstern
  segregation potential (Can. Geotech. J., 1980–81); O'Neill & Miller
  rigid-ice model (1985); Lundin (1990) impedance factor; Flerchinger &
  Saxton SHAW model frozen-soil hydraulics (Trans. ASAE, 1989).
- Pinned baseline sources: `frzng.for:370-437` (the term), `saxfun.for`
  (sign convention), `watdst.for` (mode-1 seam), `cwint.inc` (heave
  symbols).
- FDHP01 evidence chain:
  `docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/artifacts/d3-staged-increment-plan.md`
  (Dd outcome + F5 block),
  `d3-increment-dd-legacy-snow-forced-20260612.md`,
  `review_claude_fdhp01_closure_status_and_cohort_validation.md`.
- Author-cited agent-memory labels, retained as non-authoritative context:
  `legacy-frost-energy-terms`, `legacy-contract-vs-bug`,
  `contract-first-forced-not-chosen`.
