# Owner authorization: snow accuracy-versus-runtime experiment

## 1. The pivot is the assignment

**Execute an end-to-end modeling experiment, not another behavior-preserving micro-optimization.**

The objective is a watershed model that can run at the required scale with quantified, acceptable losses of numerical and process resolution. The present high-detail implementation is a comparison reference, not the definition of scientific truth or the mandatory computational procedure.

Previous work repeatedly preserved equations, arithmetic order, nonlinear trajectory, coupling cadence, exact transition behavior, and independently charged evaluation roles, then demanded that a small disjoint operation plausibly save 5% of the whole runner before implementation. That was useful for ruling out some implementation waste. It cannot determine whether the numerical specification itself is unaffordable. Do not repeat that approach.

This authorization deliberately permits giving up unnecessary precision, intermediate-state agreement, temporal resolution, and detailed trace-snow representation. Measure the consequences for runoff, meltwater delivery, consequential peak timing, SWE, sublimation, and soil thermal/water state. A physically elaborate model that cannot execute the intended application is not an adequate engineering result.

The hypothesis is that resolving small snow inventories and their transitions, together with tightly converged repeated coupling, imposes disproportionate cost. **Test that hypothesis; do not assume it is true or promise a thousands-fold speedup.** Distinguish inexpensive snow arithmetic from expensive downstream physical evaluations requested by snow/error/event control.

The required endpoint is an implemented accuracy–cost experiment with real-runner results: an accuracy–runtime frontier, a causal explanation of the changed work, and a recommendation about the next model fidelity. A carefully executed negative or mixed result is valid. Another source map, oracle-only stop, or statement that the existing contract forbids changing accuracy is not the intended endpoint.

## 2. Package, baseline, and efficient execution

Create or resume the equivalent package:

`docs/work-packages/YYYYMMDD-stage3-snow-accuracy-runtime-001/package.md`

Use the actual execution date in `America/Los_Angeles`. Read the current locator before creating a duplicate. Keep `package.md` as the single maintained narrative under current repository governance. Raw results, reproducible analysis, source patches, fixtures, and necessary receipts may be separate; do not generate a parallel set of handoff/review/manifest prose files.

The review checkpoint is `f8783de68ae97529eeacf87d31db7df65f063453`. Inspect actual HEAD and relevant drift; do not reset the user's checkout. Preserve unrelated files, branches, prior experiments, and historical FAIL/HOLD results.

Use the existing authentic runner, collector, timing boundaries, and retained experimental source where available. Distinguish the repository commit from any detached source composition. Freeze a strict reference **R0** with no newly introduced F, R, or inactive-Jacobian treatment. Use one common observation/metrics patch for comparisons. Prove its neutral, reference-policy execution against R0 once. Do not repeat historical source/index reconstruction; unavailable old assets permit a newly identified reference, with the limitation stated.

Use isolated experimental source/build directories. Keep the production default and primary runtime source unchanged. Local scoped commits of the package, necessary experimental authority, reproducible patches, and evidence are authorized. No push, deployment, production activation, or named-branch creation/switch is authorized.

Read root and nearest instructions, the current work-package guide, testing strategy, and the affected science-contract and numerical-solver sections. Start with the latest native-pipeline package summary; earlier studies are on-demand evidence, not mandatory onboarding. Relevant source anchors include:

- `hydrology/support_helpers_mod/runoff_reconciliation.rs`: thin/resolved classification and terminal coupling comparisons.
- `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/{evaluation,terminal_event}.rs`: direct endpoint versus nested terminal integration, coupling calls, step rejection, and phase accounting.
- `snow_stage3_v11_adaptive_execution.rs`, `snow_stage3_v11_adaptive_frontend.rs`, and `canonical_owner_bytes.rs`: complete-owner error control, discrete comparisons, and actual rejection diagnostics.
- The covered LSE solver, native physical builder, surface-liquid/WB14, and soil consumers actually reached by the selected policies.

