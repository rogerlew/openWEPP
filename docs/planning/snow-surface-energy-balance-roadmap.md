# Snow Surface-Energy-Balance Campaign Roadmap

Status: **active scientific campaign roadmap** (2026-08-04).

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

### Snowbird Development Forcing Policy

Beginning with the next snow work package, result-bearing Snowbird development
lanes use an explicit precipitation-scaled derivative of the canonical CLI:

- multiply precipitation only by `1.2155576`; preserve temperature, timing,
  wind, radiation inputs, and all non-precipitation fields;
- preserve the canonical `p8.cli` byte-for-byte and retain it as the historical
  and provenance control;
- label the derivative `DEVELOPMENT_ONLY`, with no production-default,
  transferability, independent-validation, or forcing-truth claim;
- bind the transformation, rounding, source and derived hashes, and exact
  consumer path before a result-bearing run; and
- do not use the scaling to prove a physics correction or reinterpret earlier
  packages retrospectively.

The factor is the median annual precipitation multiplier required for the
current no-lateral-input fixture ceiling to reach the point-pillow peak over 35
complete Snowbird years. It is a development normalization for the live
input-feasibility signal, not a calibrated site coefficient. The first
successor package must materialize and validate the derivative before using it.

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
| 12 | [`SNOW-SURFACE-EB-04E`](../work-packages/20260731-snow-surface-eb-04e-corrected-population-runtime-qualification-001/package.md) corrected-population runtime qualification — **complete / pass** | Qualified the corrected candidate across the exact 12-lane/48-cell population using independently reconstructed WAT/trace, mass, energy, latent/mass, thermal, and layer ledgers without observation scoring. | All 48 cells and all 24 former failures complete across 761,212 rows; longwave and sublimation each reach all 24 enabled cells; all physical, identity, focused, Critical full, dual-review, and dual-verification gates pass. EB-04R is admitted after promotion-grade selector/environment provenance is frozen. |
| 13 | [`SNOW-SURFACE-EB-04R`](../work-packages/20260801-snow-surface-eb-04r-fresh-factorial-execution-adjudication-001/package.md) fresh factorial execution and adjudication — **executed / hold** | Ran the newly frozen B/L/S/LS population once with sanitized environment and promotion-grade provenance. | Runtime completes 48/48, but the consumer applies `1e-6 kg m^-2` to vapor-to-sublimation closure while the frozen protocol states `1e-9 kg m^-2`; 12 cells exceed the frozen bound. Observation scores and the generated nonpromotion result are inadmissible. Terminal disposition is `HOLD_PHYSICAL_OR_PROVENANCE_GATE`; no EB-04R rerun is authorized. |
| 14 | [`SNOW-SURFACE-EB-04S`](../work-packages/20260801-snow-surface-eb-04s-authority-reconciliation-retained-adjudication-001/package.md) result-blind authority reconciliation and retained-output adjudication — **executed / close nonpromotion** | Froze dimensional authority before reading results, amended `SC-SNOWENERGY-001` version 6, and re-adjudicated the immutable EB-04R outputs without launching the model. | `1e-9 m` SWE is `1e-6 kg m^-2` for the same transfer residual; all 48 cells and 288 retained file identities pass. LS changes robust score/failures `177/16 -> 180/16`, so the unchanged rule invokes the stop-loss and authorizes no further calibration/factorial round. EB-04R remains an unchanged HOLD. |
| 15 | [`SNOW-SURFACE-EB-04T`](../work-packages/20260801-snow-surface-eb-04t-unchanged-failure-attribution-001/package.md) unchanged-failure attribution and promotion-criterion fitness — **executed / diagnostic complete** | Reconstructed all 16 baseline failures across B/L/S/LS and mapped primary metrics, process ownership, control role, exact error direction, and interactions without rerunning the model or changing thresholds. | Eleven failures measure density/geometry or mixed interception debt. The five timing failures are open controls: they test sublimation and block combined-LS promotion but cannot identify canopy-longwave efficacy. LS moves 15 exact primary errors away and leaves one unchanged; 13/16 interactions are nonzero. EB-04S nonpromotion remains valid without a materiality or retroactive-promotion claim. |
| 16 | [`SNOW-SURFACE-EB-04U`](../work-packages/20260801-snow-surface-eb-04u-mechanistic-failure-partition-001/package.md) mechanistic failure partition and prospective-study design — **executed / diagnostic design complete** | Froze the exact `9/2/5` cohorts, observed-anchored primary operators, evidence roles, trace/ledger requirements, materiality/noninferiority/interaction rules, candidate-identity seal, and successor prerequisites before new candidate results. | All existing observations are diagnostic-only. Forty retained B/L/S/LS cells expose substantial state/mass/energy evidence but lack seven mechanism-critical operands, including authoritative CoE melt drivers and separate rain/sensible heat. EB-04V/04W/04X are admitted only for bounded operator/observability diagnostics; numeric replication/site-spread rules and independent data must be sealed before result-bearing efficacy work. |
| 17 | [`SNOW-SURFACE-EB-04V`](../work-packages/20260801-snow-surface-eb-04v-density-structure-mechanics-001/package.md) density-structure mechanics investigation — **executed / diagnostic complete / efficacy hold** | Added the contract-governed, behavior-neutral daily density-process ledger through the real direct-production JSONL consumer and executed all nine EB-04U density lanes across B/L/S/LS. | All 36 cells pass with maximum independent closure `3.411e-13 kg m^-3`; all WAT and pre-existing trace values remain identical to EB-04R. Wet compaction is the largest positive compaction contribution in both retained bias groups, so inactive compaction is rejected, but mixing, projection, and caps oppose it and no coefficient is uniquely identifiable. Retained B-cell counts/KGE reproduce the frozen operator. Existing observations remain diagnostic-only; no fitting, efficacy verdict, physics amendment, or promotion occurred. |
| 18 | [`SNOW-SURFACE-EB-04W`](../work-packages/20260801-snow-surface-eb-04w-accumulation-under-persistence-001/package.md) accumulation and mountain under-persistence investigation — **executed / diagnostic complete / calibration hold** | Added behavior-neutral hourly phase, accumulation, and exact CoE empirical melt-component diagnostics through the real JSONL consumer, then executed four unique open mountain lanes across 16 B/L/S/LS cells and five frozen operators. | All ledgers close within `1e-12 m`; baseline seasonal modeled peaks are about `0.39-0.62` of observed peaks, while observed-date retained-SWE lane medians are about `0.21-0.46`. Ownership remains mixed among realized input, endogenous liquid retention, external input causes, and phase-conditioned pre-peak modeled loss. No ablation amendment, fitting, efficacy claim, or promotion is admitted. |
| 19 | [`SNOW-SURFACE-EB-04W1`](../work-packages/20260802-snow-surface-eb-04w1-precipitation-scaling-calibration-001/package.md) precipitation-scaling calibration — **complete / calibration lever confirmed** | Execute the four open mountain lanes under the frozen `0.8-1.5` total-precipitation grid, using the observations as calibration data and preserving an exact `1.0` replay. | All 32 runs, transformations, baseline replays, closures, dual reviews, and dual verifications pass. Paradise/Snowbird retain joint upper-boundary response; Niwot magnitude continues upward; Mica magnitude is best near `1.4` while chronology improves at `1.5`. Final multipliers remain unidentified; no promotion/validation claim is available. |
| 20 | [`SNOW-SURFACE-EB-04W2`](../work-packages/20260802-snow-surface-eb-04w2-precipitation-scaling-grid-extension-001/package.md) bounded precipitation-scaling grid extension — **complete / site-specific calibration** | Retained all 24 EB-04W1 anchors and executed 20 new `1.6-2.0` cells under the frozen magnitude-first, chronology-protected rule. | Mica `1.4`, Niwot `1.7`, and Paradise `1.8` are interior site-specific calibration candidates. Snowbird reaches peak ratio `0.977` at the final `2.0` boundary but remains 23 days early. All 44 cells close within `4.441e-15 m`; dual review and verification pass, and the forcing branch stops without W3 or any validation/promotion claim. |
| 20A | [`SNOW-SURFACE-EB-04W2A`](../work-packages/20260802-snow-surface-eb-04w2a-residual-melt-chronology-attribution-001/package.md) residual melt-chronology attribution — **executed / hold / partial admission** | Retain the four selected W2 forcing cells, reconstruct phase-conditioned production ledgers, and compare existing legacy/albedo melt paths within the diagnostic snowbench harness. | Direct ledgers close. Cold-content threshold hits remain interannual; empirical `b + c` exceeds `a + d` at three sites; corrected early-gap late-input hits are Niwot `0/27` and Snowbird `5/16`. The albedo contrast is withdrawn because the public entry predicate loses one warm-mean, zero-pack snowfall event per lane (`0.0377-0.0708 m` SWE). |
| 20B | [`SNOW-SURFACE-EB-04W2B`](../work-packages/20260802-snow-surface-eb-04w2b-warm-day-new-snow-activation-closure-001/package.md) warm-day new-snow activation and consumer closure — **complete / technical, review, and verification pass** | The in-envelope correction, W2C prerequisite, exact-source isolated terminal-v2 rerun, dual re-review, and dual verification pass. | All eight terminal-v2 cells close within `2.220e-15 m`; the frozen albedo response remains immaterial and receives no promotion. Correct winter hydrology retains `4/231` explicit EROD16 refusals. |
| 20C | [`SNOW-SURFACE-EB-04W2C`](../work-packages/20260802-snow-surface-eb-04w2c-corrected-winter-erosion-continuity-001/package.md) corrected-winter-forcing downstream continuity diagnosis — **complete / technical, review, and verification pass** | Partitioned the corrected EROD16 population and amended the Wave-1 diagnostic under `SC-SED-001` revisions 56–60 without changing snow physics, the erosion solution, the `5e-3` diagnostic tolerance, or exact mass closure. Fresh revision-60 review and renewed broad gates pass; revisions 58–60 close mandatory profile/history/diff evidence after terminal-verifier/reviewer HOLD. | Matched-order same-zone Simpson blocks retain `4/231` refusals after correction; seven focused tests, EROD16, owning-crate, quick, frost, erosion, Critical full, clippy, format, doctest, assurance, dual review, and dual terminal verification pass. W2B is admitted to resume; EB-04X remains held behind it. |
| 21 | [`SNOW-SURFACE-EB-04X`](../work-packages/20260803-snow-surface-eb-04x-harvard-depth-swe-geometry-interception-001/package.md) Harvard depth–SWE geometry/interception investigation — **complete / technical, review, and verification pass / no promotion** | Terminal-v2 consumes profile/trajectory evidence, publishes daily B/LS phase-input identity and true same-day forest/open extrema, and enforces guarded contiguous pairing under a pre-result quantitative screen. Fresh dual re-review and verification pass. | Model geometry closes, but `746/746` complete HF237 rows fail supplied SWE identity; S/LS durations are incomplete and canopy load/interception/unloading/drip operands are absent. All component screens remain `NOT_EVALUABLE`; no promotion. |
| 21A | [`SNOW-HOURLY-ERA5-DIAGNOSTIC`](../work-packages/20260803-snow-hourly-era5-diagnostic-001/package.md) historical hourly forcing attribution — **complete / DIVERGES / no promotion** | Eight product/site comparisons and four deterministic figures bind validated sources and preserve precipitation. Wet-winter horizontal differences range from about -6% at Paradise to +28.5% at Snowbird. | Closed at the verified radiation-first boundary; later lanes are separate packages. |
| 21B | [`SNOW-HOURLY-ERA5-CLOUD-PROXY-SANITY`](../work-packages/20260803-snow-hourly-era5-cloud-proxy-sanity-001/package.md) SIMIMPL cloud-proxy sanity check — **complete / sanity association passes / no promotion** | Primary 24-hour ERA5 cloud residual has the expected inverse association with shortwave residual at all four sites, but wet-winter daily proxy correlations are only 0.121–0.321. | Closed after dual review and verification; no cloud correction, provider admission, or snow-model improvement is claimed. |
| 21C | [`SNOWBIRD-SNOTEL-CLIMATE-FORCING-DIAGNOSTIC`](../work-packages/20260803-snowbird-snotel-climate-forcing-diagnostic-001/package.md) colocated precipitation/temperature comparison — **complete / bounded dry-forcing evidence / no correction** | Guarded common-day fixture precipitation is about 84% of SNOTEL with wet-winter daily `r=0.913`; wet-winter Tmax is `+0.62 C` and Tmin `-0.38 C` relative to SNOTEL. | Dual review/verification pass. Missingness, sensor-bias, gauge/scale, and causation limits prevent a forcing correction or full attribution. |
| 21D | [`SNOWBIRD-SNOTEL-CLIGEN-SWE-RESPONSE`](../work-packages/20260803-snowbird-snotel-cligen-swe-response-001/package.md) controlled observed-mode forcing substitution — **complete / mediated response improves but remains insufficient / no correction** | Relative to CLIGEN control, SNOTEL P+T improves median peak ratio `0.313 -> 0.360` and melt-out `-27 -> -22 d`; the CLIGEN control itself is materially worse than the original fixture. | Dual review/verification pass. P/T input bundles are CLIGEN-mediated; no forcing correction, transferability, or promotion. |
| 21E | [`SNOWBIRD-RST-PREPEAK-FLUX-DIAGNOSTIC`](../work-packages/20260803-snowbird-rst-prepeak-flux-diagnostic-001/package.md) phase threshold and accumulation-season loss attribution — **complete / phase insufficient / no correction** | Active Harder-Pomeroy is exactly `rst`-invariant. Legacy `0 -> 4 C` raises Snowbird peak ratio only `0.308 -> 0.385`; pre-peak liquid release remains about `0.5 m`. | Dual review/verification pass. Values above `1 deg C` are best-case stress only; accumulation-season melt/liquid evacuation is the dominant next hypothesis. |
| 21F | [`SNOW-PREPEAK-LIQUID-EVACUATION-PHYSICS-AUDIT`](../work-packages/20260803-snow-prepeak-liquid-evacuation-physics-audit-001/package.md) first-principles model and implementation audit — **executed / reviewed / verified / hold-evidence** | Same-binary v3 finds Snowbird median pre-peak loss `0.5296 m` against `0.5379 m` gross-positive CoE melt. Stage 3 and explicit longwave are mass-neutral; legacy routing gives an order-one rollback bound. | Stage-3 liquid operands are omitted from JSONL, mixed export/refreeze is reachable, and wet-compaction operand authority is incomplete. Add behavior-neutral trace completeness and re-adjudicate signed-hour/export physics before correction. |
| 21G | [`SNOW-STAGE3-LIQUID-SIGNED-HOUR-TRACE-CLOSURE`](../work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/package.md) behavior-neutral diagnostic publication — **complete / reviewed / verified** | Additive schema v4 exposes exact Stage-3 liquid closure plus existing signed-hour forcing/state/cold-content operands through the real JSONL consumer. | All `14,245` Snowbird rows close within `1.23e-17 m`; four aliases separate, WAT/HBP/PASS and pre-v4 fields remain identical, and Stage 3 routes most incoming liquid. Observability is complete; physics adjudication remains separate. |
| 21H | [`SNOW-MASS-TRANSITION-LEDGER-PERSISTENCE`](../work-packages/20260803-snow-mass-transition-ledger-persistence-001/package.md) durable linked mass-accounting architecture — **complete / review and verification pass** | Preserve the solid-to-liquid and liquid-disposition boundaries as two exact linked compact ledgers over one authoritative calculation, while separating production outcomes from the opt-in hourly research payload. | Exact schema-v4/WAT/HBP identity, compact-ledger closure, payload-footprint reduction, runtime/RSS bounds, quick/frost/Critical full, dual review, and dual verification pass. Physics adjudication remains separate. |
| 21I | [`SNOW-PREPEAK-MASS-TRANSITION-PHYSICS-ADJUDICATION`](../work-packages/20260804-snow-prepeak-mass-transition-physics-adjudication-001/package.md) exact-current cross-fixture correction-family adjudication — **complete / upstream-generation priority / review and verification pass** | Four fixtures localize modeled pre-peak pack loss to the authoritative upstream CoE path. Daily signed opportunity is material at only Mica/Niwot; post-CoE cold opportunity is `0.455-1.37%` of peak deficit; current Stage-3 disposition controls neither SWE nor runoff publication. | Exact real CLI/compatibility/closure, six figures, primitive anti-alias reconstruction, dual review, and dual terminal verification pass. No equation, calibration, forcing exoneration, or correction is authorized. |
| 21J | [`SNOW-ACCUMULATION-TARGET-FEASIBILITY-AND-INPUT-LOSS-DISCRIMINATION`](../work-packages/20260804-snow-accumulation-target-feasibility-input-loss-discrimination-001/package.md) target/input/loss discrimination — **complete / loss-priority signal / coverage-fragile / no correction** | Across 154 primary windows, only Snowbird fails the all-phase zero-loss ceiling; Niwot, Paradise, and Snowbird pass guarded dry-loss screens. Cold-event input remains non-systemic but is padding-sensitive. | Prioritize scale-aware warm/mixed pre-peak loss attribution while retaining Snowbird as multifactor. Point-versus-hillslope, coverage, resolution, and PRCPSA independence limits prohibit a correction. |
| 21K | [`SNOW-WET-COMPACTION-OPERAND-AUTHORITY-AND-DUPLICATE-ALIAS-CLOSURE`](../work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/package.md) — **complete / defect corrected / review and verification pass** | Canonical v125 authority now requires positive generated melt plus interval-start snow-contact rain exactly once. Production and offline replay consume exact lineage; the accumulated driver falls to `55.7-61.9%` of the retired alias while upstream mass and density/layer closure pass. | Quick `2181/2181`, frost `358/358`, Critical full `2270/2270`, dual review, and dual verification pass. The Snowbird scaled lane is verified development-only evidence; early-melt attribution remains unproven. |
| 21L | [`SNOW-WARM-MIXED-PREPEAK-LOSS-ENERGY-ATTRIBUTION`](../work-packages/20260804-snow-warm-mixed-prepeak-loss-energy-attribution-001/package.md) — **complete / multifactor warm-mixed and state signal / review and verification pass** | All four canonical sites place `99.61-99.91%` median pre-peak loss on warm/mixed days; `cmelt` is the largest annual-first positive empirical term. Material-loss days are warmer/moister, generally more radiative, denser, and shallower; wind is not systemic. | Corrected dry loss matches 21J within `9.02e-17 m`; scaled Snowbird adds `0.1258 m` peak SWE and `0.0190 m` loss. Evidence is chronology-confounded and authorizes no correction. |
| 21M | [`SNOW-WARM-MIXED-COE-PHYSICS-AUTHORITY-AUDIT`](../work-packages/20260804-snow-warm-mixed-coe-physics-authority-audit-001/package.md) — **complete / baseline fidelity with authority gap / review and verification pass** | Exact reconstruction over `394705` warm/mixed hours excludes a Rust term-generation defect. The 2008 canopy branch is the sole net-positive `C` subcomponent in signed site sums; material post-handbook changes lack cited independent validation or bounded transferability authority. | No correction is authorized. Route the intentional CoE/Stage-3 ownership seam to contract-first 21N. |
| 21N | [`SNOW-COE-STAGE3-MELT-OWNER-AUTHORITY-RECONCILIATION`](../work-packages/20260804-snow-coe-stage3-melt-owner-authority-reconciliation-001/package.md) — **executed / Stage 3 authority admitted / assurance refreshed** | Stage 3 is the admitted future sole melt owner; current CoE remains byte-identical compatibility runtime on implementation hold. Dual science review and focused authority gates pass. | ASSURE-06 has replaced the stale locked flagship with an exact pending-review root and passing campaign gates. Later runtime implementation and cutover remain separately unauthorized. |
| 22 | [`ASSURE-06` snow/frost flagship authority refresh](../work-packages/20260805-assure06-snow-frost-flagship-refresh-001/package.md) / `SNOW-SURFACE-EB-05` campaign assurance closeout — **DRAFT / review authority returned** | Refreshed the flagship against 21K-21N and v7/v126 authority and reproduced all 188 retained values. The later typed lifecycle event returned it to governed `DRAFT`; contract-v8 adoption now binds generation `910ab3d3`. | Public count, active review events, and approval roots are zero. Target authority remains distinct from implemented physics, efficacy, noninferiority, default change, and cutover. A separately authorized exact-subject human review entry is required. |
| 23 | [`SNOW-STAGE3-COMPLETE-CARRIER-SHADOW-MELT`](../work-packages/20260805-snow-stage3-complete-carrier-shadow-melt-001/package.md) — **executed / carrier plausibility FAIL / structural and authority HOLD** | Contract v8 binds the CLIGEN virtual-instrument geometry. A default-off, non-mutating complete carrier and within-day cold-content/fusion shadow ran through the real Snowbird consumer. After review corrections, the frozen screen remains strongly positive at `+170.2536 MJ m^-2`; shadow melt is `0.5889 m` versus CoE `0.4101 m`. | Chronology is not evaluable because state reinitializes daily. Per-term shadow lineage, cold-content export, persistence, terminal meltout, and receiving-surface authority remain open. CoE stays authoritative; the `3,177`-line solver requires extraction before feature work. |

