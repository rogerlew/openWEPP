# Snow Surface-Energy-Balance Campaign Roadmap

Status: **active scientific campaign roadmap** (2026-07-30).

Campaign ID: `SNOW-SURFACE-EB`.

## 1. Outcome

Determine whether explicit sub-canopy longwave exchange and physically coupled
snow sublimation improve openWEPP snow behavior independently and in
combination. The campaign must preserve separate mass and energy accounting,
retain contrary evidence, and stop without promotion if the prespecified
science gates are not met.

The campaign is not a general snow retuning exercise. It does not reopen
rejected shortwave/albedo, phase-partition, density-specialization, or prior
open-surface sublimation candidates merely because they exist. It asks a
narrower question: can two missing or incompletely composed surface processes
explain forest/open and climate-dependent snow behavior without compensating
errors?

The downstream product is a refreshed snow/frost flagship assurance manuscript
under `ASSURE-06`, whether the campaign promotes a candidate or records a
well-bounded nonpromotion.

## 2. Current Scientific Baseline

The starting point is not an empty surface-energy implementation:

- Production openWEPP retains the WEPP Corps-of-Engineers melt lineage.
- `openwepp-meteorology` provides typed, surface-agnostic shortwave, longwave,
  sensible, latent, conductive, advected-heat, and energy-sum primitives.
- The opt-in multilayer Stage 3 path consumes the shared surface-energy API,
  but its current runtime surface sum is shortwave-only; sensible heat, latent
  heat, and explicit longwave are zero in that sum.
- Existing Stage A and Stage B snow-sublimation candidates remove bounded SWE
  as vapor and preserve a vapor-mass ledger. They were not promoted: Stage B
  conserved mass but scored `15` robust failures / `178`, compared with the
  current default at `15` / `179`.
- Sub-canopy longwave remains excluded from the implemented candidates.
- Canopy-gradient and snow/frost fixtures provide a mixture of paired
  forest/open, winter-deciduous, mixed, and dense-conifer contexts, but their
  observation correspondence and verdict authority are not uniform.

`SNOW-SURFACE-EB-01` must reconcile these facts against the current tree before
any successor treats a planning statement as current implementation truth.

## 3. Scientific Questions

The campaign must answer:

1. What explicit longwave flux is absent from the current snow-surface runtime
   balance, and what authoritative formulation and canopy-temperature surface
   can supply it?
2. Does the existing sublimation implementation debit the same physical energy
   ledger that governs snow warming, melt, and refreeze, or is it currently only
   a mass-side diagnostic?
3. What are the independent effects of longwave and sublimation on surface
   energy, vapor loss, SWE, depth, melt/refreeze, runoff timing, snow
   persistence, and snow-to-frost insulation?
4. What is their combined effect, and is it consistent with simple additivity
   or dominated by an interaction?
5. Do the mechanisms improve the prespecified forest/open and climate-regime
   contrasts without worsening protected open, leaf-off, or cold-site
   signatures?
6. Is any apparent improvement a compensating error between added longwave
   energy and sublimation mass loss?

## 4. Binding Factorial Design

The result-bearing experiment must expose longwave and sublimation as
orthogonal, independently selectable process mechanisms. It must execute the
same frozen forcing, initial state, canopy projection, density/liquid-routing
selection, and observation operator in four cells:

| Cell | Sub-canopy longwave | Snow sublimation |
| --- | --- | --- |
| `B` baseline | off | off |
| `L` longwave only | on | off |
| `S` sublimation only | off | on |
| `LS` combined | on | on |

For every prespecified response `Y`, report:

- longwave marginal effect: `Y(L) - Y(B)`;
- sublimation marginal effect: `Y(S) - Y(B)`;
- combined effect: `Y(LS) - Y(B)`; and
- interaction/additivity residual:
  `Y(LS) - Y(L) - Y(S) + Y(B)`.

The interaction term must be reported even when it is small. The campaign may
not assume that the processes add linearly. It must also examine process order
and shared-state effects within the hourly solver. In particular:

- sublimated mass must be a vapor export, never routed melt or liquid;
- its latent-energy debit must appear exactly once in the energy ledger;
- longwave energy must affect the snow thermal/melt state through the shared
  surface balance, not through an independent temperature-index correction;
- the combined cell must not count sublimation once as a latent flux and again
  as an unrelated mass loss; and
- mass and energy closure require independent reconstruction from produced
  operands, not only producer self-consistency.

If the current selector architecture cannot express all four cells without
entangling unrelated melt behavior, a successor must first create typed,
orthogonal diagnostic selectors. It may not infer factorial effects from
non-comparable legacy profiles.

