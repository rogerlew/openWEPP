# Snow / Frost Fidelity Strategy

Status: **active strategy** (2026-06-25). Primary gap:
`GAP-SNOWFREEZE-002`.

Evidence mode: **Static summary of run evidence** from
SNOWFROST-FIDELITY-D/E/F/H/I0 and SNOWDENSITY-01..04, plus a **2026-06-25
post-1976 melt-physics literature sweep** (two independent research passes,
load-bearing citations cross-verified) and a **Ran** grep of the openWEPP melt
code. This file is planning guidance, not canonical science authority. Canonical
authority remains `SC-SNOWFREEZE-001`, the array-native runtime specification,
and ADR-0011 / ADR-0017 / ADR-0024 / ADR-0026 until amended. The 2026-06-25
amendment adds the melt-model decision (§2), guardrails (§4), physics target
(§5), revised work-package sequence (§7), and references (§11).
A 2026-06-26 amendment adds the melt-tuning site set and physics guidance (§10).

## 0. TL;DR

Frost-depth fidelity is blocked by snow insulation. The paired field sites show
modeled snow depth is often too large, and the SNOTEL corpus shows the snow
state error is shared with pinned legacy WEPP: openWEPP and legacy as-built
density residuals differ by only about `0.4-4.4 kg m^-3` across the five SNOTEL
sites. That makes this a shared WEPP-lineage snowpack problem, not an
array-native regression.

The problem is not solved by tuning the empirical snow settling density (`ssd`).
SNOWFROST-FIDELITY-H routed all five SNOTEL sites `STRUCTURAL`: replacing the
as-built `ssd = 250 kg m^-3` arm with an observed-density arm did not close the
depth/density residuals. PySnobal remains useful as process-reference and
diagnostic flag evidence, but it did not beat the WEPP variants on the H rubric
and one CSS Lab water-year failed inside the SNOBAL core on a known thin-snow
instability.

Recommended route: build a **staged, opt-in bulk snow physics sub-solver**:
`snow_model = legacy_wepp | physics_bulk`. Start offline in Rust snowbench,
then promote only if the candidate beats legacy on forcing-robust SNOTEL
signatures. The candidate should replace site-tuned empirical settling with
literature-governed state evolution: mass-conserving SWE/depth/density, bulk
cold content or snow temperature, temperature-dependent fresh-snow density,
overburden/metamorphism compaction, and rain/melt liquid-water handling. The
leading densification candidate is the Anderson-1976/SNOBAL lineage, but the
equations and constants must be ratified contract-first before production use.

