# Snow / Frost Fidelity Strategy

Status: **active strategy** (2026-06-25). Primary gap:
`GAP-SNOWFREEZE-002`.

Evidence mode: **Static summary of run evidence** from
SNOWFROST-FIDELITY-D/E/F/H/I0. This file is planning guidance, not canonical
science authority. Canonical authority remains `SC-SNOWFREEZE-001`, the
array-native runtime specification, and ADR-0011 / ADR-0017 / ADR-0024 /
ADR-0026 until amended.

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
- **No site tuning:** SNOTEL validates. It does not supply fitted constants.
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
- Keep the model selected through an explicit runtime option:
  `snow_model = legacy_wepp | physics_bulk`.
- Add a narrow snow-kernel boundary:

```text
forcing + prior snow state + model constants
    -> snow state after timestep
    -> liquid release / retained water / insulation operands
```

- Drive the first implementation through offline Rust `snowbench` using the
  existing WEPP/openWEPP/PySnobal forcing bridge and SNOTEL observation corpus.
- Promote to production only after the offline solver passes contract-derived
  tests, conservation tests, and SNOTEL rubric comparison.
- Preserve direct/compatibility rollback. The new model can deliberately diverge
  from legacy only under the opt-in mode and only after the governance ADR lands.

## 7. Work-Package Sequence

| Stage | Goal | Gate |
|---|---|---|
| **SNOWDENSITY-01 Evidence Reconciliation** | Complete in `docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001/`: consolidated E/F/H/I0 evidence; pinned openWEPP-vs-legacy density deltas; classified rubric cells; read Shen 2011 and the WEPP Eq. 3.7.5 divergence. | Evidence package only; no production code. |
| **SNOWDENSITY-02 Contract + ADR** | Complete in `docs/work-packages/20260625-snowdensity-02-contract-adr-001/`: amended `SC-SNOWFREEZE-001` v75 with `INV-SNOWFREEZE-051`, candidate `physics_bulk` state/process envelope, no-site-tuning rule, conservation obligations, and opt-in activation constraints; added ADR-0027. | Contract guard tests only; no production code. |
| **SNOWDENSITY-03 Offline Physics Core** | Complete in `docs/work-packages/20260625-snowdensity-03-offline-physics-core-001/`: implemented `physics_bulk` in Rust snowbench with candidate constants, bulk thermal state, fresh-snow density, densification, liquid retention/release, focused tests, production-confinement guard, and five-site SNOTEL rubric output. | Candidate remains offline only; first profile is finite but not production-promotable (`forcing_robust fail=24`, `marginal=13`, `pass=3`, `strong=5`, `unavailable=15`). |
| **SNOWDENSITY-04 Offline Adjudication Loop** | Complete in `docs/work-packages/20260625-snowdensity-04-offline-adjudication-loop-001/`: added global named candidate variants and adjudicated four variants against H openWEPP/legacy/PySnobal profiles. `dense_slow_melt_v1` is the promotion candidate. | No per-site constants; robust fail count improves from `9` to `6` and robust ordinal score from `84` to `102` against openWEPP and legacy as-built. |
| **SNOWDENSITY-05 Runtime Opt-In** | Couple the accepted candidate into the winter column behind `snow_model = physics_bulk`; preserve `legacy_wepp` default. | Conservation, publication, anti-alias, direct/compatibility rollback, and full workspace gates. |
| **SNOWDENSITY-06 Snow/Frost Gate Rerun** | Rerun non-SNOTEL snow-control and SNOTEL rubric baselines. | `TOL-SNOWFREEZE-009` passable or bounded; frost attribution can resume. |
| **FROST-RESUME** | Return to heat-flow, frozen-K/SFCC, or migration/fringe adjudication only after snow insulation is controlled. | Mechanism-specific work package. |

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

- `SC-SNOWFREEZE-001` (`INV-SNOWFREEZE-047/048/049/050`,
  `TOL-SNOWFREEZE-007..011`).
- ADR-0011, ADR-0017, ADR-0024, ADR-0026.
- SNOWFROST-FIDELITY-D/E/F/H/I0 work-package evidence.
- WEPP CRM Ch. 3.7 snow depth/density equations.
- `docs/backlog/20260605-snow-code-deferred-science-review.md`.
- Anderson, E. A. (1976), *A point energy and mass balance model of a snow
  cover*, NOAA Technical Report NWS-19.
- Marks et al. (1999) SNOBAL; Oleson et al. (2013) CLM; Lute et al. (2022)
  SnowClim shallow-snow stability discussion.
- Shen, D. (2011/2012) WSU MS thesis on WEPP snow distribution.