## 5. Evidence And Observation Roles

Before result-bearing execution, `SNOW-SURFACE-EB-01` must freeze:

- the exact current default and rollback candidates;
- the warm-maritime, cold-continental, dense-conifer, mixed, deciduous/leaf-off,
  and open-control lanes;
- modeled-to-observed canopy-stratum correspondence;
- forcing identity and uncertainty;
- response units, aggregation periods, and comparison operators; and
- each measured dataset's role as `CALIBRATION`, `INDEPENDENT_VALIDATION`, or
  `DIAGNOSTIC_ONLY`.

Within-site forest/open contrasts receive priority because shared meteorology
reduces forcing confounding. Absolute SWE and depth remain forcing-limited where
the existing snow strategy says they are. No site may supply both fitted values
and an independent-validation claim.

Default coefficients and bounds must come from admitted science authority.
Execution grids introduced only to test machinery are
`ASSUMED_FOR_EXECUTION`, not empirical priors, physiological bounds, or
calibrated ranges.

## 6. Ordered Work Packages

| Order | Package | Objective | Advancement gate |
| ---: | --- | --- | --- |
| 1 | [`SNOW-SURFACE-EB-01`](../work-packages/20260729-snow-surface-eb-01-reconciliation-factorial-design-001/package.md) — **complete / pass** | Reconciled canonical authority, current implementation, prior outcomes, observations, selectors, and ledgers; froze the factorial, data roles, exact-one latent/mass rule, and stop-loss. | EB-01A/02/03/03A/03B close the prerequisites; EB-04 is admitted for scaffolding. |
| 2 | [`SNOW-SURFACE-EB-01A`](../work-packages/20260730-snow-surface-eb-01a-longwave-authority-research-001/package.md) — **complete / pass** | Resolved equation-level atmospheric longwave, hemispherical sky/canopy view, effective-unity treatment, canopy-temperature, and trunk-scope authority before EB-02. | At EB-01A closure, deterministic sky-view and daylight cloud mapping remained EB-02 prerequisites. EB-02 now binds them without a new user coefficient or required remote dataset; thermal and polar-night runtime prerequisites remain with EB-03. |
| 3 | [`SNOW-SURFACE-EB-02`](../work-packages/20260730-snow-surface-eb-02-subcanopy-longwave-contract-001/package.md) sub-canopy longwave contract — **complete / contract pass / runtime held** | Binds the selected displaced-sky/additive-canopy formulation, corrected hourly Dilley-Unsworth estimator using daily vapor/cloud state, internally derived radiometric sky view from effective canopy cover, effective-unity canopy/snow treatment, and named air-temperature proxy limits. | Corrected dual review and dual terminal verification pass. `SC-SNOWENERGY-001`, 38 analytical vectors, figures/sidecars, typed guards, no direct cover/sky-view alias, no new user coefficient, and no required remote data are complete. EB-03 must bind the shared thermal/cloud provider before a real runtime consumer and energy-closure increment. |
| 4 | [`SNOW-SURFACE-EB-03`](../work-packages/20260730-snow-surface-eb-03-shared-thermal-energy-composition-001/package.md) shared thermal/cloud provider and energy-consistent sublimation composition — **executed / hold / model limitation** | Implemented the shared Stage 3 provider, longwave, exact-one vapor/latent exchange, independent selectors, diagnostics, and real consumer. | Same-binary absent/empty/disabled selector equivalence, focused closure, B, and L pass. The real S cell reaches the provider's `0 K` cold-content bound after 18 days with material SWE remaining; S/LS are not mechanically admissible. Stop-loss invoked. |
| 5 | [`SNOW-SURFACE-EB-03A`](../work-packages/20260730-snow-surface-eb-03a-active-layer-thermal-coupling-001/package.md) active-layer thermal coupling — **complete / pass** | Closed the science defect `GAP-SNOWENERGY-007` by replacing the snowfall-event top-layer heat capacity with the Marks/SNOBAL active thermal control volume, coupling it to the lower pack within each energy substep, and applying authority-defined shallow-pack timestep subdivision. | Real B/L/S/LS, rollback, conservation, snow-domain, quick, frost, and Critical full gates pass after EB-03B lifted the unrelated validation hold. |
| 6 | [`SNOW-SURFACE-EB-03B`](../work-packages/20260730-snow-surface-eb-03b-terminal-validation-hold-lift-001/package.md) terminal validation hold lift — **complete / pass** | Isolated optional-QA state from the synthetic CQR self-test and decomposed two assurance negative matrices without changing snow physics, assurance authority, timeout limits, or quality thresholds. | CQR 4/4, assurance publication 37/37, quick 2109/2109, frost 324/324, and Critical full 2158/2158 pass. |
| 7 | [`SNOW-SURFACE-EB-04`](../work-packages/20260730-snow-surface-eb-04-factorial-execution-adjudication-001/package.md) factorial execution and adjudication — **executed / hold / nonpromotion** | Executed the frozen 48-cell design once and published the complete inventory, partial/completed trajectories, and sidecars. | B completed 12/12 lanes, L 10/12, S 2/12, and LS 0/12. Failures comprise 22 masked Stage 3 effective-conductivity-path errors and two prior-layer thickness reconciliation errors. No LS observation score or interaction is admissible. Physical-gate and one-round stop-losses close promotion; retained output lacks shortwave and the signed latent/mass operands needed for the package's anti-tautological reconstruction gates, so package closure holds. |
| 8 | [`SNOW-SURFACE-EB-04A`](../work-packages/20260730-snow-surface-eb-04a-failure-observability-state-capture-001/package.md) failure observability and failing-state capture — **complete / pass** | Preserved exact typed causes and complete rejected thermal/prior-layer state; published shortwave, signed vapor mass, and latent conversion operands; targeted all 24 frozen failures. | All failures reproduced on the original day: 17 below-absolute-zero projections, five saturation-vapor-pressure underflows at extreme cold, and two approximately `1.0e-9 m` layer-depth mismatches. Independent mass, surface-energy, and latent/mass reconstruction pass without a physics change. |
| 9 | [`SNOW-SURFACE-EB-04B`](../work-packages/20260731-snow-surface-eb-04b-coupled-dynamics-characterization-001/package.md) coupled-dynamics characterization — **complete / pass** | Retained and classified the complete 83,232-row chronology for all 24 exact EB-04A failures without rerunning the factorial or changing physics. | Seventeen failures enter impossible below-absolute-zero states from positive cold content on vanishing mass; five remain valid Kelvin states but underflow in the SNOBAL saturation-vapor-pressure dependency. Sublimation is strongly associated and mechanistically contributory in 20/22 cases but is not necessary, and the unpublished crossing substep prevents a terminal-amplification claim. The two geometry failures exactly equal one SWE fragment filtered below `1e-9 m` while its physical depth remains above the independent depth tolerance. |
| 10 | [`SNOW-SURFACE-EB-04C`](../work-packages/20260731-snow-surface-eb-04c-thin-pack-thermal-domain-closure-001/package.md) thin-pack thermal-state and vapor-pressure-domain defect closure — **complete / pass** | Resolved the 17 impossible thin-pack states and five valid-Kelvin saturation-vapor-pressure underflows using libsnobal's exact `1 kg m^-2` branches while preserving CoE mass authority. | Total mass `<=1 kg m^-2` suspends before partition; a resolved pack's lower volume `<1 kg m^-2` collapses to a continuing one-volume solve; lower equality remains two-volume. All 22 captured failures pass their formerly rejected processing day, 23/23 focused tests and quick/frost/Critical full profiles pass, and no clamp, epsilon pressure, forced melt, cold-content deletion, or coefficient tuning was introduced. |
| 11 | [`SNOW-SURFACE-EB-04D`](../work-packages/20260731-snow-surface-eb-04d-layer-thickness-reconciliation-001/package.md) layer-thickness reconciliation defect closure — **complete / pass** | Corrected the dimensionally inconsistent fragment filter/closure boundary exposed by `harvard_open/S` and `marcell_open/LS`. | Represented layers now use the existing strict `1e-9 kg m^-2` boundary after named SWE-to-mass conversion; the independent `1e-9 m` residual guards remain unchanged. Both 16,437-day trajectories complete with independently reconstructed conservation, and focused/quick/frost/Critical full plus dual review/verification pass. |
| 12 | `SNOW-SURFACE-EB-04E` corrected-population runtime qualification — **next** | Execute the corrected candidate across every formerly failing lane/cell with the complete 04A trace and independently reconstructed ledgers. Characterize remaining runtime and process-boundary behavior before observation scoring. | All 48 prescribed cells must be attempted; all S/LS cells must complete their physical, trace-identity, mass, energy, latent/mass, thermal, and layer-reconciliation gates. No empirical score, interaction, calibration, or promotion claim is admissible in this package. |
| 13 | `SNOW-SURFACE-EB-04R` fresh factorial execution and adjudication — **conditional on 04E pass** | Freeze a new exact executable and prospective protocol, then rerun the B/L/S/LS population as a second scientific experiment. Compare the corrected round with the frozen EB-04 failed baseline while retaining the original observation roles and protected signatures unless prospectively amended. | Run the new 48-cell matrix once. Score marginal, combined, interaction, protected-group, and compensation-error criteria only after every physical and anti-tautological gate passes. Promotion remains default-off and requires the complete preregistered rule. |
| 14 | `SNOW-SURFACE-EB-05` campaign assurance closeout — **deferred pending 04R or proven limitation** | Refresh `ASSURE-06` with the complete iteration history, corrected result or defensible model limitation, uncertainty, and default disposition. | Close after 04R adjudication, or after a predecessor proves that further correction requires missing/contradictory authority or an independently unavailable process formulation. A failed EB-04 first round alone is not sufficient campaign-retirement evidence. |

