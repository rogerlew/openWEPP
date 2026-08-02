# SNOW-SURFACE-EB-04W1 Precipitation-Scaling Calibration Experiment

Status: `complete`

Date: `2026-08-02`

Campaign: `SNOW-SURFACE-EB`

Plan class: `Empirical calibration sensitivity + chronology adjudication`

This living ExecPlan follows `docs/codex_exec_plans.md` and must remain
self-contained as execution proceeds.

## Purpose / Big Picture

Legacy WEPP users commonly scale precipitation early in calibration when a
point climate underrepresents precipitation over the modeled hillslope. EB-04W
showed systematic snowpack-effective input deficits at Paradise and Snowbird,
near-parity input at Mica Creek, and adequate median input at Niwot, but it did
not execute precipitation multipliers. After EB-04W1, a human can see whether
uniform total-precipitation scaling improves snow magnitude and chronology
together at each open mountain lane, or merely compensates for excessive
pre-peak loss.

This is empirical calibration evidence, not independent validation or process
promotion. It changes package-local copies of climate forcing only. Production
physics, defaults, selectors, source fixtures, observations, and public schemas
remain unchanged.

## Objective

Execute the baseline `B` snow configuration for Mica Creek, Niwot, Paradise,
and Snowbird over a prospectively frozen total-precipitation multiplier grid;
reconstruct the existing peak-magnitude and chronology operators; classify
each lane as precipitation-responsive, loss-limited, chronology-limited,
tradeoff-limited, or grid-boundary-limited; and report a site-specific
calibration recommendation without claiming transferability.

## Implementation Intent

Intent: `empirical-calibration + calibration-readiness`.

`science_implementation_status` begins `IMPLEMENTED`: EB-04W's authoritative
phase and CoE snow physics are unchanged. The four SNOTEL series are assigned
prospectively to `CALIBRATION` for this package. No observation is assigned to
`INDEPENDENT_VALIDATION`; therefore no predictive validation, generalization,
transferability, or promotion claim is possible.

The multiplier is an external forcing calibration coefficient, dimensionless,
applied uniformly to every nonnegative daily CLIGEN `prcp` depth. It is not a
new production model coefficient. Search values are `ASSUMED_FOR_EXECUTION`,
not physical bounds or priors.

## Prospective Experiment Freeze

The frozen grid is `0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5`. It is identical
for all four lanes. Stage 1 contains exactly 32 baseline runs: four lanes times
eight multipliers. The `1.0` candidate must reproduce the committed EB-04W
baseline outputs and operators.

The daily precipitation depth alone is multiplied. Event duration, peak-time
fraction, dimensionless peak-intensity shape, temperatures, radiation, wind,
dew point, management, slope, soil, and snow parameters remain byte-equivalent
to their source fixture values. The parser derives event intensity from scaled
depth and the unchanged duration/shape fields.

The observation operator is evaluated by water year using observed SWE/depth
at daily scale. Report:

- median modeled/observed seasonal peak SWE ratio;
- median modeled SWE retained on the observed peak date;
- the exact inherited seasonal peak-date or melt-out-date offset;
- median effective-input/observed-peak ratio;
- pre-peak modeled loss, sublimation, and applied CoE melt;
- water-balance and EB-04W diagnostic closure.

Candidate selection is result-blind and lexicographic. A candidate first must
improve both absolute log peak-ratio error and absolute chronology offset over
`1.0`. Among joint improvers, prefer a peak ratio in `[0.9, 1.1]`, then smaller
absolute chronology offset, then the multiplier closest to `1.0`. If none
improves both axes, report `NO_JOINT_IMPROVER`. If the selected candidate is
`0.8` or `1.5`, report `GRID_BOUNDARY` and do not call it calibrated.

For Niwot, the primary magnitude ratio is the SWE-peak ratio and the lane
chronology error is the maximum absolute offset across its frozen peak-depth
and peak-SWE operators. For every other lane, the sole frozen operator supplies
the chronology error and its SWE peak ratio supplies magnitude. “Improve” is
strict relative improvement beyond `1e-12`; ties do not qualify.

## Included Scope

- immutable reuse of EB-04W's four lanes, five operator definitions, observed
  records, exact release runner, diagnostics, and baseline results;