Resolve their exact paths through current source. Read affected `SC-SNOWENERGY`, `SC-LANDSURFACEENERGY`, `SC-COUPLEDTIME`, `SC-SURFACELIQUID`, soil, and vegetation obligations as needed, not whole catalogs by default.

Two complementary independent reviewers are required for the changed numerical/model authority and experimental implementation: correctness/modeling, and QA/evidence. They verify accepted fixes in their scopes. No automatic extra verifier wave or mandatory runner-agent layer. Serialize builds and measured execution; reviewers can inspect source independently outside timing windows.

## 3. Explicit experimental authority

This prompt supplies owner authority to create narrowly scoped experimental amendments **before dependent implementation** for the following changes:

1. Dimensioned local-error, coupling, and nonlinear stopping policies, including removal of inappropriate near-zero relative-error demands and unnecessarily strict step-norm requirements.
2. Conservative reduced-order trace/thin-snow representation, including changed entry/exit thresholds and loss of internal layer detail.
3. Less frequent snow–canopy–soil coupling, a declared predictor/corrector or lagged exchange scheme, changed temporal error-estimation strategy, and bounded transition-time resolution.
4. A composite of the preceding changes, with interaction measured rather than speedups multiplied.

Earlier restrictions requiring the old trajectory, every Full/Half1/Half2/Root evaluation, the old physical timestep policy, exact old transition times, bit-identical cross-model physical outputs, or a preimplementation 5% gain are **not constraints on these newly identified experimental profiles**. Amend affected authority and tests explicitly; do not silently bypass them or rewrite historical conformance.

Changing temporal integration or surface representation is a declared numerical/model approximation, not a behavior-preserving optimization. An experimental model may approximate fluxes or coupling over a finite interval when the scheme, conservation accounting, validity range, and error evaluation are specified. The precise governing model being evaluated must be identifiable in every result.

Do not import unrelated changes to runoff/erosion constitutive laws, plant parameters, weather forcing, routing geometry, physical constants, or calibration. Start by keeping deep/resolved-snow physics unchanged except for the separately identified numerical/coupling policy. No neural surrogate, new general solver framework, GPU port, historical F/J salvage, or support-clone/hash optimization belongs here.

A small immutable policy selected at run startup is appropriate in the isolated executable. State-dependent thin/resolved-snow selection within a declared model is allowed. A failed candidate must not silently retry through R0, switch models, return a partial day as success, or fabricate the old evaluation counts.

### Separate the obligations correctly

**Remain mandatory:** finite valid states; nonnegative physical inventories; accounted water and energy; correct units/areas; no duplicated or missing intercomponent transfer; continuous chronology without gaps/overlap; honest forcing integrals; sound ownership; failure rollback; reproducibility and restart behavior under each declared policy.

**May change under the experiment:** integration error, algebraic solve accuracy, intermediate fluxes/states, layer count, internal representation, evaluation count/order, numerical failure incidence, event count/timing, and accepted physical outputs within the reported accuracy bands.

**Do not conflate conservation with integration accuracy.** Split any source constant currently used for both a closure check and an integration/coupling tolerance. Relaxing the latter must not silently relax the former. Independently audit the new scheme's actual transfers and inventory balances; do not hide discrepancies by naming them a residual reservoir. If an existing check conflates algebraic solve residual with transfer-accounting closure, identify and prospectively separate those meanings rather than preserve the conflation.

Existing internal residual/trajectory goldens are not automatically application accuracy requirements. Conversely, passing an approximate trajectory comparison does not establish conservation or correct physical behavior.

## 4. Establish the few causal facts needed to run the experiment

Use a compact initial inspection and one untimed diagnostic execution per necessary regime. Do not start a new broad profiling campaign.

For each numerical control to be changed, record its units, baseline value, exact caller/branch, whether it is exercised, and whether it actually causes iteration/rejection/refinement. Distinguish:

