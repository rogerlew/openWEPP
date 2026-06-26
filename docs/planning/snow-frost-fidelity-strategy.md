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
(§5), revised work-package sequence (§7), and references (§10).

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
| **SNOWDENSITY-06 Density Compaction** | Entry gate first: drive diagnostic/adjudication `cancov` from the real per-day growth state, demonstrate configured coniferous forest winter `cancov` near `0.9`, consume native openWEPP shortwave or prove the PySnobal-bridge radiation inversion like-for-like, and rerun 05E-style adjudication in that representative regime. Then add the Anderson-1976/SNOBAL overburden + metamorphism compaction to the `physics_bulk` density path, constants carried from cited authority, on top of (not instead of) the modernized melt. | Harness-fidelity gate passes before density verdicts; conserves SWE; beats legacy on forcing-robust density/densification signatures without site tuning; no melt change substituting for density. |
| **SNOWDENSITY-07 Runtime Opt-In** | Couple the accepted melt + density model into the winter column behind `snow_model = physics_bulk`; preserve `legacy_wepp` default. | Conservation, publication, anti-alias, direct/compatibility rollback, and full workspace gates. |
| **SNOWDENSITY-08 Snow/Frost Gate Rerun** | Rerun non-SNOTEL snow-control and SNOTEL rubric baselines. | `TOL-SNOWFREEZE-009` passable or bounded; frost attribution can resume. |
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

## 10. References / Authority

DOIs below are from the 2026-06-25 literature sweeps (two independent research
passes, load-bearing items cross-verified). Verify any DOI on retrieval; a few
forest-canopy DOIs were agent-inferred and are flagged.

### 10.1 Internal authority

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

### 10.2 Literature in this repository

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

### 10.3 To track down (still not obtained)

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