EB-04B assigned all captured failures and correction authority. EB-04C closes
the 22 thermal failures, and EB-04D closes both isolated geometry failures.
EB-04E is next to prove population-wide runtime and anti-tautological
admissibility; EB-04R waits for that qualification.

### 6.1 Iteration Discipline After EB-04

EB-04 remains immutable evidence of the first preregistered experiment. Its
one-round stop-loss forbids editing that package's result rule, tuning against
its observations, or replacing its failed cells with post-result reruns. It
does not retire sub-canopy longwave, sublimation, or their coupled dynamics.

Successor iteration is defect- and information-driven:

1. expose the exact rejected state and independently reconstruct the ledgers;
2. characterize chronology and coupling before selecting a correction;
3. separate conductivity and layer-geometry correction authority;
4. qualify corrected runtime behavior without empirical scoring; then
5. run one new prospectively frozen factorial.

Targeted diagnostic executions in 04A–04D are not factorial retries and may not
produce promotion evidence. Each defect-closure package must diagnose and
correct within its declared authority envelope rather than stopping at the
first reproducible error. If 04B exposes a missing science authority, insert a
bounded research package before the affected correction; do not install proxy
physics.

The sequence may loop only through a newly authorized defect-shaped package
with a named finding, write set, authority, and acceptance test. It may not loop
through coefficient search, guard relaxation, observation rescaling, or
result-aware metric changes.