- ordinary resolved-snow timestepping;
- thin/terminal local error estimation;
- terminal coupling iteration;
- outer complete-owner error control;
- exact discrete/active-set mismatch;
- LSE residual versus prospective step-norm convergence;
- event detection/localization and minimum-support admission.

**A changed but unexecuted constant is not a treatment.** The current direct-endpoint mode can bypass the terminal full-versus-refined loop. Do not infer that `RELATIVE_ERROR_TOLERANCE=1e-8` is driving the benchmark merely because it appears in `terminal_event.rs`. Verify the actual path and distinguish direct endpoints from discovery/error-estimation work.

Check the stopping-component diagnostics: tiny surface-liquid/BGC values, exact layer-membership comparisons, or a step norm may drive a rejection attributed casually to snow. Report the actual driver. Revised error control may use physical aggregate state instead of exact internal layer membership, but it must not ignore changed destinations, missing transfers, invalid state, or consequential omitted processes.

Record nanosecond timestamp representation separately from physical timestep size. Identify the actual temporal floor and its active use. Preserve exact chronology for the chosen approximate intervals; timestamp width is not an accuracy target.

The result of this step is a short control map and runnable policy seam, not another admission report requiring perfect attribution before code.

## 5. Implement a small, interpretable fidelity ladder

Use one reference, a numerical-precision family, a trace-snow family, a coupling/cadence family, and at most two combinations. Keep no more than about twelve named policies in the first screening matrix; do not run a full Cartesian product.

The values below are **proposed experimental settings**, not claims about physically sufficient accuracy. Review units and source applicability, then freeze exact policy values before comparative execution. They may be adjusted prospectively for a concrete mathematical/domain reason, not fitted after observing candidate errors.

### A. Numerical-precision family: same process representation

Start with two relaxed profiles:

| Snow/coupled numerical control | P1 | P2 |
| --- | ---: | ---: |
| Relative integration/coupling error | `1e-4` | `1e-3` |
| Absolute snow ice/liquid error | `1e-4 kg m^-2` | `1e-2 kg m^-2` |
| Absolute integrated-energy error | `0.1 J m^-2` | `1 J m^-2` |
| Temperature iteration/step scale | `1e-3 K` | `1e-2 K` |

Apply these only to the relevant numerical controls, never to conservation, identity, or invalid-domain checks. Do not tighten an already looser baseline control just to match the table. Report each effective setting and its absolute-plus-relative formula. A mass tolerance must not be pasted into water-depth, carbon, concentration, or energy fields with different units.

Allocate inner residual/iteration error consistently with the outer requested accuracy; do not leave an unnecessary `1e-9 K` coupling test or `1e-8 K` step requirement as a hidden veto. Retain a defensible finite residual/error estimate. Do not replace convergence with unconditional success after an arbitrary iteration count.

If a grouped numerical profile changes work, add at most two targeted ablations to distinguish the decisive control families. If it changes no executed acceptance decisions, report that fact and proceed to the representation/cadence families. A negative tolerance result does not block them.

### B. Conservative trace-snow family: deliberately less detailed physics

Implement one reduced bulk ice/liquid/enthalpy model for small represented snow. Use exploratory entry thresholds of **0.1, 1, and 5 mm SWE**, with a prospectively defined exit rule. The 5 mm profile is a stress test, not a proposed default. Use total retained snow inventory consistently and specify how ice/liquid contribute to classification.

The trace treatment must avoid simply running the old fully coupled thermal/terminal procedure under a different name. It should remove fine layer-temperature evolution and/or repeated detailed terminal solves while explicitly evolving bulk mass and enthalpy and delivering actual meltwater/energy exchanges.

Before implementation, specify its state, equations, flux approximation, phase partition, timestep, and conversion in both directions. Requirements:

