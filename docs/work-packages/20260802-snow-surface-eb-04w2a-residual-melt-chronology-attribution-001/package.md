# SNOW-SURFACE-EB-04W2A Residual Melt-Chronology Attribution

Status: `executed / hold / direct attribution admitted / albedo contrast withdrawn`

Date: `2026-08-02`

Campaign: `SNOW-SURFACE-EB`

Plan class: `Diagnostic mechanism attribution`

This living ExecPlan follows `docs/codex_exec_plans.md` and remains
self-contained as execution proceeds.

## Purpose / Big Picture

EB-04W2 approximately corrected peak snow magnitude with site-specific total-
precipitation factors, but Mica Creek still melts out 21 days early and Niwot
and Snowbird retain early peak chronology. Paradise reaches zero-day melt-out
error. EB-04W2A uses those mass-controlled cells to determine whether the
remaining chronology is most consistent with excessive shortwave melt,
temperature/dewpoint/wind melt, melt concurrent with positive modeled cold
content, liquid retention/release behavior, or missing late accumulation.

This is not EB-04W3. It does not extend or refit precipitation, change kernel
physics, promote a melt model, or treat diagnostic replays as production.

## Objective

Retain the four exact EB-04W2 selected forcing cells; independently reconstruct
their phase-conditioned CoE melt, cold-content, liquid, refreeze, snowfall, and
storage ledgers; run the existing diagnostic `openwepp-snowbench coe-melt`
harness for `legacy_coe` and `coe_shortwave_albedo_v1` on the same scaled
fixtures; and rank the supported residual-chronology hypotheses without adding
or tuning coefficients.

## Implementation Intent

Intent: `diagnostic-research + calibration-readiness`.

Production science remains `IMPLEMENTED` and unchanged. Calibration evidence
remains `EMPIRICALLY_CALIBRATED` only for the exact EB-04W2 forcing/site pairs.
Mechanism identifiability begins `PARTIALLY_IDENTIFIABLE`: the package may
support or deflate hypotheses, but no diagnostic harness result is independent
validation or production-promotion evidence.

## Prospective Diagnostic Freeze

The exact retained cells are Mica Creek `1.4`, Niwot `1.7`, Paradise `1.8`, and
Snowbird `2.0`. Their EB-04W1/W2 fixtures, direct-production traces, WAT files,
observations, selectors, and hashes are immutable anchors.

The only new model executions are eight diagnostic harness cells: four lanes
times `legacy_coe` and `coe_shortwave_albedo_v1`. Both use the same selected
scaled fixture and configured management/canopy surface. The harness pair is a
within-harness albedo contrast; it is not asserted to reproduce the current
direct-production density, liquid, or Stage-3 state.

For each water year, preserve the inherited W2 observation operator. Define the
chronology-gap window as modeled-to-observed peak date for peak operators and
the final 60 days ending at modeled melt-out for melt-out operators. Also report
the complete accumulation and ablation phases so the fixed window cannot hide
opposite-season behavior.

Report, without result-aware threshold changes:

1. signed and applied `amelt`, `bmelt`, `cmelt`, and `dmelt` contributions;
2. snowfall SWE, rain retained/released, routed melt, retained liquid, refreeze,
   sublimation, and modeled-zero redistribution;
3. applied melt on days beginning with positive Stage-3 cold content;
4. direct-production chronology and magnitude anchors;
5. within-harness legacy-versus-albedo changes in peak magnitude, peak date,
   melt-out date, and seasonal SWE trajectory.

Hypothesis flags are diagnostic and `ASSUMED_FOR_EXECUTION`:

- `ALBEDO_RESPONSE_MATERIAL` when the albedo replay moves the applicable
  chronology at least 5 days toward observation without increasing absolute
  log peak-SWE error by more than `0.10`;
- `COLD_CONTENT_MELT_COINCIDENCE_MATERIAL` when at least 10% of applied melt or
  at least `0.010 m` occurs on days beginning with positive cold content;
- `TURBULENT_EMPIRICAL_TERMS_DOMINANT` when absolute `bmelt + cmelt` exceeds
  absolute `amelt + dmelt` over the chronology-gap window;
- `LATE_INPUT_DEFICIT_SUPPORTED` when modeled snowfall during an early peak gap
  is smaller than the observed SWE gain over that same interval.

These flags rank follow-up questions. They do not establish unique causality,
physical correctness, parameter values, or promotion.

## Included Scope

- hash-bound retention of the four selected EB-04W2 cells;
- eight existing-harness diagnostic replays with no source modification;
- phase/window reconstruction of melt, cold content, liquid, refreeze,
  snowfall, storage, and chronology;
- hypothesis ranking, uncertainty and confounding disposition;
- accessible SVG figures with same-stem Markdown sidecars;
- calibration-readiness, dual review/disposition, dual verification, and
  roadmap/catalog handoff.

## Excluded Scope

- production Rust, contracts, tests, fixtures, observations, selectors,
  defaults, coefficients, schemas, or assurance authority;
- precipitation scaling beyond the retained EB-04W2 selections;
- a cold-content melt gate, component suppression, coefficient tuning, or any
  new melt equation;
