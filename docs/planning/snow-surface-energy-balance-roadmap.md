# Snow Surface-Energy-Balance Campaign Roadmap

Status: **active scientific campaign roadmap** (2026-07-29).

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
| 8 | `SNOW-SURFACE-EB-05` bounded campaign closeout — **admitted / nonpromotion only** | Record the negative result and EB-04 evidence limitation in `ASSURE-06`, preserve default-off selectors, and close the campaign without another tuning round. | Do not promote, recalibrate, or reinterpret EB-04 as a completed validation. Reopen science only with new authoritative process science, new discriminating observations, or an independently testable formulation. Diagnostic defects may be investigated separately but do not authorize another EB-04 round. |

Orders 3 and 4 may complete their contract work in parallel after EB-01A, but
EB-02 runtime wiring depends on the EB-03 temperature/cold-content decision.
Neither is a result-bearing substitute for Order 5. Order 5 starts only when
the same runtime can express all four orthogonal cells.

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
- EB-04 reaches its declared bounded candidate/experiment budget without a
  promotable result.

After stop-loss, do not open another calibration round from the same evidence.
Adjudicate the limitation, update `ASSURE-06`, and reopen only with new
authoritative process science, new discriminating observations, or an
independently testable formulation.

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
- No default activation before the Order 5 promotion gate.
- No tuning of shared meteorological radiation to improve snow.
- No reuse of validation observations for fitting.
- No replacement of the current snow-density, phase, liquid-routing, frost, or
  canopy-phenology defaults outside a successor's explicit contract.
- No public output-schema change merely to support campaign diagnostics.
- No claim that comparator agreement establishes scientific correctness.

Canonical process authority remains in `SC-SNOWFREEZE-001` and any reviewed
successor amendment; this roadmap is sequencing and experimental-design
authority only.