- Preserve total water and enthalpy through layer-to-bulk conversion and back, within independently justified arithmetic accounting error. Lost stratification is a declared approximation; creating or deleting inventory is not.
- Do not melt the remnant for free, delete it, permanently freeze it in a residual store, or divert unexplained energy into soil. Pay latent heat and account for refreezing, sublimation/deposition, rain/snow enthalpy, and meltwater release consistently.
- Do not divide vanishing mass into a badly conditioned temperature and then attempt to resolve that temperature to extreme precision. Use a documented enthalpy/phase formulation suitable for the reduced state.
- Surface albedo, roughness, radiation, evaporation/sublimation, insulation, and snow–soil heat exchange must have a declared trace-regime treatment. Tiny SWE does not automatically imply negligible radiative or soil-thermal effect.
- A hysteresis band is allowed to prevent representation chatter, but must be fixed from policy/state, preserve inventory, and be tested under repeated snowfall/melt/reappearance. Do not tune it by case identity.
- Permit approximate disappearance timing under the evaluation budget. Use a consistent within-step regime/flux treatment; do not knowingly apply the wrong surface over an unaccounted interval or duplicate the terminal water parcel.
- Unsupported states return an honest typed outcome. The chosen model's declared transition back to resolved snow is legitimate regime selection, not a hidden strict-solver rescue.

Test this family with otherwise reference-like numerical controls first where that comparison is meaningful. Label differences that necessarily accompany the reduced model; do not claim a perfectly independent factor when it is not.

### C. Coupling/cadence family: buy fewer expensive evaluations

Implement one explicit numerical scheme with target exchange intervals of **600 and 1800 seconds**, limited by actual forcing discontinuities and materially consequential events. These are experimental schedules, not guaranteed stable steps or mandatory minima. Keep rainfall/runoff dynamics at their required evaluation resolution; do not coarsen every process merely because snow coupling is less frequent.

A bounded predictor/corrector or conservative lagged exchange scheme is allowed. Replacing always-executed step doubling or repeated fixed-point convergence with a declared alternative is explicitly in scope. State what error/stability information is retained and when the scheme subdivides; do not claim it provides the old embedded error guarantee.

Use the same integrated snow–soil energy exchange with opposite signs in the participating budgets and a consistent water transfer. Impose finite/nonnegative/phase guards and a documented response to unresolved instability. Any lag, exchange interpolation, event approximation, or coalescing is visible in the method specification and metrics.

Evaluate coupling-only against R0 before the selected combined profile. Do not require every old Full/Half1/Half2/Root role to execute: reducing that work is part of this experiment. Preserve physical/custody obligations, not historical role counts.

### D. Combinations

On the development cases, select at most two combinations using the frozen selection rule: nondominated cost versus error, then the fastest policy meeting each predefined error band. Carry those fixed policies into held-out validation. Do not multiply isolated speedups or conceal interactions. Once held-out results are inspected, a changed policy is a new exploratory revision, not the same confirmed candidate.

## 6. Freeze application-oriented comparisons before screening

Use common initial physical inventories and external forcing, not independently equilibrated states that conceal transient differences. Map a reduced state into common physical outputs conservatively; do not require identical internal arrays or layer IDs.

The owner has not supplied a final calibrated application error budget. Therefore use the following **prospective comparative screening bands**, expressly not production acceptance or observational truth. Report every metric in physical units as well as its band classification. These numbers guide selection without requiring another permission request; reviewers may correct a dimensional/inapplicable definition before screening, with rationale.

For value comparisons, the allowance is `absolute + relative * abs(reference)` on the stated interval/basis. Never use a candidate-dependent denominator that grows to excuse its own error.

| Metric | Tight comparative band | Coarse comparative band |
| --- | --- | --- |
| Daily/common-time SWE | `0.1 mm + 0.1%` | `1 mm + 1%` |
| Daily meltwater delivered and daily runoff depth, separately | `0.1 mm + 0.5%` | `0.5 mm + 2%` |
| Cumulative melt/runoff over a window | `1 mm + 0.5%` | `5 mm + 2%` |
| Cumulative sublimation/deposition, each separately | `0.1 mm + 1%` | `0.5 mm + 5%` |
| Event peak runoff, equivalent contributing-area depth rate | `0.1 mm h^-1 + 2%` | `0.5 mm h^-1 + 5%` |
| Matched consequential melt-out/runoff-peak timing | `15 min` | `60 min` |
| Soil temperature at fixed available depths | `0.1 K` | `0.5 K` |
| Root-zone water storage | `0.1 mm + 0.5%` | `1 mm + 2%` |
| Frost/thaw depth, where the model supplies it | `0.01 m` | `0.05 m` |