EB-04B assigned all captured failures and correction authority. EB-04C closes
the 22 thermal failures, and EB-04D closes both isolated geometry failures.
EB-04E proves population-wide runtime and anti-tautological admissibility.
EB-04R completed the runtime population but remains held under its own frozen
protocol. EB-04S independently resolved the cross-unit authority before reading
results, admitted the immutable retained population, and closed nonpromotion
because failures did not decrease. EB-04T shows the requirement was mixed but
not merely technical: 11 failures emphasize density/geometry or mixed
interception debt, while five open-control timing failures test sublimation and
block combined-LS promotion without identifying canopy-longwave efficacy. It
does not authorize another calibration/factorial round or change EB-04S.
EB-04U freezes the prospective mechanistic partition and admits EB-04V–04X for
bounded diagnostics with prerequisites. EB-04V closes density-process
observability without an efficacy result. EB-04W closes the open-mountain
chronology diagnostic with a closed input/storage/loss ledger but cannot
identify unique causal ownership. EB-04W1 confirms the ordinary legacy-WEPP
precipitation-scaling calibration lever. Paradise and Snowbird retain joint
upper-boundary response, Niwot magnitude continues upward, and Mica exposes a
boundary chronology versus already-turning magnitude tradeoff. EB-04W2A then
admitted direct attribution but exposed a warm-day, zero-pack snowfall
activation defect that invalidates its albedo harness. EB-04W2B closes that
consumer defect in its snow-scoped paths, but corrected winter hydrology then
exposed a lower-order downstream EROD16 discretization instrument. EB-04W2C
separates exact mass closure from that instrument and implements a
matched-order diagnostic. Fresh revision-57 reviews and renewed terminal gates
pass; terminal verifiers accepted the technical correction but held complete
profile/history/diff evidence. Revisions 58–60 correct that evidence, and dual
terminal verification passes. W2B's resumed terminal eight-cell rerun also
passes with `2.220e-15 m` maximum mass closure. Its frozen albedo response
remains immaterial and receives no promotion. Fresh terminal-v2 dual review,
dual verification, and exact-diff closure pass. EB-04X may advance. A new result-bearing
promotion factorial remains inadmissible until the relevant successor closes
its authority, observability, ownership, and independent-data gates.

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