## 7. Stop-Loss

Stop without promotion when any of the following is true:

- authoritative sub-canopy longwave or sublimation composition physics cannot
  be admitted without a surrogate canopy-temperature, emissivity, aerodynamic,
  or exchange formulation;
- the four cells cannot be made comparable without changing other snow
  processes;
- mass or energy fails independent closure;
- apparent improvement depends on site-specific tuning, observation leakage,
  forcing rescaling, or a post-result change to metrics or lanes;
- the combined cell improves a headline score while protected process
  signatures reveal compensating longwave/sublimation errors;
- the candidate does not improve its prespecified target contrasts, or causes
  material protected-site regression; or
- a result-bearing factorial reaches its declared candidate/experiment budget
  without a promotable result.

After stop-loss, do not tune, promote, or repeat the same result-bearing
experiment from the same executable and evidence. Preserve the failed
experiment, enter the diagnostic/correction sequence above, and admit a new
factorial only after its physical and anti-tautological prerequisites pass.
Close as a model limitation only when a predecessor proves a named
authority/formulation boundary after the in-envelope diagnostic and correction
routes are exhausted.

## 8. Required Human-Interpretation Artifacts

Result-bearing packages must provide plots without embedded prose blocks and a
Markdown sidecar for every figure. The candidate set must include:

- representative surface-energy component time series;
- cumulative sublimation, melt, refreeze, and SWE trajectories;
- paired forest/open trajectories under the same forcing;
- marginal and combined effect plots for `B/L/S/LS`;
- interaction/additivity-residual plots; and
- protected-signature and contrary-evidence views.

Each sidecar must explain the question, population, units, processing,
uncertainty, exclusions, interpretation, and limitation in language suitable
for a reader who finds the figure outside the package narrative.

## 9. Protected Boundaries

- No surrogate, provisional, proxy, or heuristic process physics in production.
- No default activation before the EB-04R promotion gate.
- No tuning of shared meteorological radiation to improve snow.
- No reuse of validation observations for fitting.
- No replacement of the current snow-density, phase, liquid-routing, frost, or
  canopy-phenology defaults outside a successor's explicit contract.
- No public output-schema change merely to support campaign diagnostics.
- No claim that comparator agreement establishes scientific correctness.

Canonical process authority remains in `SC-SNOWFREEZE-001` and any reviewed
successor amendment; this roadmap is sequencing and experimental-design
authority only.