**Melt model — decided 2026-06-25: modernize the energy balance, do not adopt
the degree-day candidate.** Production openWEPP already ports WEPP's
Corps-of-Engineers *energy-balance* melt (`amelt`/`cmelt` radiation terms in the
hydrology kernel) — there is no degree-day factor in production. The degree-day
melt factor exists only in the offline `snowbench`, where the solar term is
roughly 1–2 orders of magnitude below the literature shortwave-radiation factor,
so it is a *decapitated* enhanced-temperature-index model whose SNOWDENSITY-04
tuning collapsed onto the temperature knob (`0.18 -> 0.05 kg m^-2 degC^-1 h^-1`).
Post-1976 melt-physics authority (Ohmura 2001; Hock 2003/2005; Pellicciotti 2005
/ Carenzo 2009; Walter 2005, the latter in WEPP's own USDA-ARS/PNW lineage)
argues for keeping melt **energy-balance / radiation-explicit**, not a fitted
temperature index. The route is therefore to **modernize the production CoE
energy-balance melt** — feed it the gridded shortwave Anderson (1976) lacked, add
a temperature/age-driven albedo state — and to **keep melt separate from
density**: the density defect is closed by overburden compaction, not by cutting
melt. The `physics_bulk` snowbench remains the offline density-physics sandbox;
its degree-day melt construct is a dead end and `dense_slow_melt_v1` is **not**
promotable as-is (see §2, §7).

Implementation implication: melt modernization is not one monolithic runtime
opt-in. It is a package ladder: contract/sign reconciliation, radiation-source
binding, albedo state, opt-in production melt, rubric adjudication, then handoff
to density compaction. Each package must close its own evidence gate before the
next one can consume it.

## 1. What We Know

- R7H closed opt-in; frost-depth fidelity reopened under
  `GAP-SNOWFREEZE-002`.
- SNOWFROST-FIDELITY-D exposed modeled WAT `Snow-Depth` and reran the
  observation harness.
- SNOWFROST-FIDELITY-E proved the paired field-site snow-depth failures are
  like-for-like depth failures, not SWE/depth aliasing or daily-timing artifacts.
  The dominant paired-site direction is modeled-over-observed snow depth.
- SNOWFROST-FIDELITY-F proved pinned legacy WEPP fails the same paired snow-depth
  control. Legacy is closer on Sleepers South and Morris; openWEPP is closer on
  Sleepers W9. Current openWEPP SWE is already close to legacy SWE, so this is
  inherited snowpack behavior rather than an array-native publication bug.
- SNOWFROST-FIDELITY-H added paired SNOTEL SWE/depth/density observations across
  five mountain snow climates and ran openWEPP, pinned legacy WEPP, and PySnobal
  against the v74 rubric.
- H found all five SNOTEL sites route `STRUCTURAL`: the observed-density `ssd`
  arm did not improve depth MAE by the required amount and often made it worse.
- H also showed openWEPP and legacy are effectively the same as-built density
  model for this purpose: their mean signed density residuals differ by only
  about `0.4-4.4 kg m^-3`, while their residuals versus observation are tens to
  hundreds of `kg m^-3`.
- PySnobal can generate sane diagnostic snow surfaces in selected lanes, but H's
  SNOTEL profile makes it a weak comparator, not an adoption target: four sites
  ran, CSS Lab WY2017 is unavailable from an upstream thin-snow instability, and
  the stable PySnobal cells are mostly worse than WEPP variants under the current
  bridge.
- The WEPP archaeology remains important: `snowd.for` uses the CRM Ch. 3.7
  empirical settlement lineage, `ssd` limits further settling when density
  exceeds the threshold, and the backlog records an unresolved Eq. 3.7.5
  code-vs-documentation divergence.
- **Melt code finding (2026-06-25, Ran — grep of the openWEPP tree):** the
  production hydrology kernel computes melt from WEPP's Corps-of-Engineers
  *energy balance* (`melt_amelt_in`/`melt_cmelt_in`, the `0.0254 * (amelt - bmelt
  + cmelt + dmelt)` lineage) — **no degree-day factor in production.** The only
  degree-day melt factor is in offline `snowbench_physics_bulk.rs`
  (`positive_degree_melt_kg_m2_per_c_hour`, plus a tiny `solar_melt_efficiency`
  of `0.005–0.02`). Converting that solar term to Pellicciotti units yields
  ~`5e-5..2e-4 mm h^-1 (W m^-2)^-1`, roughly 1–2 orders of magnitude below the
  near-physical shortwave-radiation factor `SRF ≈ 0.0094` (Static: derived from
  the snowbench formula, assumes `net_solar` is absorbed shortwave). So the
  snowbench is a temperature-index model with its radiation term effectively off,
  forcing all melt energy through the temperature knob.
- **Melt-physics literature finding (2026-06-25, Static — two research sweeps,
  cross-verified):** the temperature/degree-day melt factor is a *lumped
  surrogate* for longwave + sensible heat (both temperature-correlated; Ohmura
  2001) and is blind to net shortwave `(1 - albedo) * I`; it consequently varies
  nearly an order of magnitude site-to-site with no regional pattern (Hock
  2003/2005), which is why a single fitted constant cannot be a transferable
  default. The enhanced-temperature-index form `M = TF*T + SRF*(1-albedo)*I`
  (Pellicciotti 2005) makes shortwave explicit; across sites the radiation factor
  `SRF` is near-physical and stable (CV ~6%) while the temperature factor `TF` is
  volatile (CV ~56%) and nearly dispensable (Carenzo 2009). Walter et al. (2005),
  in WEPP's own USDA-ARS/PNW lineage, show an *uncalibrated* energy balance
  driven by `Tmax`/`Tmin` (+ now gridded shortwave) beats a best-fit temperature
  index. Model complexity does not predict skill (Krinner 2018; Ménard 2021).

## 2. Decision

Do **not** tune `ssd` or make site-specific density parameters the fix. The
observed-density arm was a diagnostic proof, not a production path, and it did
not close the residuals.

Move toward an opt-in physics snow model that lets density emerge from forcing
and state:

```text
snow_model =
    legacy_wepp      # current empirical WEPP lineage
    physics_bulk     # opt-in, literature-governed bulk snowpack solver
```

`legacy_wepp` preserves existing behavior and compatibility. `physics_bulk`
removes the `ssd` tuning surface from the new model path, but does not delete or
reinterpret legacy `snow.txt` fields until a compatibility/ADR gate explicitly
authorizes that cleanup.

**Melt decision (2026-06-25):**

1. **Modernize the energy-balance melt, do not adopt the degree-day candidate.**
   Production already uses the CoE energy balance; the path is to *strengthen* it
   with the gridded daily shortwave Anderson (1976) lacked and a temperature/age
   albedo state — not to replace it with the snowbench's decapitated
   temperature-index model. If a lighter scheme is ever wanted, the documented
   fallback is a *properly weighted* enhanced temperature-index (Pellicciotti
   2005: grounded `SRF`-primary, small/calibratable `TF`), never a lone fitted
   degree-day constant.
2. **Separate melt from density.** The SNOWDENSITY program's target is *density*
   (overburden compaction). The SNOWDENSITY-04 `dense_slow_melt_v1` "win" came
   chiefly from cutting the melt factor `3.6x` (`0.18 -> 0.05`), i.e. tuning the
   melt knob to mask a density gap. That conflation is rejected: density is fixed
   by Anderson-1976 compaction, melt is fixed (if at all) in the energy balance,
   and neither stands in for the other. `dense_slow_melt_v1` is **not promotable
   as-is.**
3. **Calibrate coefficients, never the shared radiation forcing.** One net/solar
   radiation field forces both melt and ET; rescaling it to fix melt necessarily
   moves ET (Gupta 2023; Beven equifinality). Any calibration handle lives on the
   melt coefficients, not the radiation input.

## 3. Goals

- **G1:** Improve forcing-robust SNOTEL signatures under `INV-SNOWFREEZE-050`:
  cold-season density, densification trajectory, depth-SWE slope, onset/peak/
  melt-out timing, regime ordering, and bias-sign consistency.
- **G2:** Improve snow depth where forcing allows. Absolute SWE/depth magnitude
  cells are forcing-limited and should not drive tuning by themselves.
- **G3:** Make snow-insulation control (`TOL-SNOWFREEZE-009`) passable or
  bounded well enough to resume frost-depth attribution.
- **G4:** Keep the model opt-in, conservation-clean, and reversible until it
  beats legacy on forcing-robust signatures and passes governance review.

## 4. Guardrails

- **Contract-first:** amend `SC-SNOWFREEZE-001` before adding production physics,
  equations, constants, tolerances, or activation behavior.
- **Physical defaults, optional calibration (refined 2026-06-25):** the bar is
  that defaults work *reasonably without requiring* site-specific calibration,
  while still exposing calibration handles consistent with WEPP's other
  sub-systems. SNOTEL *validates*; it does not supply the default constants.
  Defaults come from cited authority — never fit to the five-site validation set
  (the SNOWDENSITY-04 failure mode).
- **Radiation/ET coupling:** one net/solar radiation field forces both snowmelt
  and ET. Do **not** rescale or tune the radiation forcing to make melt match —
  that silently moves ET (Gupta 2023) and is the compensating-error trap (Beven
  equifinality). Calibration handles live on the melt coefficients; the radiation
  field stays consistent across melt and ET. If gridded radiation has a
  characterized bias, correct it once at the source so both processes see the
  same corrected field.
- **Melt/density separation:** do not use a melt change to fix a density gap, or
  vice versa. Density is closed by overburden compaction; melt by the energy
  balance. A candidate that improves a density signature only by reducing melt is
  rejected.
- **External physics authority:** equations must come from literature,
  source-level reference implementations, or a ratified science-contract
  decision. PySnobal/SNOBAL can inform the implementation, but cannot become
  correctness authority under ADR-0017.
- **Conservation:** densification must conserve SWE unless explicit liquid
  retention/release/refreeze terms change the water store. Depth changes should
  follow state and density, not a second ledger.
- **Opt-in first:** no default activation, no compatibility deletion, and no
  snow-influenced parity re-baseline without an ADR.
- **Profile, not scalar:** use the v74 rubric. Forcing-robust cells carry model
  verdict weight; forcing-limited magnitude cells are reported and discounted.
- **No frost shortcut:** heat-flow, frozen-K, SFCC, impedance, or `Qwet` work
  remains blocked until snow-depth/density control is passable or bounded.

## 5. Physics Target

The first candidate should be the smallest model that can plausibly close the
forcing-robust snow signatures:

- **State:** SWE, physical depth, bulk density, liquid water retained in pack,
  snow cold content or bulk snow temperature, and snow-cover age.
- **Fresh-snow density:** replace fixed `100 kg m^-3` in the `physics_bulk` path
  with a temperature- and/or wind-aware literature relationship. Candidate
  families: Anderson/SNOW-17 or Hedstrom-Pomeroy. The choice is still open.
- **Densification:** evaluate Anderson-1976/SNOBAL-style temperature
  metamorphism, overburden compaction, and wet-snow compaction as the leading
  candidate. Constants must be carried from cited authority, not fitted to the
  five SNOTEL sites.
- **Liquid water:** represent rain-on-snow retention/release and meltwater
  storage explicitly enough to preserve the `INV-SNOWFREEZE-015/019` storage
  lineage.
- **Melt (modernize, do not replace):** keep production's CoE energy-balance melt
  and feed it the gridded daily shortwave Anderson (1976) lacked. The melt factor
  is *not* a physical constant — it is a lumped surrogate for longwave + sensible
  heat that is blind to net shortwave (Ohmura 2001), so a fitted degree-day
  constant has no transferable authority. The documented lighter alternative, if
  ever needed, is a properly weighted enhanced temperature-index
  `M = TF*T + SRF*(1-albedo)*I` with grounded `SRF ≈ 0.0094` primary and a small
  calibratable `TF` (Pellicciotti 2005 / Carenzo 2009) — never a lone degree-day
  knob.
- **Albedo:** the control on the shortwave melt term. Use a temperature/age-driven
  decay (high on fresh snowfall ~0.8–0.9, decaying toward ~0.4–0.5 aged/melting),
  e.g. the accumulated-positive-temperature form of Brock et al. (2000), which
  needs no forcing beyond temperature. Apply WEPP canopy attenuation
  `(1 - cancov)` to the shortwave term to preserve forest behavior; note the net
  canopy effect on melt flips sign with climate (Lundquist 2013), so a single
  lumped canopy melt reduction is wrong.
- **Thermal coupling:** start with bulk cold content / snow temperature. Escalate
  to a two-layer or fuller energy-balance model only if the bulk candidate fails
  forcing-robust rubric cells for reasons the missing thermal structure explains.
- **Bounds:** maintain non-negative SWE/depth, density domain constraints, and a
  contract-ratified upper density cap.

This is a native Rust implementation target. PySnobal is a reference
implementation and cross-check in stable established-pack regimes only; it is
not a runtime dependency and should not be hardened unless a later diagnostic
package needs broader PySnobal coverage.

## 6. Architecture Target

- Implement the new snow model inside the ADR-0026 winter-column sub-solver,
  where snow state is already typed and coupled to frost.
- Split model selection by process. Melt modernization is not the same switch as
  density compaction:

```text
snow_melt_model =
    legacy_coe                 # current default production CoE melt
    coe_shortwave_albedo_v1    # opt-in modernized CoE melt

snow_model =
    legacy_wepp                # current empirical WEPP depth/density lineage
    physics_bulk               # later opt-in density/pack sub-solver
```

- Keep density/pack selection through an explicit runtime option:
  `snow_model = legacy_wepp | physics_bulk`.
- Keep the modernized melt implementation in the production winter-column CoE
  melt seam (`amelt`/`bmelt`/`cmelt`/`dmelt` in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/`). Do not route it
  through `snowbench_physics_bulk.rs`.
- Add a narrow melt boundary:

```text
hourly forcing + prior snow/albedo state + canopy state
    -> CoE melt terms and signed raw melt
    -> corrected daily redistribution / routed melt
    -> downstream liquid forcing and publication operands
```

- Add a narrow density/pack boundary:

```text
forcing + prior snow state + model constants
    -> snow state after timestep
    -> liquid release / retained water / insulation operands
```

- Drive density/pack candidates through offline Rust `snowbench` using the
  existing WEPP/openWEPP/PySnobal forcing bridge and SNOTEL observation corpus.
  Use SNOWDENSITY-04 degree-day variants only as negative benchmarks for
  melt/density conflation.
- Promote production melt only through the staged SNOWDENSITY-05A..05F ladder
  below, with contract-derived tests, conservation tests, default rollback, and
  SNOTEL rubric comparison at the package that owns each gate.
- Preserve direct/compatibility rollback. The new model can deliberately diverge
  from legacy only under the opt-in mode and only after the governance ADR lands.

## 7. Work-Package Sequence

| Stage | Goal | Gate |
|---|---|---|
| **SNOWDENSITY-01 Evidence Reconciliation** | Complete in `docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001/`: consolidated E/F/H/I0 evidence; pinned openWEPP-vs-legacy density deltas; classified rubric cells; read Shen 2011 and the WEPP Eq. 3.7.5 divergence. | Evidence package only; no production code. |
| **SNOWDENSITY-02 Contract + ADR** | Complete in `docs/work-packages/20260625-snowdensity-02-contract-adr-001/`: amended `SC-SNOWFREEZE-001` v75 with `INV-SNOWFREEZE-051`, candidate `physics_bulk` state/process envelope, no-site-tuning rule, conservation obligations, and opt-in activation constraints; added ADR-0027. | Contract guard tests only; no production code. |
| **SNOWDENSITY-03 Offline Physics Core** | Complete in `docs/work-packages/20260625-snowdensity-03-offline-physics-core-001/`: implemented `physics_bulk` in Rust snowbench with candidate constants, bulk thermal state, fresh-snow density, densification, liquid retention/release, focused tests, production-confinement guard, and five-site SNOTEL rubric output. | Candidate remains offline only; first profile is finite but not production-promotable (`forcing_robust fail=24`, `marginal=13`, `pass=3`, `strong=5`, `unavailable=15`). |
| **SNOWDENSITY-04 Offline Adjudication Loop** | Complete in `docs/work-packages/20260625-snowdensity-04-offline-adjudication-loop-001/`: added global named candidate variants and adjudicated four variants against H openWEPP/legacy/PySnobal profiles. `dense_slow_melt_v1` scored best. | **Superseded disposition (2026-06-25): `dense_slow_melt_v1` is NOT promotable.** Its improvement came chiefly from cutting the degree-day melt factor `3.6x` (`0.18 -> 0.05`) — tuning the melt knob to mask a density gap (melt/density conflation, §2). Keep as a *negative* benchmark only. |
| **SNOWDENSITY-05A Melt Contract + Sign Reconciliation** | Complete in `docs/work-packages/20260625-snowdensity-05-melt-modernization-contract-first-001/`: amended `SC-SNOWFREEZE-001` v76 with `INV-SNOWFREEZE-052`, `OBL-SNOWFREEZE-P-027`, opt-in `snow_melt_model`, shortwave/albedo operand placeholders, no-radiation-tuning guard, explicit negative-benchmark disposition for degree-day snowbench variants, and signed `melt_bmelt_in` convention reconciling WEPP Chapter 3 `amelt - bmelt + cmelt + dmelt` prose with current trace identity. | Contract-derived tests pass. No production runtime code, constants, parser surfaces, output schemas, or defaults changed. Superseded by 05B source/provenance binding as the current melt-modernization gate. |
| **SNOWDENSITY-05B Shortwave Source Binding** | Complete in `docs/work-packages/20260626-snowdensity-05b-shortwave-source-binding-001/`: amended `SC-SNOWFREEZE-001` v77 with `INV-SNOWFREEZE-053`, `OBL-SNOWFREEZE-P-028`, `winter_shortwave_daily_radly`, and the Shortwave Source Binding Addendum. The canonical openWEPP source is the existing daily climate `rad`/`radly` field in `Ly d^-1`; upstream gridded provider selection/spatialization remains outside engine ownership. Runtime transformation is the existing `SC-CLIMATE-001#INV-CLIMATE-013` path into `winter.hourly.rad_mj_m2_####`, and ET/snow share the same daily radiation authority. | Contract-derived tests pass. No production runtime code, constants, parser surfaces, output schemas, source selectors, albedo constants, or defaults changed. Superseded by 05C albedo state/core as the current completed gate. |
| **SNOWDENSITY-05C Albedo State Core** | Complete in `docs/work-packages/20260626-snowdensity-05c-albedo-state-core-001/`: amended `SC-SNOWFREEZE-001` v78 with `INV-SNOWFREEZE-054`, `OBL-SNOWFREEZE-P-029`, `brock2000_temperature_age_v1`, `Ta` state, fresh-snow reset threshold, `[0, 0.85]` bounds, missing-state fail-closed behavior, and a standalone typed Rust albedo update core. | Contract/unit tests pass for albedo bounds, monotonic age decay, fresh-snow reset, missing-state fail-closed behavior, and no effect on `legacy_coe` default runs. No routed-melt acceptance, production wiring, parser surface, output schema, or default changed. Superseded by 05D opt-in CoE melt implementation. |
| **SNOWDENSITY-05D Opt-In CoE Melt Implementation** | Complete in `docs/work-packages/20260626-snowdensity-05d-opt-in-coe-melt-implementation-001/`: amended `SC-SNOWFREEZE-001` v79 with `INV-SNOWFREEZE-055` and `OBL-SNOWFREEZE-P-030`; wired `coe_shortwave_albedo_v1` into the typed production CoE melt path behind an explicit selector; added albedo carry through direct snow runtime state; exposed raw melt, redistributed melt, routed `wmelt`, and SWE-loss lineage totals. | Focused tests prove `legacy_coe` identity, missing active opt-in albedo state fail-closed behavior, formula-level `amelt` reconstruction, routed-melt/SWE-loss/WB12-WB13 liquid-forcing closure, and direct-runtime albedo carry. No default activation, parser surface, output schema, radiation source, or coefficient fitting changed. Next gate is 05E melt rubric adjudication. |
| **SNOWDENSITY-05E Melt Rubric Adjudication** | Complete in `docs/work-packages/20260626-snowdensity-05e-melt-rubric-adjudication-001/`: added diagnostic-only `openwepp-snowbench coe-melt` replay for `legacy_coe` and `coe_shortwave_albedo_v1`; generated five-site SNOTEL rubric profiles; reran the non-SNOTEL rubric baseline; and preserved diagnostic confinement/no-default-activation boundaries. | Bounded `PROMOTION-CANDIDATE` relative to diagnostic legacy (`robust_fail_count 13 -> 10`, `robust_ordinal_score 61 -> 84`), not default activation. H as-built context remains `robust_fail_count=9`, `robust_ordinal_score=84`; non-SNOTEL frost attribution remains blocked by snow-control failures with `openwepp_defective_cells=0`. |
| **SNOWDENSITY-05F Melt Closure / Density Handoff** | Complete in `docs/work-packages/20260626-snowdensity-05f-melt-closure-density-handoff-001/`: amended `SC-SNOWFREEZE-001` v82 with `INV-SNOWFREEZE-056`, `OBL-SNOWFREEZE-P-031`, boundary disposition, and the 05F addendum; froze `coe_shortwave_albedo_v1` as an opt-in-only density-facing melt interface; preserved `legacy_coe` default/rollback; ratified same-day future snowfall albedo continuity; dispositioned the Claude review by labeling 05E replay evidence regime-limited (`cancov = 0.0`, PySnobal-bridge radiation); recorded the operator clarification that validation management should be coniferous forest with winter `cancov` about `0.9`; and bound activation evidence to both 05E diagnostic replay and H as-built context. | SNOWDENSITY-06 may consume the opt-in melt boundary without retuning melt, albedo, coefficients, or shared radiation, but its entry gate must first fix/prove harness fidelity for configured coniferous-forest canopy and native/proven shortwave. No default activation, parser/runfile/CLI selector, output schema, coefficient, radiation-source, or density-physics change was made. |
| **SNOWDENSITY-05G Harness Fidelity Rerun** | Complete in `docs/work-packages/20260626-snowdensity-05g-harness-fidelity-rerun-001/`: amended `SC-SNOWFREEZE-001` v83 with `INV-SNOWFREEZE-057`, `OBL-SNOWFREEZE-P-032`, and the 05G addendum; corrected diagnostic `coe-melt` replay to consume configured runtime-surface canopy (`cancov = 0.9` for all five SNOTEL coniferous fixtures) instead of the prior `0.0` harness constant; recorded the PySnobal bridge shortwave inversion identity as like-for-like; and reran five-site SNOTEL adjudication. | Representative-regime result is `NON-PROMOTION` for default activation: robust failures do not improve (`9 -> 9`), though ordinal score rises slightly (`84 -> 86`). The 05E promotion-candidate result is superseded as regime-limited. External review endorsed the deflating result and clarified that conifer neutrality is expected because `(1 - cancov)` leaves little radiation-melt leverage at `cancov ~= 0.9`. `legacy_coe` remains default/rollback; no production selector, output schema, coefficient, radiation, density, or frost change was made. |
| **SNOWDENSITY-05H Low-Canopy Mixed-Forest Melt Adjudication** | Future melt-value fork: before retiring or reopening activation for `coe_shortwave_albedo_v1`, rerun melt adjudication across the installed `tests/fixtures/cancov_forest/` canopy gradient and the normalized `tests/fixtures/cancov_forest/observations/` Harvard HF237 / Marcell RDS-2021-0016 stratified tables. The harness must use real per-day seasonal `cancov`, not the single representative evergreen value used by 05G. | Decisive only for melt value in low-canopy or leaf-off regimes. Marcell conifer/deciduous/open and Harvard open/hardwood are model-bound; Harvard hemlock remains observation-installed but unbound to a pure model hillslope and must be excluded or explicitly proxy-scoped. No density compensation, no default activation, and no site-specific tuning. This package is not required to start SNOWDENSITY-06 density work if 06 keeps fixed melt boundaries. |
| **SNOWDENSITY-06 Density Compaction** | Complete in `docs/work-packages/20260626-snowdensity-06-density-compaction-001/`: amended `SC-SNOWFREEZE-001` v84 with `INV-SNOWFREEZE-058`, `OBL-SNOWFREEZE-P-033`, and the 06 addendum; added offline `density_compaction_v1`; exposed named SNOBAL-lineage PTM/POC/liquid-water compaction constants; preserved baseline candidate melt constants, albedo, canopy, radiation, production defaults, and rollback; and extended adjudication with density/densification robust-cell summaries. | Density-cell gate improved without site tuning or melt changes (`fail 9 -> 7`, score `16 -> 22` vs legacy/as-built), but whole-rubric promotion remains blocked (`robust fail 9 -> 18`, score `84 -> 46`). No default activation, parser/runfile/CLI activation selector, output schema, frost attribution, or mixed/deciduous melt change. |
| **SNOWDENSITY-06B CoE-Bound Density Replay** | Complete in `docs/work-packages/20260626-snowdensity-06b-coe-bound-density-replay-001/`: amended `SC-SNOWFREEZE-001` v85 with `INV-SNOWFREEZE-059`, `OBL-SNOWFREEZE-P-034`, and the 06B addendum; added diagnostic-only `openwepp-snowbench coe-bound-density`; replayed `density_compaction_v1` against fixed `legacy_coe` and `coe_shortwave_albedo_v1` CoE melt/liquid/SWE-loss boundaries; and ran five-site SNOTEL adjudication. | Offline gate cleared without site tuning or melt/radiation/canopy/albedo retuning. Best result is `coe_bound_density_compaction_v1_coe_shortwave_albedo_v1`: robust failures `9 -> 5`, robust score `84 -> 110`, density failures `9 -> 5`, density score `16 -> 41`, daily CoE SWE identity residual about `4.44e-16 m`. No runtime/default activation, parser/runfile/CLI selector, output schema, mixed/deciduous canopy adjudication, or frost attribution changed. |
| **SNOWDENSITY-07 Runtime Opt-In** | Complete in `docs/work-packages/20260626-snowdensity-07-runtime-opt-in-001/`: amended `SC-SNOWFREEZE-001` v86 with `INV-SNOWFREEZE-060`, `OBL-SNOWFREEZE-P-035`, typed `snow_density_model`, and CoE boundary carry variables; added `physics_bulk_density_compaction_v1` as a typed runtime opt-in; preserved `legacy_wepp` as default/rollback; kept CoE SWE/liquid/routed-melt boundaries authoritative; and projected opt-in runtime depth/density plus separate CoE boundary carry through direct R4G state, downstream operands, shadow projection, runtime carry, and publication-facing winter-column state. | Runtime opt-in is available only to typed callers; no default activation, parser/runfile/CLI selector, output schema, mixed/deciduous canopy adjudication, coefficient retuning, radiation/albedo/melt retuning, or frost attribution changed. Full workspace gates pass. |
| **SNOWDENSITY-08 Snow/Frost Gate Rerun** | Complete in `docs/work-packages/20260626-snowdensity-08-snow-frost-gate-rerun-001/`: amended `SC-SNOWFREEZE-001` v87 with `INV-SNOWFREEZE-061`, `OBL-SNOWFREEZE-P-036`, and the 08 addendum; reran the accepted CoE-bound density lineage against the SNOTEL rubric; reran current direct-production non-SNOTEL WAT snow-control/frost evidence; and published a compact decision artifact. | SNOTEL density evidence still clears (`coe_bound_density_compaction_v1_coe_shortwave_albedo_v1`, robust failures `9 -> 5`, robust score `84 -> 110`, density failures `9 -> 5`, density score `16 -> 41`, CoE SWE identity residual about `4.44e-16 m`), but frost attribution remains blocked. The non-SNOTEL path is still default `legacy_wepp` density with no authorized coupled opt-in WAT/publication path; current default rerun has three `SNOW_CONTROL_FAILED` sites and two sites with no paired observed snow rows. Next route is an authorized diagnostic coupled opt-in WAT/publication path before frost attribution resumes. |
| **SNOWDENSITY-09 Diagnostic Coupled WAT Rerun** | Complete in `docs/work-packages/20260626-snowdensity-09-diagnostic-coupled-wat-rerun-001/`: amended `SC-SNOWFREEZE-001` v89 with `INV-SNOWFREEZE-062`, `OBL-SNOWFREEZE-P-037`, the 09 addendum, and the paired-snow gate correction; added a package-bound diagnostic direct-production selector `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL`; extended direct snow trace rows with selected `snow_density_model`; and reran non-SNOTEL default-vs-opt-in WAT evidence. | The coupled opt-in WAT path is now proven (`75,610` opt-in trace rows selected `physics_bulk_density_compaction_v1`) and reduces snow-depth residual magnitudes at all three paired-snow sites, but snow control still fails at those three gate-eligible sites. SCAN Mandan ND and Reynolds Creek ID lack observed snow-depth rows and are reported out-of-gate rather than counted as snow-control gate pass/fail/blocker inputs. Frost attribution remains blocked as `NON-SNOTEL-OPT-IN-SNOW-CONTROL-FAILED`. No default activation, parser/runfile/user CLI selector, output schema, WAT rewriting, site constants, tuning, or production physics changed. |
| **FROST-RESUME** | Return to heat-flow, frozen-K/SFCC, or migration/fringe adjudication only after snow insulation is controlled. | Mechanism-specific work package. |

SNOWDENSITY-05 package sizing rule: do not collapse 05A..05F into one package.
The contract/sign gate, radiation source binding, albedo state, production
opt-in, rubric adjudication, and handoff each have distinct evidence and hold
boundaries. Combining them recreates the SNOWDENSITY-04 failure mode: a green
profile can hide a wrong mechanism.

## 8. Existing Assets

- SNOTEL fixtures, normalized observations, and observed density:
  `tests/fixtures/snotel_observed/`.
- Frost-depth fixtures: `tests/fixtures/snowfreeze_observed/`.
- Canopy-gradient melt-tuning fixtures (8 sites, conifer→mixed→deciduous→
  pasture): `tests/fixtures/cancov_forest/` (see §10).
- Canopy-stratified Harvard HF237 and Marcell RDS-2021-0016 normalized
  observations: `tests/fixtures/cancov_forest/observations/`.
- Rubric authority: `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-050` +
  `TOL-SNOWFREEZE-011`.
- Correspondence authority: `INV-SNOWFREEZE-048` for depth and
  `INV-SNOWFREEZE-049` for SNOTEL density.
- Three-way harnesses under `tools/snowfreeze_observed/`: `observed_harness`,
  `legacy_snow_compare`, `pysnobal_compare`, `snotel_density_three_way`, and
  `non_snotel_rubric_baseline`.
- H package evidence:
  `docs/work-packages/20260625-snowfrost-fidelity-h-snotel-density-three-way-001/`.
- I0 baseline:
  `docs/work-packages/20260625-snowfrost-fidelity-i0-non-snotel-rubric-baseline-001/`.
- Snow archaeology:
  `docs/backlog/20260605-snow-code-deferred-science-review.md` and
  `references/copyrighted/D_Shen_020312.pdf`.
- Candidate reference implementations: local PySnobal/SNOBAL source
  (`_time_compact.c`, `_h2o_compact.c`, `_precip.c`) and the literature cited by
  the future contract amendment.

## 9. Open Decisions

- **Resolved 2026-06-25:** the melt-model fork. Decision = modernize the
  production CoE energy-balance melt with gridded shortwave + albedo; do not adopt
  the degree-day snowbench candidate; keep melt separate from density (§2).
- **Resolved 2026-06-26:** shortwave source/provenance. SNOWDENSITY-05B bound
  the existing daily climate `rad`/`radly` seam and left gridded provider
  selection/spatialization outside engine ownership.
- **Resolved 2026-06-26:** albedo decay scheme for first opt-in melt route.
  SNOWDENSITY-05C ratified `brock2000_temperature_age_v1` as the standalone
  temperature/age albedo state core. CLASS/ISBA/Crocus remain fuller
  alternatives for later adjudication only if 05E shows the Brock core is not
  adequate under forcing-robust signatures.
- Whether the current WEPP canopy attenuation `(1 - cancov)` should remain the
  only first-pass canopy modifier for opt-in melt, or whether forest-specific
  canopy radiation/longwave terms require a later package. SNOWDENSITY-05A must
  preserve the current attenuation as the initial contract default.
- Which coefficient handles, if any, may be exposed for melt after physical
  defaults are ratified. SNOWDENSITY-05E may evaluate handles, but may not fit
  defaults to the five SNOTEL sites or tune shared radiation forcing.
- Which fresh-snow-density relationship should be ratified for `physics_bulk`.
- Whether Anderson-1976/SNOBAL compaction is sufficient in a bulk model or
  requires a two-layer thermal structure.
- Which density cap is physically and contractually correct for the opt-in model.
- How to represent wind, canopy, and precipitation undercatch without
  introducing site calibration.
- How to score PySnobal-unavailable cells in future rubric revisions; H already
  dispositioned the CSS Lab WY2017 failure as upstream thin-snow instability.
- Whether the FDHP01 F4 density discrepancy is still relevant after SNOWSCI-S1
  storage single-sourcing and H's openWEPP≈legacy density evidence.

## 10. Melt-Tuning Site Set and Physics Guidance

Added 2026-06-26. The SNOWDENSITY arc closed the SNOTEL **density** gap (06B:
robust fail `9 -> 5`, beats both as-builts, CoE SWE identity to machine epsilon)
but left two **melt / accumulation** questions open: the CoE shortwave/albedo
modernization is **neutral at high evergreen `cancov ≈ 0.9`** (05G) and
**untested at less-attenuated winter canopies**; and the non-SNOTEL maritime frost
sites **over-accumulate snow** (08/09
`NON-SNOTEL-OPT-IN-SNOW-CONTROL-FAILED`). Density compaction did not clear those
positive residuals, so winter melt, rain-on-snow heat, and snow/rain partition are
leading hypotheses, not yet adjudicated defects. This site set spans the canopy
gradient needed to test those regimes without using density as compensation.

### 10.1 Site set (in-repo: `tests/fixtures/cancov_forest/`)

All eight are operator wepp.cloud builds with DAYMET + GRIDMET + CLIGEN + PRISM
forcing. The current fixtures all run frost with `ksflag=1`: seven forest
hillslopes were changed from `ksflag=0` to `1` for paired snow/frost diagnostics,
while `sleepers_pasture_vt` already had `ksflag=1`.

**Coniferous — high `cancov` control (extends `snotel_observed`):**

| Fixture | Site (topaz→wepp, years) | Lat, Lon | Elev | Vegetation / climate | Snow data |
|---|---|---|---|---|---|
| `hjandrews_conifer_or` | HJ Andrews EF, OR (22→p2, 1980–2024) | 44.23, −122.17 | 410–1630 m | Douglas-fir / W. hemlock; **Pacific maritime, transient rain-snow** | EDI `MS007` under-canopy + `719:OR:SNTL` |
| `tenderfoot_conifer_mt` | Tenderfoot Creek EF, MT (22→p2, 1980–2024) | 46.917, −110.850 | 1996–2261 m | Lodgepole / spruce-fir; N. Rockies continental | on-forest `1008`/`1009:MT:SNTL` (SWE+depth) |
| `berthoud_conifer_co` † | Berthoud Summit / Fraser EF, CO (32→p4, 1986–2024) | 39.80, −105.78 | 3444 m | Engelmann spruce / subalpine fir / lodgepole; continental subalpine | `05K14S:CO:SNTL` (SWE+depth+soil-T) |
| `morescreek_conifer_id` † | Mores Creek Summit, ID (42→p6, 1986–2024) | 43.933, −115.667 | 1857 m | Ponderosa / lodgepole; intermountain | `15F01S:ID:SNTL` (soil-T since 1992) |

**Mixed / deciduous / pasture — lower `cancov`, the untested regime + the new managements:**

| Fixture | Site (topaz→wepp, years) | Lat, Lon | Elev | Vegetation / winter canopy | Snow data |
|---|---|---|---|---|---|
| `marcell_mixed_mn` ★ | Marcell EF, MN (61→p10, 1980–2024) | 47.53, −93.47 | ~422 m | aspen/birch/maple + pine/spruce + tamarack + peat; **stratified conifer/deciduous/open** | USFS RDA `10.2737/RDS-2021-0016` (SWE+depth+frost by cover type, 1962–) |
| `harvard_mixed_ma` ★ | Harvard Forest, MA (43→p8, 1980–2024) | 42.537, −72.173 | 348 m | red oak/red maple (leaf-off) + pine/hemlock | `HF155` SWE pillow + `HF237` depth/density by **hemlock/hardwood/open** + NEON soil-T |
| `hubbardbrook_deciduous_nh` | Hubbard Brook EF, NH (62→p10, 1980–2024) | 43.945, −71.720 | 222–1015 m | northern hardwood (sugar maple/beech/birch) leaf-off; spruce-fir up high | EDI `knb-lter-hbr.27` (depth/SWE/frost, 1956–) + `2069:NH:SCAN` |
| `sleepers_pasture_vt` | Sleepers River, VT (23→p3, 1980–2024) | 44.459, −72.092 | 200–670 m | mixed hardwood-conifer watershed; **fixture hillslope = pasture/clearing** | USGS `10.5066/P9NMQX70` (60-yr depth+SWE); pairs with W9 hardwood frost fixture |

† RAP_TS-adjusted `cancov`. ★ **canopy-stratified observations** (paired open vs
under-canopy snow at one site/climate). The current fixtures are one representative
modeled hillslope per site; they become decisive canopy-attenuation evidence only
after an explicit observation-stratum mapping or paired open/under-canopy model
variants exist. Marcell + Harvard exercise the new wepppy **mixed-forest**
management; Hubbard Brook the **deciduous** management.

**Why this spread.** The confirmed raw fixture management values currently span
conifer `cancov ≈ 0.90`, mixed `≈ 0.55`, and deciduous `≈ 0.20`; the Sleepers
pasture runtime trajectory still needs to be archived before treating it as the
lowest-cancov endpoint. Prior wepppy management validation suggested lower effective
winter values for mixed/deciduous canopies (`≈ 0.44` / `≈ 0.07`), but those values
are planning targets until the per-day runtime `cancov` projection is captured as
evidence for these fixtures. Even the raw gradient exercises the CoE melt terms
`amelt ∝ (1−cancov)` and `cmelt ∝ (1−0.8·cancov)` far more than the 05G evergreen
case; the first gate is reconciling raw `.man` values, wepppy seasonal projection,
and openWEPP runtime `cancov`. The climate axis is orthogonal and matters:
**maritime, near-0 °C-DJF, rain-on-snow** (HJ Andrews, Sleepers, Harvard, Hubbard
Brook) vs **cold continental** (Berthoud, Tenderfoot, Marcell, Mores Creek).

### 10.2 Physics to consider for melt incorporation / tuning

**Evidence update (2026-06-27, SNOWDENSITY-10.3.3 / 10.3.4 / 10.3.5c).** Three
results re-order the priorities below:

- **The shortwave/albedo modernization is NON-PROMOTION even at low `cancov`**
  (10.3.3, `LOW-CANOPY-NON-PROMOTION`): `coe_shortwave_albedo_v1` is neutral-to-
  worse across the whole gradient — neutral at conifer/open, **worse at
  deciduous** (`fail 3→4`), 7→8 verdict-bearing. The 05G open question is answered
  **no**: the low-`cancov` melt deficit is not a radiation/albedo problem, so
  item 2 below is closed as a default lever (density, 06B `9→5`, remains the win).
- **The maritime over-accumulation is partition-first, not rain-on-snow-first**
  (10.3.4, `PARTITION-THAW-FIRST`). Defect-eligible mechanism ranking, with
  quantitative evidence on the four paired Sleepers/Harvard surfaces: **(1)
  snow/rain partition near 0 °C** (21.2 m phase-ambiguous precip, 6.9 m "warm"
  modeled snow input), **(2) winter-thaw melt response** (167,815 positive-temp
  snowpack hours), **(3) sub-canopy longwave**, **(4) rain-on-snow heat —
  demoted** (CoE `dmelt` already carries it; magnitude 1.86 m). `precipitation
  bias` and `representativeness` are **forcing-limited** (report, do not tune);
  `wind undercatch` is **not-supported** (would worsen). So among items 3–5
  below, **partition (5) and thaw lead; rain-on-snow (4) is demoted.** Next =
  `SNOWDENSITY-10.3.5` partition/thaw-window candidate. Over-accumulation appears
  on **open *and* hardwood** surfaces (canopy-independent), reinforcing partition
  over sub-canopy longwave as the dominant cause.
- **The 10.3.5 phase candidate did not remediate snow depth** (10.3.5c,
  `PHASE-PARTITION-NEUTRAL-OR-WORSE`). The Harder-Pomeroy hourly selector
  improves observed-phase classification in Jennings validation, but coupled WAT
  snow-depth evidence worsened on all four paired Sleepers/Harvard surfaces:
  snow-control failures increased `1147 -> 1273`, and all seven maritime
  diagnostic surfaces accumulated more snow-depth days under the opt-in path. Keep
  `harder_pomeroy_hourly` opt-in only; do not promote it as a snow-depth
  remediation. The active route moves to 10.3.4 rank-2 **winter-thaw melt
  response** before sub-canopy longwave or rain heat.

1. **Canopy attenuation is the first-order control across this set.** The CoE melt
   already attenuates by `(1−cancov)` (radiation) and `(1−0.8·cancov)` (turbulent).
   The melt result is only as good as the **per-day winter `cancov`** the
   managements emit — step one is confirming the wepppy conifer/mixed/deciduous
   managements produce the right winter `cancov` per fixture (cross-repo). Tune the
   melt **coefficients/albedo**, never `cancov` to mask a snow error.
2. **Shortwave + albedo (the modernization — tested, NON-PROMOTION).** At
   less-attenuated `cancov`, the `SRF·(1−albedo)·I` term is active. **10.3.3
   tested this across the gradient and it did not earn value — neutral-to-worse,
   worse at deciduous.** Retained here as documentation of the rejected lever, not
   a live tuning target. Levers (if ever revisited): the Brock-2000 albedo state
   (verify the five constants against `references/copyrighted/brock2000.pdf` —
   05F review F5), snowfall albedo reset, and a **canopy shortwave transmissivity**
   for under-canopy radiation. The ★ stratified sites can isolate this only after
   the model-to-observation stratum mapping above is made explicit. Keep
   `SRF ≈ 0.0094` near-physical (Carenzo CV ~6%); expose at most a melt multiplier
   as a calibration handle.
3. **Longwave / sub-canopy enhancement — the canopy sign-flip.** Under dense
   conifer, melt is **longwave-dominated** (warm canopy emits downward); under
   open/deciduous it is shortwave-dominated. Lundquist (2013): above ~1 °C DJF
   mean, forest *reduces* snow duration (longwave wins); below it, forest *retains*
   snow (shading wins). The current `(1−cancov)` shortwave attenuation does **not**
   capture sub-canopy longwave — a forest longwave-enhancement term is the leading
   candidate for the warm-maritime forest fixtures, and this set is built to test
   it (it is also an Open Decision in §9).
4. **Rain-on-snow / advective melt — a leading frost-blocker hypothesis.** The
   non-SNOTEL over-accumulation is concentrated at **maritime, thaw-prone** sites
   and density compaction did not close it. That is consistent with under-melt
   during winter thaws and rain-on-snow, but phase partition, precipitation bias,
   wind/undercatch, and representativeness remain live confounders. Check/tune the
   CoE rain-heat term (warm rain at >0 °C transferring heat to the pack), verify
   the snow/rain partition first, and confirm the pack ablates during
   Vermont/Cascades January thaws. HJ Andrews (transient rain-snow) and
   Sleepers/Harvard/Hubbard Brook (maritime) are the test bed; this is the most
   direct route to adjudicating `NON-SNOTEL-OPT-IN-SNOW-CONTROL-FAILED`.
5. **Snow/rain partition + accumulation.** Over-accumulation can also be too much
   precip falling as snow; the rain-snow partition threshold is most sensitive at
   near-0 °C maritime sites. Verify the partition before attributing all
   over-accumulation to melt.
6. **Forcing limits (carry the existing tiering, §4).** Absolute SWE/depth are
   **forcing-limited** (DAYMET undercatch, high-relief lapse — Berthoud 3444 m,
   Hubbard Brook 222–1015 m; point-vs-hillslope), so reported and discounted;
   **density, densification, depth-SWE slope, onset/peak/melt-out timing, regime
   ordering, bias sign** are forcing-robust and carry verdict weight
   (`INV-SNOWFREEZE-050`). The ★ stratified open-vs-canopy observations are
   forcing-robust *within* a site (same climate), but require explicit modeled
   stratum mapping before they carry decisive canopy-attenuation verdicts.

### 10.3 Work-Package Tuning Sequence

Treat melt/canopy work as a short sequence of evidence-gated work packages, not as
a coefficient search. Each package must preserve the §4 discipline: physical
defaults without required site calibration, no shared-radiation tuning, no
validation-set fitting, opt-in or diagnostic confinement until ratified, and
profile/signature scoring rather than absolute-magnitude promotion.

1. **Canopy Projection Provenance.** Archive, for all eight `cancov_forest`
   fixtures, the raw `.man` values, wepppy projected per-day winter `cancov`, and
   openWEPP runtime `cancov`. Resolve the Sleepers pasture endpoint and any
   mixed/deciduous raw-vs-projected mismatch before interpreting melt evidence.
   Closure gate: per-fixture canopy trajectory evidence plus a disposition for
   every mismatch; no melt-physics changes.
2. **Canopy-Stratum Correspondence.** For Marcell and Harvard, map observed open /
   deciduous / conifer or hemlock / hardwood strata to the modeled hillslope(s).
   Decide whether one representative hillslope is defensible or whether paired
   open/under-canopy model variants must be generated. Closure gate: explicit
   observation-to-model stratum binding before those sites carry canopy verdicts.
3. **Gradient Melt Adjudication.** Re-run the 05G-style melt adjudication across
   the confirmed canopy gradient with `legacy_coe` and `coe_shortwave_albedo_v1`.
   Answer only whether the shortwave/albedo modernization earns value outside the
   high-evergreen regime. Closure gate: rubric profiles for conifer, mixed,
   deciduous, and pasture regimes; no coefficient retuning. **Complete (10.3.3):
   `LOW-CANOPY-NON-PROMOTION` — neutral-to-worse across the gradient, worse at
   deciduous. The modernization is not a default lever.**
4. **Maritime Over-Accumulation Diagnosis.** Decompose HJ Andrews, Sleepers,
   Harvard, and Hubbard Brook residuals into candidate causes: snow/rain
   partition, rain-on-snow heat, winter-thaw melt, precipitation bias,
   wind/undercatch, representativeness, and possible sub-canopy longwave.
   Closure gate: ranked blocker disposition with evidence for which mechanisms are
   defect-eligible versus forcing-limited. **Complete (10.3.4):
   `PARTITION-THAW-FIRST` — partition near 0 °C (1) and winter-thaw response (2)
   are the lead defect-eligible causes; sub-canopy longwave (3); rain-on-snow
   demoted (4); precip-bias/representativeness forcing-limited; wind not-supported.
   Next = `SNOWDENSITY-10.3.5` partition/thaw-window candidate.**
5. **`SNOWDENSITY-10.3.5` Robust Rain/Snow Partition (lead candidate).** 10.3.4
   isolated **snow/rain partition near 0 °C** as the #1 defect-eligible cause, so
   the partition is the first opt-in physics package. Per the robustness principle
   (the code is cheap with AI; the legwork is references / validation / testing),
   it targets the **most physically defensible** method, not the cheapest:
   - **Method:** Harder & Pomeroy (2013, R-57) **psychrometric hydrometeor energy
     balance** as primary — a law, not a fit or a table: it solves the falling
     particle's ice-bulb temperature from air temp + humidity and partitions on
     that, so it generalizes by construction and needs no site threshold. Susong
     (1999, R-54) **dew-point** table is the SNOBAL/SMRF-validated fallback. *Not*
     a tuned `RST` — Mariana's `−2 °C` for Oregon/DAYMET must be an *emergent
     output*, never a knob (and is partly a daily-mean artifact: Jennings shows the
     observed air-temp threshold is ~0–1 °C, above `−2 °C`).
   - **Resolution:** apply at openWEPP's existing **hourly** partition
     (`snow.hourly.stmtim.rst_c` lineage); hourly air temp + dew-point are already
     in the winter routine, so no daily-mean compromise.
   - **Validation (observed data, not rule-of-thumb):** score against the Jennings
     (2018, R-53) observed-phase corpus in `tests/fixtures/precip_phase_observed/`
     — the per-station 50% threshold (`file3`) and the 17.8M **hourly**
     temp/dew-point/RH/phase obs (`file2`, local). Maritime authority: Marks 1998
     (R-55, Oregon Cascades rain-on-snow), Kormos 2014 (R-56, transition zone).
   - **Closure gate (no site calibration):** one physical formulation must
     reproduce the observed maritime-low / continental-high threshold variation
     across the `cancov_forest` climates, and cut the 10.3.4 phase-ambiguous
     (`21.2 m`) / warm-snow (`6.9 m`) over-classification; conservation;
     opt-in/rollback isolation; contract-first (ADR-0011).
   - **Execution split:** `10.3.5.a` = new `crates/openwepp-meteorology` crate
     (the Harder-Pomeroy psychrometric `Ti` core + psychrometric primitives,
     **clean-room from the paper** — CHM is GPLv3 and excluded; MetPy BSD-3 as
     reference only); `10.3.5.b` = wire it into the hourly partition and validate
     against the Jennings corpus under the no-site-calibration gate; `10.3.5.c`
     = coupled WAT snow-depth impact gate. **Complete (10.3.5c):
     `PHASE-PARTITION-NEUTRAL-OR-WORSE` — opt-in WAT changed all seven surfaces
     but worsened paired snow-depth evidence (`1147 -> 1273` failures; all four
     paired surfaces worse). The selector remains opt-in and is not a snow-depth
     promotion candidate.**
6. **Subsequent candidate packages (one lever each).** After the partition,
   the winter-thaw melt response (10.3.4 #2) and then sub-canopy longwave
   (10.3.4 #3); each its own opt-in package under the same gate (conservation,
   independent operand reconstruction, rollback/default isolation, rubric
   improvement without site constants). Rain-heat is **not** revisited first
   (CoE `dmelt` already carries it; 10.3.4 #4).
7. **Activation / Retirement Decision.** Decide whether to promote, hold, or retire
   `coe_shortwave_albedo_v1` and any added opt-in melt lever. Closure gate:
   SNOTEL plus non-SNOTEL rubric profiles, explicit frost-attribution impact, and
   contract amendments for any production activation.

## 11. References / Authority

DOIs below are from the 2026-06-25 literature sweeps (two independent research
passes, load-bearing items cross-verified). Verify any DOI on retrieval; a few
forest-canopy DOIs were agent-inferred and are flagged.

### 11.1 Internal authority

- `SC-SNOWFREEZE-001` (`INV-SNOWFREEZE-047/048/049/050`,
  `TOL-SNOWFREEZE-007..011`).
- ADR-0011, ADR-0017, ADR-0024, ADR-0026.
- SNOWFROST-FIDELITY-D/E/F/H/I0 and SNOWDENSITY-01..04 work-package evidence.
- **WEPP Ch. 3 "Winter Hydrology"** (Savabi, Young, Benoit, Witte, Flanagan), in
  repo at `references/50201000/chap3.pdf` — **the production CoE energy-balance
  melt authority.** Carries `hrmelt = 0.0254 (amelt - bmelt + cmelt + dmelt)`,
  the radiation term `amelt = 0.0607 hrrad (1 - cancov)` and turbulent term
  `cmelt = 0.0188 U (1 - 0.8 cancov)(...)`, plus the snow settling/density code.
  Modernization = drive `hrrad` with gridded shortwave + an albedo state, keeping
  the `(1 - cancov)` canopy attenuation. Production implementation: the
  `amelt`/`cmelt` lineage in `crates/openwepp-hillslope-orchestrator/src/hydrology/`;
  the offline degree-day construct is in
  `crates/openwepp-runner/src/hillslope/snowbench_physics_bulk.rs`.
- `docs/backlog/20260605-snow-code-deferred-science-review.md`.

### 11.2 Literature in this repository

Vendored (`references/vendorable/`, redistributable — CC-BY or US-Gov public domain):

- **Anderson, E. A. (2006).** *Snow Accumulation and Ablation Model — SNOW-17.*
  NWSRFS User Documentation, NOAA/NWS. — `Anderson2006_SNOW17.pdf`. Seasonal
  (`MFMAX`/`MFMIN`) and forest-reduced degree-day melt-factor lineage.
- **Krinner, G., et al. (2018).** ESM-SnowMIP. *Geosci. Model Dev.* 11:5027–5049.
  DOI `10.5194/gmd-11-5027-2018`. — `Krinner2018_ESM-SnowMIP.pdf`. Complexity does
  not predict skill.
- **Lute, A. C., Abatzoglou, J. T., Link, T. E. (2022).** SnowClim v1.0.
  *Geosci. Model Dev.* 15:5045–5071. DOI `10.5194/gmd-15-5045-2022`. —
  `Lute2022_SnowClim.pdf`. Shallow-snow SNOBAL-lineage stability (§2.2.7).
- **Vionnet, V., et al. (2012).** Crocus / SURFEX v7.2. *Geosci. Model Dev.*
  5:773–791. DOI `10.5194/gmd-5-773-2012`. — `Vionnet2012_Crocus.pdf`.
  Reference snow density/albedo implementation.
- **Gupta, A., et al. (2023).** *Hydrol. Earth Syst. Sci.* 27:191–212.
  DOI `10.5194/hess-27-191-2023`. — `Gupta2023_HESS.pdf`. Shortwave distribution
  shifts melt timing *and* ET slope together (the radiation/ET-coupling evidence).

Local cache (`references/copyrighted/`, gitignored — read-only, not redistributable):

- **Anderson, E. A. (1976).** *A Point Energy and Mass Balance Model of a Snow
  Cover.* NOAA Tech. Report NWS-19. — `noaa_6392_DS1.pdf` (+ OCR `noaa_6392_DS1.md`).
  Energy-balance melt + §III density/compaction (PTM/POC). US-Gov public domain
  (eligible to move to `vendorable/`).
- **Ohmura, A. (2001).** Physical basis for the temperature-based melt-index
  method. *J. Appl. Meteorol.* 40(4):753–761.
  DOI `10.1175/1520-0450(2001)040<0753:PBFTTB>2.0.CO;2`. —
  `Ohmura2001_meltindex.pdf`. The authority for what the melt factor lumps
  (longwave + sensible heat) and misses (net shortwave).
- **Ménard, C. B., et al. (2021).** *Bull. Amer. Meteor. Soc.* 102(1):E61–E79.
  DOI `10.1175/BAMS-D-19-0329.1`. — `Menard2021_BAMS.pdf`. Implementation
  correctness can dominate model choice.
- **Shen, D. (2011/2012).** WSU MS thesis, WEPP snow distribution. —
  `D_Shen_020312.pdf`.

Melt-physics papers acquired 2026-06-25 (operator-supplied; verified full text,
identities confirmed by title/page check):

- **Pellicciotti, F., et al. (2005).** An enhanced temperature-index glacier melt
  model including the shortwave radiation balance, Haut Glacier d'Arolla.
  *J. Glaciol.* 51(175):573–587. DOI `10.3189/172756505781829124`. —
  `pellicciotti2005.pdf`. *The* ETI reference form; `TF = 0.05`, `SRF = 0.0094`.
- **Carenzo, M., et al. (2009).** Transferability and robustness of an ETI
  glacier-melt model. *J. Glaciol.* 55(190):258–274.
  DOI `10.3189/002214309788608804`. — `carenzo2009.pdf`. `SRF` CV ~6% vs `TF`
  CV ~56% (the transferability asymmetry).
- **Hock, R. (1999).** A distributed temperature-index ice- and snowmelt model
  including potential direct solar radiation. *J. Glaciol.* 45(149):101–111.
  DOI `10.3189/S0022143000003087`. — `hock1999.pdf`.
- **Brock, B. W., Willis, I. C., Sharp, M. J. (2000).** Measurement and
  parameterization of albedo variations, Haut Glacier d'Arolla.
  *J. Glaciol.* 46(155):675–688. DOI `10.3189/172756500781832675`. —
  `brock2000.pdf`. Accumulated-positive-temperature albedo decay.
- **Walter, M. T., Brooks, E. S., McCool, D. K., King, L. G., Molnau, M.,
  Boll, J. (2005).** Process-based snowmelt modeling: does it require more input
  data than temperature-index modeling? *J. Hydrology* 300:65–75.
  DOI `10.1016/j.jhydrol.2004.05.002`. — `walter2005.pdf`. **WEPP/USDA-ARS-PNW
  lineage; uncalibrated energy balance from `Tmax`/`Tmin` beats best-fit
  temperature index.**
- **Marks, D., Domingo, J., Susong, D., Link, T., Garen, D. (1999).** A spatially
  distributed energy balance snowmelt model (SNOBAL). *Hydrol. Process.*
  13:1935–1959.
  DOI `10.1002/(SICI)1099-1085(199909)13:12/13<1935::AID-HYP868>3.0.CO;2-C`. —
  `marks1999.pdf`. The Anderson-1976 compaction reference implementation.
- **Magnusson, J., et al. (2015).** Evaluating snow models with varying process
  representations. *Water Resour. Res.* 51:2707–2723.
  DOI `10.1002/2014WR016498`. — `magnusson2015.pdf`. Fewer/more-physical
  parameters transfer better.
- **Lundquist, J. D., Dickerson-Lange, S. E., Lutz, J. A., Cristea, N. C. (2013).**
  Lower forest density enhances snow retention in warmer-winter regions.
  *Water Resour. Res.* 49:6356–6370. DOI `10.1002/wrcr.20504`. —
  `lundquist2013.pdf`. The ~1 °C DJF sign-flip in the net canopy melt effect.
- **Varhola, A., Coops, N. C., Weiler, M., Moore, R. D. (2010).** Forest canopy
  effects on snow accumulation and ablation: an integrative review.
  *J. Hydrology* 392:219–233. DOI `10.1016/j.jhydrol.2010.08.009`. —
  `varhola2010.pdf`.

### 11.3 To track down (still not obtained)

- **Hock, R. (2003).** Temperature index melt modelling in mountain areas.
  *J. Hydrology* 282:104–115. DOI `10.1016/S0022-1694(03)00257-9` (Elsevier).
- **Hock, R. (2005).** Glacier melt: processes and modelling review.
  *Prog. Phys. Geogr.* 29(3):362–391. DOI `10.1191/0309133305pp453ra` (Sage).
- **Lapo, K. E., et al. (2015).** Impact of downwelling-irradiance errors on snow
  simulations. *Water Resour. Res.* 51:1649–1670. DOI `10.1002/2014WR016259`
  (Wiley/AGU).
- **Beven, K. (2006).** A manifesto for the equifinality thesis.
  *J. Hydrology* 320:18–36. DOI `10.1016/j.jhydrol.2005.07.007` (Elsevier).
- **Sicart, J. E., et al. (2006).** Incoming longwave radiation to melting snow
  under forest. *Hydrol. Process.* 20:3697–3708. DOI `10.1002/hyp.6383 ?`
  (Wiley; DOI agent-inferred).
- **Rutter, N., et al. (2009).** SnowMIP2 forest snow-process evaluation.
  *J. Geophys. Res.* 114:D06111. DOI `10.1029/2008JD011063 ?` (AGU; DOI
  agent-inferred).

Public-domain, locate official PDF (no paywall — just not yet fetched):

- **Oleson, K. W., et al. (2013).** CLM 4.5 Technical Description. NCAR/TN-503+STR.
- **USACE (1956).** *Snow Hydrology.* U.S. Army Corps of Engineers, North Pacific
  Division (NTIS / archive). Origin of the CoE melt + albedo-decay curves WEPP
  inherits.
- **Allen, R. G., Pereira, L. S., Raes, D., Smith, M. (1998).** Crop
  evapotranspiration (FAO-56). FAO Irrigation & Drainage Paper 56 (open — FAO).