- treating snowbench as the direct-production consumer;
- independent validation, transferability, default activation, or promotion.

## Intended Write Set

- this package tree;
- `docs/ROADMAP.md`, `docs/planning/snow-surface-energy-balance-roadmap.md`, and
  `docs/work-packages/README.md`;
- package-local tooling;
- ignored outputs under
  `target/snow_surface_eb04w2a_melt_chronology_attribution/`.

All production, contract, test, fixture, observation, assurance, and historical
package paths are protected.

## Phase Plan

### Phase A — Scaffold and freeze

Bind retained identities, diagnostic binaries, exact cells/models, operators,
windows, hypothesis flags, data roles, protected paths, and stop conditions
before new replay output.

### Phase B — Implement and self-test

Create a package-local runner with synthetic operator/window checks, retained
identity audit, exact command construction, and fail-closed inventory rules.

### Phase C — Execute and reconstruct

Run exactly eight snowbench cells, retain receipts and outputs, then combine the
within-harness contrast with independently reconstructed direct-production
phase/window ledgers.

### Phase D — Interpret and visualize

Render the frozen figures and rank supported hypotheses while preserving the
production-versus-harness and calibration-versus-validation boundaries.

### Phase E — Validate, review, and close

Run scoped identity, inventory, closure, figure, documentation, security, and
exact-diff gates; complete dual review/disposition, dual verification, prompt
archival, and campaign handoff.

## Acceptance Criteria

1. Freeze predates all eight new harness executions and binds every authority,
   input, binary, operator, flag, and stop condition.
2. The four retained selected cells and their source outputs pass exact identity
   checks without rerun or mutation.
3. Exactly eight unique harness cells complete successfully, with like-for-like
   legacy/albedo inputs within each lane.
4. Direct-production phase/window operands reconstruct from retained trace/WAT
   outputs with all inherited closures within `1e-12 m` and `1e-6 J m^-2`.
5. Harness and direct-production evidence remain explicitly separated.
6. Every hypothesis flag reconstructs from frozen rules, including negative or
   mixed results.
7. Every figure parses, is visually inspected, and has an accessible Markdown
   sidecar covering population, units, processing, uncertainty, and limits.
8. Calibration-readiness and scientific disposition prohibit unique causality,
   fitting, transferability, default, and promotion claims.
9. No protected production, contract, test, fixture, observation, selector,
   default, assurance, or historical evidence path changes.
10. All scoped validation, dual review/disposition, dual verification, prompt,
    and terminal exact-diff gates pass.

Any unmet current-scope criterion forces `HOLD`.

## Validation Selection

Risk is `Moderate / diagnostic harness plus retained-output analysis`. Required
gates are tool self-test/compilation, freeze and binary identities, exact
retained/new inventories and hashes, within-lane input parity, operand and
closure reconstruction, frozen hypothesis-rule reconstruction, SVG
parse/visual inspection, Markdown lint, security impact, exact diff, dual
review/disposition, and dual verification. Rust workspace suites are not
selected because the intended diff cannot touch Rust, contracts, manifests,
tests, or production inputs; terminal reconciliation must escalate if that
assumption changes.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent science/QA reviewers and two terminal
verifiers. Expected outputs are compact package-local Markdown artifacts; each
role has write access only to its named artifact. No heavy workspace suite or
comparator batch is selected.

## Progress

- [x] (2026-08-02) User directed residual melt-chronology diagnostic work.
- [x] (2026-08-02) Prospectively defined the retained cells, two-model harness
  contrast, phase/window operands, and result-blind hypothesis flags.
- [x] (2026-08-02) Invalidated a uniformly failed pre-simulation attempt caused
  by supplying the production `*-B.run` stem to snowbench; no scientific output
  was produced or inspected, and the failure was preserved before refreezing.
- [x] (2026-08-02) Corrected and froze the canonical scaled-fixture run-file
  identities before the result-bearing execution.
- [x] (2026-08-02) Executed all eight frozen snowbench cells and reconstructed
  the retained direct-production operands and figures.
- [x] (2026-08-02) Traced the failed harness SWE closure to the public snow-
  activation predicate and withdrew the albedo contrast from scientific use.
- [x] (2026-08-02) Dual review corrected the cold-content aggregation,
  restricted late-input screening to early gaps, bounded activation diagnosis
  to snowbench phase authority, and quarantined raw harness values.
- [x] (2026-08-02) Archived the execution prompt after review disposition.
- [x] (2026-08-02) Dual terminal verification independently passed the
  identities, inventories, direct closures, adjudicated publication, figures,
  prompt lifecycle, write set, and truthful package hold.
- [x] (2026-08-02) Final exact-diff reconciliation confirmed only the package
  tree and three authorized roadmap/catalog files changed.

## Surprises & Discoveries

- Observation: Paradise reaches zero-day melt-out error after precipitation
  calibration, so a universal melt-rate defect is already disfavored.
  Evidence: EB-04W2.
- Observation: Niwot and Snowbird retain peak-date errors, not directly observed
  melt-out errors; missing late accumulation can mimic premature melt.
  Evidence: inherited frozen operators.