The sequence may loop only through a newly authorized defect- or process-shaped
package with a named finding, write set, authority, and acceptance test. The
EB-04U–04X sequence is a new prospective study cycle, not a rerun of EB-04R or
EB-04S. It may not loop through coefficient search, guard relaxation,
result-aware metric changes. EB-04W1 and its prospective EB-04W2 extension are
separately frozen calibration studies, not retries or promotion rounds; their
user-authorized forcing scaling does not alter EB-04R/04S evidence or
stop-loss outcomes.

### 6.2 Terminal Meltout And Receiving Surface

The Stage 3 shadow may not stop physics permanently at the libsnobal
`1 kg m^-2` resolved-layer boundary. The authorized planning target is a
persistent cross-day shadow with a one-volume shallow-snow enthalpy state and
an implicit or error-controlled terminal integrator. The earliest combined
melt/sublimation solid-exhaustion event is localized within contract tolerance;
the snow ledger closes at the event; generated and terminally retained liquid
is dispositioned once through infiltration before residual runoff; and the
surface changes to a reviewed snow-free land-surface regime selected from
actual cover, liquid, and frost state for the remainder of the interval.

Do not assign the snow-computed terminal excess directly to soil. Recompute
the remaining-interval radiation, turbulent, evaporation, precipitation, and
ground/soil terms using the new surface state. A seasonal claim additionally
requires coupled persistence of all affected surface-liquid, soil
thermal/water, frost, and evaporation state; a snow-only shadow is event-local
evidence. Missing receiving-surface or coupled-state authority is a Phase-1
pre-implementation contract hold, not permission to install an energy sink.
This work remains shadow-only until a separate atomic cutover closes every
owner, consumer, default, rollback, and assurance gate.

