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
    coe_liquid_holding_capacity_v1  # current default production CoE melt boundary
    legacy_coe                 # explicit rollback/test selector
    coe_shortwave_albedo_v1    # opt-in modernized CoE melt

snow_model =
    physics_bulk_density_compaction_v1  # current default production depth/density lineage
    legacy_wepp                # explicit rollback/test selector
```

- SNOWDENSITY-10.3.15 activated `coe_liquid_holding_capacity_v1` plus
  `physics_bulk_density_compaction_v1` as the direct-production no-env default
  under the active `522 kg m^-3` cap. Legacy melt/density remain explicit
  rollback/test selectors, not parser/runfile/user CLI surfaces.
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

**Evidence update (2026-06-27, SNOWDENSITY-10.3.3 / 10.3.4 / 10.3.5c / 10.3.6).** Four
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
- **The winter-thaw melt response is defect-eligible** (10.3.6,
  `WINTER-THAW-MELT-RESPONSE-DEFECT-ELIGIBLE`). Across the four paired
  Sleepers/Harvard surfaces, the event-window diagnostic found `219` observed
  thaw-ablation windows and `132` under-ablation windows (`0.603` fraction), with
  `24.105 m` aggregate snow-depth loss deficit, `19,166` positive-temperature
  snowpack hours, `8.685 m` raw CoE melt, `4.628 m` modeled SWE loss, and only
  `0.190 m` warm-rain heat equivalent. This moves the next active route to a
  contract-first **opt-in winter-thaw melt-response correction** package. Rain
  heat remains contextual and sub-canopy longwave remains the following separate
  lever.

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
4. **Winter-thaw under-ablation — the confirmed over-accumulation lever
   (adjudicated 2026-06-27).** The hypotheses here have been tested against
   *coupled* evidence (see the §10.2 callout), which overruled the 10.3.4
   diagnostic ranking: the snow/rain **partition** was the #1 diagnostic suspect
   but is **neutral-or-worse for snow depth** in the coupled model (10.3.5c), and
   **rain-on-snow heat is demoted** (CoE `dmelt` already carries it; 0.19 m,
   10.3.4/10.3.6). The dominant lever is **winter-thaw under-ablation** (10.3.6):
   on the paired Sleepers/Harvard surfaces the pack under-ablates in ~60% of
   observed thaw windows (24.1 m aggregate depth-loss deficit). The **tell is a
   melt-realization gap** — raw CoE melt `8.69 m` but modeled SWE loss only
   `4.63 m`, so ~half the computed melt energy is not realized as pack loss. The
   correction (§10.3 step 6) targets that melt→SWE-loss application — *not* the
   rain-heat or partition terms — to adjudicate
   `NON-SNOTEL-OPT-IN-SNOW-CONTROL-FAILED`.
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
7. **Open-surface ablation via the SNOBAL two-layer energy balance (deferred
   after narrow-lever non-promotions).** After bundle activation (§10.3), the residual splits into two
   opposite-sign defects: a density-arm-induced under-persistence tail, and an
   over-persistence **mass excess** that compaction cannot remove (cap-limited
   depletion, concentrated at wind-exposed open surfaces — Harvard open, Sleepers
   open field). The over-persistence tail is an **ablation deficit**: openWEPP
   carries too much SWE at open exposures because the CoE melt under-realizes mass
   loss there. The reference mechanism is the SNOBAL two-layer point energy-mass
   balance — authority **Marks et al. 1999** (`references/copyrighted/marks1999.pdf`).
   The `pysnobal` `libsnobal` C (USDA-ARS-NWRC clone, **CC0 1.0 / public-domain
   dedication** declared in `setup.py`; US-Government-work origin) is now usable as
   a **direct implementation reference and port source**, not only an equation
   cross-reference — CC0 is Apache-2.0-compatible and outside the `deny.toml`
   GPL/AGPL/LGPL denials (no clean-room restriction). Ground the physics in Marks
   1999, port the structure from `libsnobal`, and record the clone commit + the
   `setup.py` CC0 declaration as provenance (no standalone `LICENSE` file ships in
   the repo).
   - **Two-layer thermal structure (`_calc_layers.c` / `_adj_layers.c`).** A
     fixed-thickness **active surface layer** (`max_z_s_0`, ~0.25 m) plus a lower
     layer, each with its own temperature and **cold content**
     (`T_s_0`/`cc_s_0`, `T_s_l`/`cc_s_l`) over one bulk density. It collapses to a
     single layer — and dumps a sub-threshold remnant to liquid — for shallow
     snow (the numerical-robustness rule the bulk model lacks). The structure
     exists for the energy balance: the thin surface layer responds fast to
     forcing and sets the surface temperature, so melt is
     **surface-temperature-driven, not bulk-average** — the surface ripens and
     ablates while the base stays cold, exactly the open-exposure regime.
   - **Surface energy budget (`_e_bal.c`):**
     `ΔQ_0 = R_n + H + L_v·E + G_0 + M` — net all-wave radiation, turbulent
     **sensible** heat `H`, turbulent **latent** heat `L_v·E`, surface↔lower
     conduction `G_0`, and advected precip heat `M`; total
     `ΔQ = ΔQ_0 + G − G_0` adds soil heat `G` at the lower boundary. The turbulent
     `H`/`L_v·E`/`E` are bulk-aerodynamic (Monin-Obukhov; `_h_le.c` / `hle1.c`),
     so **wind drives them** — the open-exposure amplifier.
   - **Melt realization (`_snowmelt.c`):** surface energy
     `Q_0 = ΔQ_0·Δt + cc_s_0`; melt only when `Q_0 > 0` (`melt = Q_0 / L_f`),
     else the deficit deepens cold content. Melt is **cold-content-gated at the
     surface layer**; lower-layer melt follows only after the surface ripens.
   - **Sublimation / direct mass loss (`_evap_cond.c`) — the term CoE most likely
     lacks.** The latent flux removes **mass** directly: `E_s = E·Δt` reduces SWE
     and depth, liquid favored over ice by the vaporization/sublimation ratio
     `0.882`. At open, windy, dry sites this is a **first-order SWE sink with no
     melt-energy analogue** — if openWEPP never sublimates, open-exposure SWE
     stays too high regardless of melt tuning. This is the prime suspect for the
     open-surface mass excess.
   - **Mapping to openWEPP's CoE melt.** CoE already carries radiation (`amelt`),
     a wind/temperature term (`cmelt`, ~`H`), and rain/advected heat (`dmelt`,
     ~`M`). What it lacks: **(a) a turbulent latent-heat / sublimation mass-loss
     term** (`L_v·E → E_s`) and **(b) the surface-vs-bulk cold-content
     distinction** (a fast surface layer that ripens/melts ahead of the bulk).
     (a) is the cheap, likely-dominant increment for the open mass tail; (b) is
     the structural two-layer addition.
   - **Staging (one-lever discipline).** *Stage A* — add the **sublimation /
     latent mass-loss term** to the existing CoE energy balance as an opt-in
     candidate: a single additive, mass-conserving SWE sink (sublimated mass
     leaves as vapor, tracked in the WB). *Stage B* (only if A under-delivers) —
     adopt the **two-layer surface cold-content structure** for surface-driven
     melt. Keep the activated bundle as default and full rollback; contract-first
     `SC-SNOWFREEZE-001` amendment; turbulent-transfer constants from Marks 1999,
     **not** fitted to fixtures.
   - **Falsifiable gates.** (1) cuts the cap-limited over-persistence / mass-excess
     tail at the open surfaces (Harvard open, Sleepers open field) in the coupled
     WAT gate; (2) **does not worsen the under-persistence tail** (the standing
     bidirectional guardrail — sublimation removes mass, so watch shallow packs and
     meltout timing); (3) sublimation magnitude falls in the literature range for
     the regime, not tuned to a fixture; (4) whole-model conservation closes (the
     vapor sink is balanced in the water balance).

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
   Next = `SNOWDENSITY-10.3.5` partition/thaw-window candidate. Follow-up
   10.3.6 adjudicated the rank-2 thaw branch as defect-eligible after 10.3.5c
   failed to remediate snow depth.**
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
6. **Winter-Thaw Melt Response Correction.** Complete in
   `docs/work-packages/20260627-snowdensity-10-3-7-winter-thaw-melt-response-correction-001/`.
   10.3.6 confirmed the rank-2 thaw branch as
   `WINTER-THAW-MELT-RESPONSE-DEFECT-ELIGIBLE`: `132/219` paired thaw-ablation
   windows under-ablated by the diagnostic threshold, with `24.105 m`
   depth-loss deficit and only `0.190 m` warm-rain heat equivalent. 10.3.7
   amended `SC-SNOWFREEZE-001` v94 and added the opt-in
   `coe_winter_thaw_state_loss_v1` candidate to test the localized
   melt-to-SWE-loss application defect. The candidate preserves CoE melt terms,
   radiation, canopy, phase partition, density constants, rain heat,
   sub-canopy longwave, frost, fixtures, public schemas, and default
   `legacy_coe`; its only delta routes positive thaw `wmelt` to state loss when
   the legacy below-`350 kg m^-3` gate would absorb it as density-only
   compaction. Paired Sleepers/Harvard evidence improves but does not close all
   residuals: under-ablation windows `132 -> 108`, aggregate deficit
   `24.105 m -> 17.629 m`, modeled depth loss `15.868 m -> 26.400 m`, routed
   melt `5.895 m -> 11.235 m`, and SWE loss `4.628 m -> 10.615 m`. Operator
   review then forced the two load-bearing gates into the package: active-ledger
   conservation/routing residuals are zero, and the real direct-production WAT
   coupled rerun improves snow-control failures `1147 -> 978` with no paired
   surface worse. This is an opt-in improvement, not activation or full
   snow-control closure: `978/1415` coupled paired rows still fail snow control,
   so the remaining residuals route to the next one-lever package.
   - **Durable fix completed as opt-in 10.3.8:** `docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/`
     amended `SC-SNOWFREEZE-001` v95 and added `coe_liquid_holding_capacity_v1`.
     The candidate replaces the `350 kg m^-3` density-gate proxy with a persistent
     retained-liquid snow-lane store and a physical holding-capacity drainage
     bound from in-repo authority (`max_liquid_water_volume_fraction = 0.01`,
     Marks R-55, Anderson 1976 NWS-19, SNOW-17 PLWHC, and SNOBAL-lineage
     `h2o_max`/excess-runoff semantics). It preserves `legacy_coe` default and
     rollback behavior, CoE melt terms, radiation, canopy, phase partition,
     density constants, rain heat, sub-canopy longwave, frost, fixtures, public
     schemas, parser/runfile/user surfaces, and compatibility runtime. Event-
     window evidence improves under-ablation `132 -> 94` and aggregate deficit
     `24.105 m -> 15.506 m` with conservation closed. Real coupled direct-
     production WAT evidence improves snow-control failures `1147 -> 761` with
     no paired surface worse. This is still **not default activation or full
     snow-control closure**: `761/1415` paired rows still fail snow control and
     three surfaces remain observation-blocked, so subsequent work must target
     the remaining one-lever snow-control residuals without tuning the capacity
     constant to fixtures.
   - **March/April residual attribution completed as diagnostic 10.3.9:**
     `docs/work-packages/20260627-snowdensity-10-3-9-march-april-residual-attribution-001/`
     consumed the 10.3.8 coupled WAT artifact and classified the remaining
     paired Sleepers/Harvard failures. March/April accounts for `282/761`
     residual failures (`37.1%` of failures) and fails `282/463` paired
     March/April rows (`60.9%`). Failures are shared across open field,
     hardwood, and open covers (`112`, `109`, and `61` rows respectively).
     Attribution is dominated by depth-only over-persistence, density/compaction,
     and patchy meltout or snow-cover depletion (`127 + 86 + 26 = 239/282`),
     with `43` under-persistence rows and no defended SWE-excess mass verdict
     because Harvard SWE/depth/density correspondence is source-caveated. The
     next one-lever route is spring pack-depletion and compaction adjudication,
     not a new mass/precipitation correction. Hubbard Brook mixed/deciduous and
     HJ Andrews remain observation-blocked for residual verdicts until paired
     snow-depth observations are installed.
   - **Spring pack-depletion/compaction adjudication completed as diagnostic
     10.3.10:**
     `docs/work-packages/20260627-snowdensity-10-3-10-spring-pack-depletion-compaction-adjudication-001/`
     tested whether the 10.3.9 over-persistent March/April rows can fit the
     observed depth tolerance by compaction alone under the existing
     `SC-SNOWFREEZE-001` `522 kg m^-3` density cap. Result:
     `SPRING-COMPACTION-FIRST`. Of `282` failed March/April rows, `190` are
     compaction-only feasible within the cap, `33` are cap-limited depletion
     required, `16` are patchy meltout or depletion required, and `43` are
     under-persistence. Depletion-required rows remain real (`49/282`, row-sum
     required SWE depletion at cap `1.230 m`), especially Harvard open (`23`
     rows) and Sleepers open field (`21` rows), but they are a tail rather than
     the leading lever. Next route: an opt-in spring compaction/densification
     candidate that preserves the cap and mass conservation; only after that
     gate should a separate spring depletion / patchy snow-cover process be
     considered.
   - **Spring compaction/densification candidate completed as opt-in 10.3.11:**
     `docs/work-packages/20260627-snowdensity-10-3-11-spring-compaction-densification-candidate-001/`
     amended `SC-SNOWFREEZE-001` v96 and added
     `physics_bulk_spring_densification_v1` to test whether wet-snow compaction
     acceleration reduces the 10.3.10 compaction-feasible residuals. It does
     **not** promote: the existing `physics_bulk_density_compaction_v1` arm,
     when paired with `coe_liquid_holding_capacity_v1`, improves the prior
     10.3.8 coupled WAT failures `761 -> 498`, but the new spring
     densification arm worsens that density baseline `498 -> 502` with three
     paired surfaces worse. The cause is over-densification into
     under-persistence after the bulk compaction arm has already consumed the
     available headroom: `harvard_hardwood` has `0` remaining compaction-only
     rows, yet the spring add-on produces `64` under-persistence rows and
     worsens failures `153 -> 156`. The 10.3.10 diagnosis remains correct
     (compaction is a dominant lever), but the proposed spring-rate mechanism is
     exhausted. The remaining active baseline is therefore the holding-capacity +
     existing density-compaction bundle, not
     `physics_bulk_spring_densification_v1`. Snow-control remains blocked
     (`498/1415` paired rows fail under the best current bundle). Carry the
     `522 kg m^-3` cap as an explicit authority check, not a hidden tuning knob.
   - **Bundle activation adjudication completed as opt-in hold 10.3.12:**
     `docs/work-packages/20260627-snowdensity-10-3-12-bundle-activation-adjudication-001/`
     amended `SC-SNOWFREEZE-001` v97 with `INV-SNOWFREEZE-069` and
     `OBL-SNOWFREEZE-P-044`, then reran the real direct-production WAT path for
     `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`.
     Result: `HOLD-OPT-IN-BUNDLE`. The bundle is the best current path:
     default `1147` failures, holding-capacity-only `761`, bundle `498`, and
     spring densification `502`; no paired surface worsens versus
     holding-capacity-only. Post-review `SC-SNOWFREEZE-001` v98 adopts
     Activation Policy B: zero paired snow-depth failures are not required for
     default activation, but a global default change requires strict improvement
     over the current default on gate-eligible paired-snow surfaces plus
     full-model-surface no-regression evidence. This package did not produce the
     full-surface evidence, so it remains `HOLD-OPT-IN-BUNDLE`. Remaining
     failures split into modeled-over-observed `264` and modeled-under-observed
     `234`; month counts are November `5`, December `46`, January `138`,
     February `112`, March `170`, and April `27`. March/April classes are now
     dominated by under-persistence (`128`), with cap-limited depletion (`33`),
     patchy depletion (`16`), and only `20` compaction-only rows. Frost
   attribution remains blocked by snow-control residuals.
   - **Residual-tail / Policy-B diagnostic completed as 10.3.13:**
     `docs/work-packages/20260627-snowdensity-10-3-13-residual-policy-b-diagnostic-001/`
     amended `SC-SNOWFREEZE-001` v99 with `INV-SNOWFREEZE-070` and
     `OBL-SNOWFREEZE-P-045`, then classified date-level transitions across
     current default, holding-capacity-only, combined bundle, and rejected
     spring densification. Result: `HOLD-ACTIVATION-EVIDENCE-MISSING`. The
     bundle remains strictly better than default (`1147 -> 498`) but lacks
     Policy-B full-surface no-regression evidence. The under-persistence tail is
     now mechanism-cost evidence: `177/234` bundle under-persistence failures
     were induced from holding-only pass or over-persistence rows (`150` from
     pass, `27` from over), while `57` persisted from holding-only
     under-persistence. This supports the bulk-compaction mechanism-cost
     hypothesis and argues against additional compaction-rate acceleration.
     March/April classes remain cap-limited depletion `33`, patchy depletion
     `16`, compaction-feasible `20`, and under-persistence `128` under the
   active `522 kg m^-3` cap. The `550 kg m^-3` SNOBAL cap re-anchor remains a
   separate contract-first follow-up, not a hidden diagnostic change.
   - **Policy-B no-regression / cap-authority diagnostic completed as
     10.3.14:**
     `docs/work-packages/20260627-snowdensity-10-3-14-policy-b-no-regression-cap-authority-001/`
     amended `SC-SNOWFREEZE-001` v100 with `INV-SNOWFREEZE-071` and
     `OBL-SNOWFREEZE-P-046`, consumed the 10.3.12 real direct-production bundle
     report plus the 10.3.13 residual diagnostic, and ran the full workspace
     no-regression gate under the package-bound bundle selectors
     `OPENWEPP_SNOWDENSITY1038_MELT_MODEL=coe_liquid_holding_capacity_v1` and
     `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_density_compaction_v1`.
     Result: `READY-FOR-ACTIVATION-PACKAGE-UNDER-ACTIVE-CAP`. Composite trace
     state closure was clean (`1.11e-16 m` max SWE-depth-density residual, zero
     cap exceedances), and no production cap/default/schema/fixture/runtime-
     selector change was made. The active `522 kg m^-3` cap is sufficient for a
     separate default-activation package. The `550 kg m^-3` SNOBAL cap
     projection remains mixed follow-up evidence only: cap-pinned paired
     failures project `105 -> 102`, but `3` passing rows become projected
     under-persistence, so dynamic cap re-anchoring requires its own contract-
   first implementation and rerun if pursued.
   - **Default activation completed as 10.3.15:**
     `docs/work-packages/20260627-snowdensity-10-3-15-default-activation-active-cap-001/`
     amended `SC-SNOWFREEZE-001` v101 with `INV-SNOWFREEZE-072` and
     `OBL-SNOWFREEZE-P-047`, activated the no-env direct-production default
     bundle `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`,
     retained `legacy_coe`/`legacy_wepp` as explicit rollback/test selectors,
     and preserved parser/runfile/user CLI, fixture, output-schema,
     compatibility-runtime, Qwet/frzftp, and active-cap boundaries. The
     activation is an improvement, not snow/frost closure: `498/1415` paired
     snow-depth rows still fail control, and frost attribution remains blocked
     by `SNOW-CONTROL-RESIDUALS-REMAIN`.
     - **Activation basis (documented honestly).** The activation rests on
       **mass conservation + improved snow input + reversibility**, not on a
       demonstrated downstream output-diff. The bundle is mass-conserving
       (composite snow-state closure at machine epsilon), the snow/melt signal
       is validated closer to observed (`1147 -> 498`), and
       `legacy_coe`/`legacy_wepp` selectors restore prior behavior. The
       Policy-B "no-regression" gate as met is **workspace-suite-level** (the
       existing suite passes under the bundle selectors); the activation commit
       changed **no** downstream output goldens, so a separate bundle-vs-legacy
       diff of snow-affected erosion / water-balance / watershed outputs was
       **not** run. That is acceptable here because conservation guarantees mass
       closure ("the water has to go somewhere") and the upstream kernel
       improved — the downstream change is **improved-input propagation, not a
       regression**. **User-facing consequence:** snow-affected
       runoff / erosion / water-balance / watershed outputs **differ** from the
       prior default; total water is conserved and rollback is available. See
       [snow-default-activation-behavior-change](../snow-default-activation-behavior-change.md).
7. **Subsequent candidate packages (one lever each).** After winter-thaw melt
   response and the 10.3.9/10.3.10 residual adjudication, 10.3.11 retired the
   spring wet-time densification candidate and 10.3.12 held the best current
   bundle as opt-in only; 10.3.13 then confirmed that most under-persistence is
   newly induced by the bulk-compaction arm, and 10.3.14 supplied the Policy-B
   workspace-suite no-regression evidence (existing suite passes under the
   bundle selectors) plus composite snow-state conservation closure under the
   active `522 kg m^-3` cap; 10.3.15 then activated that bundle by default
   (activation basis and user-facing output-change consequence documented in
   step 6 above and the behavior-change note). Do not pursue another
   wet-compaction acceleration or density-rate variant without new external
   authority and a different residual class.
   - **Open-surface ablation Stage A completed as opt-in non-promotion
     10.3.16:**
     `docs/work-packages/20260627-snowdensity-10-3-16-open-surface-ablation-stage-a-001/`
     amended `SC-SNOWFREEZE-001` v102 with `INV-SNOWFREEZE-073`,
     `OBL-SNOWFREEZE-P-048`, and opt-in
     `coe_open_sublimation_stage_a_v1`. The candidate's only algorithmic delta
     from the activated default is a Marks-lineage turbulent latent mass-loss
     sink (`snow_sublimation`) that removes SWE as vapor and keeps routed liquid
     melt separate. Real coupled direct-production WAT/trace evidence proves the
     selector reached the snow partition and conservation closed (max snow-state
     residual `5.55e-17 m`); sublimation magnitude stayed within the provisional
     literature sanity envelope (total traced `0.586 m`, max daily-lane
     `0.0048 m`). It still **does not promote**: the open-surface cap-limited
     tail improved only `30 -> 27`, while under-persistence worsened `54 -> 57`
     (`sleepers_south_field` `19 -> 22`; Harvard open neutral `35 -> 35`).
     The standing bidirectional guardrail failed, so the Stage A selector remains
     diagnostic-only, default/rollback/output schema/fixtures/density cap/frost
   attribution stay unchanged, and standalone sublimation is not a production
   lever.
   - **Shallow-pack compaction guard completed as opt-in non-promotion
     10.3.17:**
     `docs/work-packages/20260627-snowdensity-10-3-17-shallow-pack-compaction-guard-001/`
     amended `SC-SNOWFREEZE-001` v103 with `INV-SNOWFREEZE-074`,
     `OBL-SNOWFREEZE-P-049`, and opt-in
     `physics_bulk_shallow_guard_v1`. The only algorithmic delta from the
     activated density arm is a `0.25 m` authority-derived shallow-pack guard
     that reduces dry/wet densification increments below the Marks/SNOBAL active
     layer threshold. It still **does not promote**: real coupled WAT evidence
     improves induced under-persistence only `177 -> 176`, recovers `0` induced
     under rows at `harvard_hardwood` (`73 -> 73`), worsens over-persistence
     `264 -> 267` with `3` new over rows from non-over rows, and worsens total
     snow-control failures `498 -> 500`. Local SWE-depth-density identity closes,
     but downstream snow mass terms change (`3.342e-03 m` max mass-term delta),
     so the "only density aggressiveness" whole-model gate fails. The selector
     remains diagnostic-only; no default/cap/schema/fixture/user-surface/frost
     change is authorized.
   - **Carry-forward after Stage A and shallow guard.** The open mass tail is still defect-shaped,
     but the next ablation package must not simply increase vapor loss. It needs
     a mechanism that can reduce excess open-surface SWE without tipping shallow
     packs into earlier meltout: the likely route is the Stage B two-layer
     surface cold-content / surface-temperature structure from §10.2 item 7, or
     another independently authorized open-exposure process with the same
     bidirectional guardrail. But narrow humid-subset levers have now washed in
     both directions (10.3.11 densification, 10.3.16 sublimation, 10.3.17 shallow
     guard), which is the signal that the **instrument is too narrow**. The next
     step is the cross-SNOTEL rubric diagnostic (step 8) to widen the instrument
     before committing more ablation or density work — **not Stage B by
     default.** Under-persistence remains large (`234` rows total;
     `128` March/April rows) and mostly induced by the density arm; it is a known
     mechanism cost to carry into activation release notes and follow-up residual
     work, not an authorization for another density-rate acceleration or larger
     ablation sink. Treat patchy meltout as structural/non-target unless a
     separate correspondence package makes it verdict-bearing. Check the physical
     defensibility of the `522 kg m^-3` ripe-snow cap as its own contract-first
     authority package if needed, not as a fitted constant; 10.3.14 showed the
     SNOBAL `_h2o_compact` `550 kg m^-3` cap projection is mixed and not required
     for activation, so any cap re-anchor now requires a dynamic implementation
     package and full rerun rather than same-SWE projection evidence. Evaluate
     sub-canopy longwave / forest energy (10.3.4 #3) only if these spring-pack
     gates do not close the residuals. Revisit rain-on-snow heat only if
     event-window reconstruction proves the existing CoE `dmelt` path is
     numerically inactive during observed failures. Each lever must remain opt-in
     until the same gate is met: conservation, independent operand
     reconstruction, rollback/default isolation, and rubric improvement without
     site constants.
8. **Cross-SNOTEL Mechanism × Legacy Rubric Diagnostic (queued after the
   shallow-pack guard non-promotion).** The one-lever arc (10.3.5–10.3.17) gated almost entirely
   on the narrow `cancov_forest` humid-New-England paired subset (depth-dominated,
   Sleepers/Harvard), where several levers washed out — partition (10.3.5c),
   spring densification (10.3.11), and open sublimation (10.3.16). Repeated washes
   on one regime are the signal that the instrument is too narrow. The richer
   `tests/fixtures/snotel_observed/` corpus (five mountain climates — Paradise WA,
   Snowbird UT, CSS Lab CA, Mica Creek ID, Niwot CO — with observed **SWE + depth
   + density**, built in SNOWFROST-FIDELITY-H) was sidelined for the activation
   arc. Before committing more levers, run a diagnostic that:
   - applies the `INV-SNOWFREEZE-050` forcing-robust fidelity rubric (the
     frost-style rubric) across all five SNOTEL climates plus the `cancov_forest`
     paired set for continuity;
   - scores **every** mechanism and the activated bundle, plus **legacy**
     (`legacy_coe + legacy_wepp`) and PySnobal **as flag profiles, never targets**
     (ADR-0017);
   - decomposes the residual into **mass (SWE) vs density vs depth directly** — the
     SNOTEL corpus has all three, so no cap-test inference (the limitation that
     caveated 10.3.9 on the depth-only sites);
   - carries verdicts only on **forcing-robust** signatures (density, depth-SWE
     slope, densification trajectory, timing, regime ordering, bias sign) and
     reports forcing-limited absolute SWE/depth without verdict weight
     (`INV-SNOWFREEZE-050`);
   - answers which mechanism improves which signature in which climate regime,
     whether the humid-New-England residual is representative, and the right next
     *global* lever.
   Sequencing: the shallow-pack compaction guard (10.3.17) closed as
   non-promotion, so this diagnostic now assesses all mechanisms cross-climate
   and carries `physics_bulk_shallow_guard_v1` as a rejected opt-in profile, not
   an in-bundle candidate. Diagnostic-only:
   no promotion/activation/cap/schema/fixture/frost change, no site calibration.
   This also feeds the open **frost-attribution-threshold** question (what residual
   is "good enough"), which can only be answered on a representative cross-climate
   instrument rather than the narrow humid subset.
   - **Completed as 10.3.18 (`DIAGNOSTIC-COMPLETE-NO-PROMOTION-DECISION`).**
     `docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/`.
     Reads: humid-New-England is **NOT representative** of the mountain SNOTEL
     signature set (`fail_cell_jaccard = 0.2`). On the forcing-robust rubric the
     **activated bundle (17 fail / 172) is marginally below legacy (16 / 176)** — a
     trade: it fixed the density bias (legacy median `−55.6 kg/m^-3` → bundle `≈0`)
     but **degraded snowmelt/accumulation timing across all five mountain climates**
     (peak-SWE / peak-depth / meltout / duration dates) and **deepened the
     depth/SWE under-bias** (the under-persistence mechanism cost, now confirmed
     global, not a humid-NE artifact). **`harder_pomeroy_partition` (activated
     bundle + Harder-Pomeroy hourly phase) is the top lever (15 / 179; 9 better,
     2 worse cells)** — fixes timing across every climate, wrongly rejected at
     10.3.5c on the now-invalidated humid gate. Sublimation (10.3.16) is worst
     (20 / 153); shallow-pack guard (10.3.17) is neutral (172, non-promoted);
     PySnobal is a weak flag (28 / 11).
   - **Decisions (operator, 2026-06-28).**
     1. **The cross-SNOTEL forcing-robust rubric is the standing primary gate.**
        Every snow candidate is scored on `INV-SNOWFREEZE-050` over the five SNOTEL
        climates + the `cancov_forest` paired set; humid-NE depth becomes one
        reported surface, not the gate.
     2. **Adopt `harder_pomeroy_partition` (bundle + Harder-Pomeroy hourly phase)
        as the new default** — justified on the primary rubric (15 / 179 vs
        17 / 172). Activation under Policy B = cross-SNOTEL rubric improvement +
        workspace-suite no-regression + conservation (partition is mass-conserving).
        Caveat: it re-introduces a **`+23.6 kg/m^-3` density bias** (bundle was
        ≈0), so density recovery is a follow-on (decision 4).
     3. **Humid-NE depth regression is a roadmap item, not a blocker.** The
        partition worsened the humid-NE depth gate (10.3.5c, `1147 → 1273`), but
        that gate is non-representative. Likely handling: a **hillslope `.run`-file
        option to disable the partition** — a deliberate new *user-facing* control
        (distinct from the internal env rollback selectors), scoped contract-first
        as its own package.
     4. **Sublimation is not abandoned — diagnose the implementation.** The science
        says sublimation matters (especially dry/windy SNOTEL), so the 10.3.16
        `−19` robust delta is an **implementation-quality** problem, not a wrong
        mechanism. Diagnose it; **unlock Stage B** (the two-layer surface
        cold-content structure) if Stage A's single-layer form is the limit. Test
        the **partition + sublimation composition** — the `+23.6` / `−23.0` density
        biases are nearly equal-and-opposite, so together they may hold density ≈0
        while combining partition timing with sublimation mass loss.
     5. **New process/science amendments are admissible — SNOBAL is a reference,
        not a ceiling.** A candidate need not be a SNOBAL/legacy port; a novel
        mechanism is admissible if its physics are defensible **and** it improves
        cross-SNOTEL forcing-robust robustness against the observed-data rubric
        (ADR-0011 contract-first; ADR-0017 legacy/PySnobal as flags). This posture
        is captured domain-general as
        [ADR-0028](../decisions/0028-observed-data-admission-authority.md)
        (observed-data admission authority when scientific authority is lacking).
   - **Completed: 10.3.19 (partition adopted) and 10.3.20 (ablation class
     exhausted).**
     - **10.3.19** activated the Harder-Pomeroy hourly phase into the no-env
       default (bundle + partition). On the cross-SNOTEL forcing-robust rubric the
       default rose to **15 fail / 179**, now **above legacy (16 / 176)** —
       resolving the 10.3.18 "bundle below legacy" finding. The partition is the
       arc's decisive cross-climate lever. (Humid-NE depth regression remains the
       deferred roadmap item per decision 3; `legacy_rst` preserved as rollback.)
     - **10.3.20** (`NON-PROMOTION-GATE-NOT-MET`) closed the open-surface ablation
       thread as a triple negative against the 15/179 default: partition + Stage A
       sublimation `19 / 168` (1 better / 8 worse cells); Stage B two-layer surface
       layer `15 / 178` (1 better / 3 worse). Both fail primary + bidirectional
       guardrail; conservation closed; the CC0 `libsnobal` Stage B port executed
       cleanly (commit `bf8b41c…`, provenance captured). Diagnostic reads: it is
       **not merely a bad Stage A implementation** (the faithful two-layer port also
       fails); the density complementarity is refuted at the rubric level (8 worse
       cells); sublimation stays physically valid but is **not a rubric lever on
       this corpus** (likely wrong regime / signatures not sensitive / guardrail-
       blocked) — the ADR-0028 discrimination working as designed. Review:
       `docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/artifacts/claude-review-mechanism-family-exhausted.md`.
   - **The SNOBAL/CoE/Anderson mechanism family is now substantially exhausted.**
     Adopted: holding-capacity melt, bulk density compaction, Harder-Pomeroy
     partition. Rejected/neutral: melt-shortwave modernization, winter-thaw
     (superseded), spring densification, shallow-pack guard, sublimation, two-layer
     Stage B. The 15/179 default is a local optimum for the family; **further levers
     must be a new mechanism class admitted under ADR-0028** (canopy snow
     interception/sublimation, sub-canopy longwave, wind redistribution), not
     another variant. **Next = residual decomposition on the post-partition default
     + the frost-attribution-threshold decision** (step 9), not another family
     candidate. The under-persistence tail (density-arm mechanism cost, never
     recovered) is the prime suspect for the binding constraint.
   - **Default phase activation completed as 10.3.19 (`ACTIVATED`).**
     `docs/work-packages/20260628-snowdensity-10-3-19-harder-pomeroy-default-activation-001/`.
     `SC-SNOWFREEZE-001` v104 activates `harder_pomeroy_hourly` as the
     direct-production no-env phase default, composed with the activated
     melt+density bundle, and keeps explicit `legacy_rst` rollback/test
     selection. Real cross-SNOTEL direct-production rerun reconfirms the primary
     gate (`15` robust fails / `179` score vs prior bundle `17` / `172`), with
     selector trace proof and partition conservation closed (`5.55e-17 m` max
     trace residual). Release notes carry forward humid-New-England depth
     regression as a non-representative roadmap item and the `+23.6 kg/m^-3`
     density-bias rise as separate recovery work. No fixture/schema/cap/frost
     change or `.run` disable option is included in 10.3.19.
   - **Sublimation diagnosis / Stage B unlock completed as 10.3.20
     (`NON-PROMOTION-GATE-NOT-MET`).**
     `docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/`.
     `SC-SNOWFREEZE-001` v105 records libsnobal CC0 provenance
     (`bf8b41c71e3e54ae654ae04005ddf72566c47ee6`, `setup.py`
     `license="CC0 1.0"`), authorizes opt-in
     `coe_open_sublimation_stage_b_v1`, and keeps the cross-SNOTEL
     forcing-robust rubric as the promotion gate. Real WAT/trace result:
     current default remains `15` robust fails / `179` score; partition +
     Stage A sublimation is worse (`19` / `168`); Stage B cuts aggregate
     sublimation magnitude and conserves vapor/phase mass, but scores `15` /
     `178` and worsens three robust cells, so it remains opt-in diagnostic only.
     The density-bias offset hypothesis did not hold on the primary rubric, and
     no fixture/schema/cap/frost/default/user-surface or `.run` disable change
     is authorized.
   - **Post-partition residual decomposition completed as SNOWDENSITY-10.3.21
     (`DIAGNOSTIC-COMPLETE-NO-PROMOTION-NO-FROST-DECISION`).**
     `docs/work-packages/20260628-snowdensity-10-3-21-post-partition-residual-decomposition-001/`.
     The diagnostic consumes `INV-SNOWFREEZE-050` and ADR-0028 without new
     gate authority. Current default remains **15 / 179**. The residual is
     **signature-concentrated but site-diffuse**: densification trajectory
     contributes `9/15` robust fails across SNOTEL and `cancov_forest`,
     humid-New-England depth-SWE geometry contributes `2/15`, and mountain
     SNOTEL timing under-persistence contributes `4/15`. The under-persistence
     tail remains present post-partition, but it is not the sole binding
     constraint; density-structure dominates and no over-persistence timing tail
     remains. Frost-threshold input read:
     `MIXED-NO-SINGLE-GLOBAL-SNOW-LEVER`. Candidate mechanism signals for later
     ADR-0028 packages are canopy/sub-canopy snow processes in the cancov
     geometry cluster and wind redistribution or forcing/representativeness in
     the mountain timing cluster; no frost-attribution threshold decision,
     production/default/cap/schema/fixture/frost change, selector, or site
     calibration is made.
   - **Snow-density paradigm assessment completed
     (`PARADIGM-ASSESSED`).**
     `docs/work-packages/20260628-snow-density-paradigm-assessment-001/`.
     The post-10.3.21 decision assessment compares climate-class parameter
     specialization (Paradigm 1), multilayer snowpack physics (Paradigm 2), and
     accepting the current floor. It recommends Paradigm 1 as the next
     snow-density candidate package because it directly targets the diffuse,
     split-sign densification-trajectory residual while fitting the existing
     scalar bulk density lane as an opt-in candidate. The clean form is
     independently assigned Sturm 1995/2010 or NSIDC-0768 snow class driving
     Anderson/SNOBAL-style coefficient specialization, not fixture-fitted raw
     empirical regression. Paradigm 2 remains the escalation path if the
     class-aware candidate fails or frost/canopy evidence requires vertical snow
     structure. The current `15` / `179` snow floor remains usable for the
     parallel frost-attribution-threshold process with uncertainty carried
     forward. No production density code, contract, fixture, schema, default,
     density-cap, or frost change is made.
9. **Activation / Retirement Decision.** Decide whether to promote, hold, or retire
   any opt-in snow melt/density bundle. Closure gate: Activation Policy B
   workspace-suite no-regression plus composite snow-state conservation evidence
   for default activation (downstream snow-affected output deltas conserved-by-
   construction, documented not separately diffed — §10.3 step 6), explicit
   frost-attribution impact as a separate snow-control gate, and contract
   amendments for any production activation.

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