For windows shorter than a day, keep full trajectories and event metrics; do not prorate the daily allowance into a fictitious convergence theorem. Show cumulative signed bias and maximum excursions, not only average error. Report absolute and relative peak differences separately so the absolute floor cannot hide creation or suppression of a consequential runoff event.

Define one common physical event threshold and persistence rule before screening, independent of each candidate's trace threshold. Report exact mathematical zero times separately from hydrologically consequential disappearance. Unmatched/missing significant events are explicit failures of that band, not discarded pairs. Do not redefine the event after a candidate misses it.

Compare hydrographs on a shared reporting grid fine enough to expose the promised peak/timing differences. Do not downsample the strict hydrograph to the candidate's coarse coupling intervals and claim unchanged peaks. Where the current output cannot resolve a metric, add a bounded common output or mark the metric unresolved; it cannot be counted as passed. Do not time-warp trajectories or align their peaks before error calculation.

Conservation and transfer-integrity checks are independent hard requirements, not these comparative bands. Record daily, event, and complete-window water and energy residuals, cumulative absolute imbalance, and equal/opposite transfers. Retain the governing roundoff-scale accounting checks where they apply; derive an independent conservative audit for newly represented storage. Changed integration tolerances do not authorize unexplained imbalance.

## 7. Cases, reference adequacy, and meaningful scale

Freeze a small development and held-out inventory before policy outcomes. Reuse existing lawful fixtures and weather; author minimal new input cases through legitimate constructors only when a necessary regime is absent. Do not make a new dataset-ingestion or calibration project.

Cover, with a compact set of approximately six regime cases:

- snow-free execution as an overhead/negative control;
- substantial persistent snow under cold conditions;
- gradual melt-out through the trace thresholds;
- rain-on-snow or rapid melt with consequential runoff;
- repeated light snowfall/deposition, disappearance, and reappearance;
- freeze/thaw and thin-snow insulation over frozen/thawing soil.

Include both real positive-area vegetation and an open/inactive-canopy condition where supported. The familiar one-OFE day is continuity evidence, not the whole application. Negative controls must not be mistaken for policy participation.

Use short continuous windows for initial screening, then at least one continuous multiweek cold/transition window and one distinct held-out intermittent-snow window for finalists. Use existing longer/seasonal forcing to test cumulative drift when practical. Advance state continuously; repeated one-day resets are not a seasonal simulation. A timeout or missing reference remains visible and is not replaced by synthetic timing or a successful prefix.

R0 is the strict numerical reference, not measured truth. Check reference adequacy on selected cases with one further temporal/numerical refinement or an independent analytical balance where available. If tighter R0 is unstable, prohibitively costly, or materially changes the outputs, record a reference limitation rather than certify candidate accuracy against an unreliable target. Use existing observations or independent validated comparisons when already available; do not postpone execution for a broad field-data search. Where only R0 exists, say **agreement with R0**, not physical accuracy.

Finalists must enter the actual integrated runner and at least a genuine 10-OFE case with treatment participation on the intended lanes. Do not reuse an earlier scale fixture that happens to avoid the changed path and call it a scaling pass. Add 19 OFEs and a longer continuous window when practical; report measured topology, regime coverage, runtime, and limits. Unavailable larger-scale evidence limits the scalability conclusion, not the validity of completed local results.

Recover the campaign throughput requirement with its actual units, OFEs, simulated years, worker count, and overhead assumptions. Compare measured costs to that requirement. Do not assume the five-second stress day represents every day or promise a 5,000× change. Report per-regime performance and clearly labeled workload-mixture scenarios; do not extrapolate one thin-snow event to a century as though it were a measured distribution.