- package-local climate transformation with byte and numeric provenance;
- exact 32-run baseline multiplier grid;
- independent verification that every daily `prcp_scaled = multiplier *
  prcp_source` within roundoff and every non-precipitation daily token is
  unchanged;
- exact `1.0` replay against EB-04W WAT, trace, and operator outputs;
- magnitude, chronology, input, loss, and response-curve analysis;
- accessible SVG figures with same-stem Markdown sidecars;
- calibration-readiness matrix, dual review/disposition, dual verification,
  and roadmap/catalog handoff.

## Excluded Scope

- production Rust, science-contract, parser, fixture, observation, runfile,
  default, selector, or public-schema changes;
- scaling temperature, radiation, wind, duration, phase fractions, snowfall
  alone, rain alone, or intensity-shape coefficients;
- changing phase, melt, density, sublimation, longwave, liquid-routing, or
  canopy physics;
- global or transferable multipliers, probability priors, uncertainty
  distributions, independent validation, or promotion;
- choosing a new grid, objective, lane, metric, or data role after results.

## Intended Write Set

- this package tree;
- `docs/ROADMAP.md`, the snow campaign roadmap, and work-package catalog;
- package-local execution/analysis tooling;
- `target/snow_surface_eb04w1_precipitation_scaling/` runtime outputs.

Production source, canonical contracts, tests, source fixtures, observations,
assurance authority, and historical package evidence are protected.

## Phase Plan

### Phase A — Scaffold and freeze

Bind the predecessor hashes, four lanes, calibration data role, grid,
transformation semantics, operators, objective, selection rule, write set, and
prohibited claims before result-bearing execution.

### Phase B — Implement and self-test the forcing transformer

Create a package-local tool that copies each fixture into `target/`, scales only
daily CLIGEN precipitation, verifies all transformed tokens independently,
builds runfiles against the copied fixture, sanitizes `OPENWEPP_*`, and records
source/binary/tool/forcing identities. Synthetic self-tests must cover zero,
decimal, and large precipitation plus malformed-row rejection.

### Phase C — Execute the frozen grid

Run exactly 32 cells once with the release binary. Any partial execution is
`INVALID` until the exact inventory completes. The `1.0` rows must reproduce
the EB-04W baseline before scaled results are interpreted.

### Phase D — Analyze and adjudicate

Reconstruct magnitude, chronology, input, loss, and conservation metrics.
Apply the frozen lexicographic selection rule per lane. Report response curves,
boundary saturation, equifinality, and whether scaling fixes magnitude and
chronology jointly.

### Phase E — Validate, review, and close

Run package/tool tests, exact inventory and provenance checks, baseline replay,
documentation, formatting, security-impact, and exact-diff checks. Complete
dual independent reviews, disposition findings, dual verification, archive the
prompt, and issue the scientific/calibration handoff.

## Acceptance Criteria

1. The freeze exists before any scaled model output and hash-binds all inputs,
   roles, grid values, operators, and selection rules.
2. Exactly 32 unique lane/multiplier runs complete with return code zero.
3. Every transformed daily precipitation value closes to its source value
   times the multiplier, and all protected tokens remain identical.
4. Every `1.0` WAT/trace/operator result matches EB-04W exactly.
5. Mass, phase, accumulation, and melt-component closures remain within the
   EB-04W `1e-12 m` contract tolerance.
6. Candidate selection is mechanically reconstructed from the frozen rule and
   reports boundary-limited or no-joint-improver outcomes explicitly.
7. Results separate improvement in peak magnitude from chronology and pre-peak
   loss; a magnitude-only compensation cannot be called successful.
8. Figures parse and have accessible same-stem Markdown sidecars.
9. No protected production, contract, fixture, observation, selector, default,
   assurance, or historical evidence path changes.
10. Required focused execution, documentation, exact-diff, review,
    disposition, and verification evidence passes.

Any unmet current-scope criterion forces `HOLD`; it cannot be reclassified as
future work after execution begins.

## Validation Selection