The 2026-08-06 Phase-1 audit established an executed authority `HOLD` before
terminal production edits; independent review and dual terminal verification
pass. Pinned libsnobal supplies a
sub-threshold snow-to-water precedent but no error-controlled terminal event;
pinned WEPP `tmpadj` supplies a frost surface-temperature driver but no closed
general land-surface energy ledger.

The predecessor complete-carrier execution materially changes the order. Its
corrected frozen Snowbird screen remains `+170.2536 MJ m^-2`, while daily
post-CoE reinitialization makes the chronology predictions not evaluable. The
near-zero resolved-domain `Q_unallocated_after_exhaustion` is only a
pre-vapor-debit allocation result above the thin-pack cutoff; it does not close
whole-shadow state energy or terminal meltout. A receiving-surface subsystem
cannot repair this upstream carrier plausibility failure.

The revised sequence is therefore:

1. scope a declared evaluation shadow under `INV-SNOWFREEZE-091`, freezing
   same-state carrier and sequential resolved-shadow operators;
2. mechanically extract the shadow solver, then publish shadow-specific flux,
   cold-content-export, mass, residual, and coverage operands;
3. run a frozen paired-window per-term audit at Mica Creek, Niwot, Paradise,
   and Snowbird;
4. admit and run persistent accumulation-season shadow state only if that
   audit is physically interpretable;
5. admit terminal enthalpy-event numerics separately from the first-class
   snow-free land-surface energy subsystem; and
6. consider one atomic CoE retirement only after all owner, consumer, default,
   rollback, assurance, terminal, and receiving-surface gates pass.

The land-surface subsystem remains necessary for post-meltout continuation and
cutover, but it is a cross-domain program rather than the next snow diagnostic
prerequisite. Until then, event-local or persistent shadow results must be
censored at every unresolved terminal/receiving boundary.

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
- No default activation: the admissible EB-04S retained adjudication closed
  nonpromotion under the unchanged empirical rule.
- No tuning of shared meteorological radiation to improve snow.
- No reuse of validation observations for fitting.
- No replacement of the current snow-density, phase, liquid-routing, frost, or
  canopy-phenology defaults outside a successor's explicit contract.
- No public output-schema change merely to support campaign diagnostics.
- No claim that comparator agreement establishes scientific correctness.

Canonical process authority remains in `SC-SNOWFREEZE-001` and any reviewed
successor amendment; this roadmap is sequencing and experimental-design
authority only.