## 8. Measurement and causality

Use an explicit run-level source/build/policy/input identity, common compiler/flags, matched output policy, fixed supported CPU placement, and no concurrent heavy jobs. Build outside timing. Preserve raw stdout/stderr, exit/timeout, finalized receipts, and analysis from the first run. Extend the existing collector to permit declared cross-policy physical differences instead of globally disabling its checks.

Only R0/common-observer neutrality requires exact old output/work identity. Candidate admission requires its policy identity, complete physical/time coverage, independent conservation/custody, and the comparison metrics above. Candidate counts, support partition, result bytes, and payload-derived hashes are expected to differ. Each policy's own producer-to-consumer transfer and reproducible rerun must remain correct. A policy cannot meet a timing target by emitting fewer required outputs or omitting validation.

Collect inexpensive aggregates by **requesting snow regime and evaluation role**, including downstream work:

- resolved-pack, trace-pack, melt-out/reappearance, and snow-free elapsed time and simulated duration;
- accepted/rejected steps, dt histogram, event/root iterations, refinement/floor hits, and representation switches;
- coupling iterations and stopping/rejection component;
- complete carrier/LSE/soil/hydrology evaluations and expensive leaf solves;
- work spent on discarded trials versus accepted updates;
- ordinary RSS, peak where readily available, and any observed growth over continuous runs.

Use nested/exclusive accounting to avoid counting downstream work twice. Report a descriptive “work invoked by the trace regime” separately from the **causal runtime difference under a changed policy**. Control-only source counters cannot prove that a tolerance caused the cost.

Keep diagnostic tracing separate from minimal-observer timing, and check observer impact on a representative reference/candidate pair. No arbitrary subtraction of estimated observer time. CPU time and wall time are separate results.

For screening, perform a correctness/metrics run and a confirmatory execution per policy/case as useful. This is not a separate 12-pair campaign for every knob. Then freeze R0 plus at most two finalists and use two warmups per executable followed by six balanced fresh-process comparisons on the selected short representative workloads. One extension to twelve pairs is allowed only when uncertainty can change the decision. Longer windows need continuity and reproducible outcomes rather than gratuitous replication.

Report paired runtime differences/ratios, raw ranges, descriptive uncertainty, failure rate, and error by case and regime. Include every valid slow run. Failed or timed-out policies belong in the results with their failure point; do not compute a favorable speedup over only the cases they finish.

A small speedup is not a failure to execute the experiment. There is no per-bucket 5% admission barrier here. Select the model fidelity on the measured cost–error frontier and the application's compute requirement, not on a predetermined promise of speedup.

## 9. Validation without rebuilding the previous bureaucracy

Before changing each method, record its equations/assumptions, supported domain, budget/accounting, treatment policy, and affected authority. Have the two reviewers disposition the scientific and evidence plan before comparative results. Keep this proportional: a finite experiment specification, not a wholesale contract rewrite.

Implement focused analytical/contract-derived tests for constant-forcing energy and melt, refreezing, finite vapor transfer, no-snow behavior, conservative layer/bulk conversion, threshold chatter, forcing discontinuities, rain-on-snow, and common-interface exchange. Exercise errors, nonfinite inputs, missing/foreign state, rejected updates, and restart across both trace/resolved transitions. A candidate restart should reproduce its own uninterrupted run under its declared semantics, not recreate R0's old layers or receipt lineage.

Tests that asserted the superseded numerical policy must either remain bound to R0 or receive explicitly identified experimental counterparts. Preserve genuine defect tests and physical requirements; do not widen unrelated assertions to make a profile pass. Where a changed model cannot expose an old internal quantity, use an explicit physical projection and mark loss of that diagnostic resolution.