Risk is `Moderate / analysis-only forcing calibration`. No production or test
source changes are intended. Required validation is transformer self-test,
source-versus-scaled token audit, exact inventory/provenance, `1.0` EB-04W
replay, diagnostic closures, package Markdown, roadmap/catalog Markdown,
Python compilation, exact diff, security impact, and dual review/verification.
Workspace Rust suites are not selected because the exact diff may not touch
Rust, contracts, manifests, or tests; terminal reconciliation must escalate if
that assumption changes.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent science/QA reviewers and two exact-terminal verifiers.
Expected outputs are compact package-local review/verification Markdown
artifacts; write access is limited to each role's named artifact. No heavy
workspace suite or comparator batch is prospectively selected.

## Progress

- [x] (2026-08-02) User authorized precipitation scaling as a legacy-WEPP
  calibration lever and directed immediate execution.
- [x] (2026-08-02) Scaffolded EB-04W1 and prospectively froze the calibration
  role, uniform grid, transformation, operators, and decision rule.
- [x] (2026-08-02) Implemented and self-tested the package-local forcing
  transformer; preflight proved precipitation-only changes.
- [x] (2026-08-02) Executed and analyzed all 32 frozen cells with zero runtime
  failures and maximum diagnostic closure `3.331e-15 m`.
- [x] (2026-08-02) Produced three accessible figures/sidecars, operand lineage,
  calibration-readiness evidence, and the scientific disposition.
- [x] (2026-08-02) Closed all dual-review findings, completed dual terminal
  verification, archived the execution prompt, and reconciled the terminal
  handoff.

## Surprises & Discoveries

- Observation: the existing campaign stop-loss prohibited forcing rescaling
  inside the EB-04R promotion factorial.
  Evidence: campaign roadmap section 7.
- Observation: the user's 2026-08-02 direction explicitly authorizes a separate
  calibration study because precipitation scaling is common legacy-WEPP
  practice.
  Evidence: current task direction; this package preserves EB-04R/04S evidence
  and makes no promotion claim.
- Observation: precipitation scaling improves both frozen objective axes in
  every lane, with 2-5 joint improvers per lane.
  Evidence: `artifacts/precipitation-scaling-results.json` and response curves.
- Observation: Paradise and Snowbird improve on both axes through `1.5`;
  Niwot magnitude continues upward despite its `1.3` chronology-tie selection;
  Mica Creek magnitude is closest to parity at `1.4` while chronology improves
  one more day at `1.5`.
  Evidence: `artifacts/scientific-disposition.md`.
- Observation: Snowbird reaches effective-input parity at `1.5` but retains
  only about `0.55` of observed peak SWE on that date.
  Evidence: input/storage adjudication figure and machine-readable results.

## Decision Log

- Decision: insert EB-04W1 without renaming EB-04X.
  Rationale: precipitation scaling directly follows EB-04W's input finding,
  while EB-04X retains its previously agreed Harvard interception scope.
  Date/Author: 2026-08-02 / Codex.
- Decision: scale total daily precipitation in package-local fixture copies.
  Rationale: this represents ordinary climate-forcing calibration without
  adding a production coefficient or mutating protected source fixtures.
  Date/Author: 2026-08-02 / Codex.
- Decision: treat all four SNOTEL records as calibration data.
  Rationale: the experiment selects multipliers from them; calling them
  independent validation afterward would be leakage.
  Date/Author: 2026-08-02 / Codex.
- Decision: recommend a separately frozen EB-04W2 upward grid extension rather
  than treating `1.5` as a calibrated value.
  Rationale: three selections are boundary-censored, and the current package's
  result-blind rule explicitly prohibits calling a boundary selection
  calibrated.
  Date/Author: 2026-08-02 / Codex.

## Outcomes & Retrospective

The experiment confirms precipitation scaling as a consequential and familiar
legacy-WEPP calibration lever. Mica Creek nearly closes peak magnitude and
retained storage at `1.5`; Paradise's chronology error falls from 37 to 12
days; Niwot's worst chronology error falls from 46.5 to 19.5 days. Snowbird
improves less and remains loss/timing-limited after its effective input reaches
observed-peak parity.

The package does not provide final multipliers: Paradise and Snowbird retain
unresolved joint upper-boundary response; Mica has a boundary-selected
chronology/magnitude tradeoff; Niwot remains magnitude-low; and no
independent-validation set exists. The justified handoff is one bounded,
prospectively frozen EB-04W2
extension followed by the already planned EB-04X Harvard investigation.