- Observation: the selected production melt model uses the legacy absorbed-
  shortwave treatment, while an existing albedo-aware diagnostic model is
  implemented but not production-promoted.
  Evidence: runtime provenance and snow-frost fidelity strategy.
- Observation: snowbench regenerates its runfile by using the supplied run-file
  stem to discover fixture sidecars; a production `*-B.run` therefore points to
  nonexistent `*-B.sol/.man/.slp/.cli` files inside the scaled fixture.
  Evidence: eight identical pre-simulation input failures in invalidated
  attempt 1 and static snowbench source inspection.
- Observation: each successful harness model loses one warm-mean, no-prior-pack
  snowfall day: Mica `0.037672727273 m`, Niwot `0.045390000000 m`, Paradise
  `0.059136000000 m`, and Snowbird `0.070800000000 m` SWE.
  Evidence: paired `coe_melt_snow.csv` ledgers and
  `artifacts/harness-closure-investigation.md`.
- Observation: the public partition activates snow coupling only for existing
  SWE or a subzero daily mean. Its inactive outcome records no accumulation,
  even when the typed hourly forcing contains snowfall.
  Evidence: static production-path inspection at the exact source revision
  bound by the experiment freeze.
- Observation: the retained production attribution closes independently. The
  frozen site-level cold-content flag is false after separately medianing its
  two threshold arms, but per-window hits occur in Mica `8/23`, Niwot `16/40`,
  Paradise `0/19`, and Snowbird `12/22`; only Paradise is uniformly negative.
  Empirical `b + c` exceeds empirical `a + d` in the frozen window at Mica,
  Paradise, and Snowbird but not Niwot. After enforcing the frozen early-gap
  population, Niwot has `0/27` and Snowbird `5/16` late-input-deficit hits.
  Evidence: `artifacts/operand-lineage.md` and the frozen results JSON.

## Decision Log

- Decision: call this package EB-04W2A rather than EB-04W3.
  Rationale: EB-04W2 froze a prohibition on another precipitation-grid
  extension; this package changes no forcing factor and asks a different,
  mechanism-attribution question.
  Date/Author: 2026-08-02 / Codex.
- Decision: compare legacy and albedo behavior only within snowbench.
  Rationale: the direct-production selector does not admit the albedo model,
  and a diagnostic package cannot widen that production surface.
  Date/Author: 2026-08-02 / Codex.
- Decision: retain component and cold-content evidence as association.
  Rationale: empirical CoE components mix drivers and daily ordering does not
  prove an energy-causal melt gate.
  Date/Author: 2026-08-02 / Codex.
- Decision: preserve and supersede the first freeze after the uniformly failed
  command-construction attempt.
  Rationale: all cells failed before forcing export or CoE simulation, so no
  result-bearing evidence informed the correction; the canonical scaled-fixture
  `.run` is the source-defined harness input.
  Date/Author: 2026-08-02 / Codex.
- Decision: admit the independently reconstructed direct-production operands
  but withdraw every snowbench chronology, trajectory, and albedo flag.
  Rationale: all eight executions completed, but the like-for-like harness
  pair shares a `0.0377-0.0708 m` SWE closure defect. Identical bias between
  models does not restore admissibility because the lost event changes the
  subsequent state trajectory.
  Date/Author: 2026-08-02 / Codex.
- Decision: hold W2A and route a bounded activation/closure correction before
  any renewed albedo experiment.
  Rationale: acceptance criterion 10 cannot pass, and the corrective write set
  is production Rust, contracts, and tests explicitly excluded from W2A.
  Date/Author: 2026-08-02 / Codex.

## Outcomes & Retrospective

All eight frozen harness cells executed successfully, and the retained
production ledgers close within `2.221e-15 m` and `6.094e-08 J m^-2`. The raw
site-level cold-content flag is false only under its separate-median
aggregation; interannual threshold hits remain material at Niwot and Snowbird
and cannot establish an energy-causal gate. Empirical `b + c` exceeds `a + d`
at three sites, while Niwot has the opposite term balance. Under the corrected
early-gap population, only Snowbird supports the frozen late-input screen, in
`5/16` windows; this is not proof of deficient precipitation.

The albedo question remains unanswered. Each harness path silently bypasses
one snowfall event when the day begins without snow and has a warm daily mean,
because the public entry predicate never invokes the hourly snow accumulator.
The maximum missing SWE is `0.0708 m`. This mechanism is proven for the
snowbench typed forcing; the retained direct traces do not contain typed snow
on the four decisive dates, so EB-04W2B must reconcile phase/input authority
before generalizing a production correction. Harness values remain only in
hash-bound raw evidence; adjudicated summaries publish null albedo values.

Disposition: `HOLD`. A narrow successor must reconcile the activation contract,
make typed hourly snowfall sufficient to enter snow coupling, add a fail-closed
mass-closure guard and mixed-day regression, then rerun this exact frozen
contrast. No precipitation extension, coefficient tuning, albedo conclusion,
physics promotion, or default change is authorized here.