Run affected formatting, lint, focused/integration and actual-consumer tests, independent balances, applicable A0/A1/A3 obligations, and the current required full-correctness evidence for consequential changes according to the testing strategy. Known unrelated baseline failures are not candidate regressions, but neither are they all proven inherited without comparison. Preserve unclassified failures and production HOLD. A changed numerical/model obligation needs prospective authority, not an exemption inferred from timing success.

Keep execution validity, comparative model findings, and integration/release qualification as separate conclusions. Applicable unmet correctness or evidence requirements prevent qualification; they do not require erasing already valid measurements. Do not spend this package fixing the entire historical workspace or initiating a new governance system.

Correct implementation/instrumentation/receipt defects within scope and retain superseded attempts. Exploratory design can be revised at most twice for a concrete demonstrated numerical or modeling defect; record new policy IDs and rerun affected cases. Do not stop merely because a planned development run exposed a fixable defect, and do not retune screening bands or held-out selection to manufacture acceptance.

## 10. Required result and stop conditions

Deliver executable policies and reproducible evidence, with one compact result table containing:

`policy | method/precision given up | cases completed/failed | runner time/speedup | complete-evaluation reduction | SWE/melt/runoff/peak/soil errors | conservation | memory | reference limits | qualification`

Provide accuracy-versus-runtime plots or equivalent machine-readable frontier data, per-regime cost, and the selected policy's long-window and multi-OFE results. Keep individual output metrics visible; do not hide tradeoffs inside a single arbitrary score. The final recommendation must state what is being sacrificed, what is preserved, and whether the measured improvement materially advances the actual workload budget.

Explicitly answer:

1. How much measured work is requested by snow/trace-transition handling, including induced coupled evaluations?
2. Which demanded precision or schedule actually causes extra work, and which proposed controls proved irrelevant?
3. What happens when tolerances alone are relaxed?
4. What additional cost/error change comes from bulk trace snow and less frequent coupling?
5. Do the effects survive continuous evolution, positive vegetation, significant runoff events, and multiple OFEs?
6. Which policy meets each comparative band, and what remains unknown about physical accuracy and production readiness?
7. Is the remaining scale gap mainly numerical tolerances, model detail/coupling, another measured process, or still unresolved?

A reduced-order scheme that violates conservation or creates consequential output errors is rejected even if fast. A slower or ineffective but correctly executed treatment is a valid negative finding. A useful speedup with acceptable comparison errors is a candidate for subsequent integration/validation, not automatic production release. If the full ladder still cannot approach the application budget, recommend the next explicit reduction in model complexity or a different snow/coupling formulation, with the measured reason—not another cache or small-wrapper experiment.

**Finish with the best supported cost–accuracy tradeoff, not another guarantee that nothing changed. The pivot is to an affordable model with quantified fidelity, not an unaffordable model with perfectly preserved numerical behavior.**

---

### Limited reference notes for the authoring agent

Repository context is pinned to `f8783de68ae97529eeacf87d31db7df65f063453`; recheck relevant source rather than treating these notes as an authority replacement. `terminal_event.rs` has a direct-endpoint branch as well as the inner full/refined path. `canonical_owner_bytes.rs` supplies separate outer comparison policies. The current package guide uses one maintained record and consequence-based independent review.

Two external primary documentation references support the experiment design, not its proposed numerical settings:

- SUNDIALS CVODE, “General advice on choice of tolerances,” distinguishes near-zero component noise levels, local tolerance, and accumulated/global error; it recommends testing tolerance sensitivity. This prompt does not propose replacing the solver with CVODE. https://sundials.readthedocs.io/en/latest/cvode/Usage/index.html#general-advice-on-choice-of-tolerances
- CTSM technical note, “Snow Hydrology,” documents snow layer management in a land model. Consult only relevant sections if useful for the selected bulk/remnant derivation; do not import thresholds, storage conventions, or phase transfers without checking their meaning. https://escomp.github.io/CTSM/tech_note/Snow_Hydrology/CLM50_Tech_Note_Snow_Hydrology.html

The candidate settings and comparative bands in this prompt are prospective engineering choices for this study, not values established by those references.

