# SNOW-SURFACE-EB-04W2 Bounded Precipitation-Scaling Grid Extension

Status: `complete`

Date: `2026-08-02`

Campaign: `SNOW-SURFACE-EB`

Plan class: `Empirical calibration bracketing + compensation adjudication`

This living ExecPlan follows `docs/codex_exec_plans.md` and must remain
self-contained as execution proceeds.

## Purpose / Big Picture

EB-04W1 proved that uniform total-precipitation scaling materially improves
snow magnitude and chronology, but its `1.5` ceiling did not bracket Paradise
or Snowbird, Niwot magnitude was still rising, and Mica Creek exposed a
magnitude-versus-chronology tradeoff. EB-04W2 performs the one bounded extension
authorized by that result. A reader will be able to see where peak magnitude
crosses observed parity, whether timing improves with it, and whether apparent
fit requires implausibly large input that the model then loses before peak.

This is a calibration experiment using the same SNOTEL records already used in
EB-04W1. It is not independent validation, a transferable regional correction,
a production coefficient, or a snow-process promotion study.

## Objective

Combine EB-04W1's immutable `1.0-1.5` anchor cells with new release-binary runs
at `1.6, 1.7, 1.8, 1.9, 2.0`; reconstruct the inherited magnitude, chronology,
effective-input, retained-storage, loss, and conservation operators; identify
the magnitude bracket and chronology tradeoff per lane; flag compensation; and
close the precipitation-scaling branch at the frozen `2.0` experiment budget.

## Implementation Intent

Intent: `empirical-calibration + calibration-readiness`.

`science_implementation_status` begins `IMPLEMENTED`: authoritative phase and
CoE snow physics remain unchanged. `calibration_evidence_status` begins
`CALIBRATION_READY_DATA_LIMITED`, and `identifiability_status` begins
`PARTIALLY_IDENTIFIABLE`. All four SNOTEL records remain `CALIBRATION`; zero
records are independent validation.

The multiplier is dimensionless and applies only to daily CLIGEN total
precipitation depth in package-local fixture copies. Grid values and reporting
thresholds are `ASSUMED_FOR_EXECUTION`, not physical bounds, probability priors,
regional defaults, or calibrated values.

## Prospective Experiment Freeze

The full analysis surface is `1.0-2.0` at increments of `0.1`. EB-04W1's exact
`1.0-1.5` results and raw outputs are retained anchors and must pass identity
checks; they are not rerun. EB-04W2 executes exactly 20 new cells: four lanes
times `1.6-2.0`.

Daily precipitation depth alone is scaled. Event duration, peak-time fraction,
dimensionless intensity shape, temperatures, radiation, wind, dew-point
fields, management, slope, soil, snow parameters, observations, and operators
remain protected.

Magnitude is the median modeled/observed seasonal peak-SWE ratio. Chronology is
the inherited absolute median peak-date or melt-out-date offset; Niwot retains
the worse of its peak-depth and peak-SWE offsets. Effective input is initial
SWE plus realized snowfall SWE plus retained rain through the observed SWE peak.
Retained storage is modeled SWE on that date.

Candidate selection is result-blind and lexicographic:

1. A candidate must have peak ratio in `[0.9, 1.1]` and strictly improve
   chronology over `1.0` by more than `1e-12` day.
2. Rank eligible candidates by absolute log peak-ratio error, then chronology
   error, then distance from `1.0`.
3. Separately report the magnitude-best and chronology-best candidates and the
   adjacent grid cells that bracket peak parity.
4. Flag `COMPENSATION_WARNING` when selected effective input exceeds `1.25`
   times observed peak SWE while retained storage remains below `0.8`. These
   are operational reporting thresholds, `ASSUMED_FOR_EXECUTION`.
5. Classify a magnitude/chronology disagreement as `TRADEOFF_BRACKETED`; an
   eligible compensation-flagged selection as
   `BRACKETED_WITH_COMPENSATION_WARNING`; and an ordinary eligible selection as
   `BRACKETED_CANDIDATE`.
6. If no eligible selection exists or the selected/magnitude-best candidate is
   `2.0`, classify `EXPERIMENT_BUDGET_BOUNDARY`. Do not extend the grid again.

The `2.0` ceiling is an experiment-budget stop, not a physical precipitation
bound. At that boundary, unresolved error transfers to the existing
forcing/process-identifiability limitation and EB-04X proceeds; EB-04W3 is not
authorized by this package.

## Included Scope

- immutable EB-04W1 freeze, receipt, results, raw anchor outputs, observations,
  lanes, operators, and release runner;
- package-local precipitation-only transformation at `1.6-2.0`;
- exactly 20 new release runs and an exact 44-cell combined analysis surface;
- independent anchor, transformation, inventory, and closure reconstruction;
- magnitude bracketing, chronology tradeoff, compensation, and stop-loss
  adjudication;
- accessible SVG figures with same-stem Markdown sidecars;
- calibration-readiness, dual review/disposition, dual verification, and
  roadmap/catalog handoff.

## Excluded Scope

- production Rust, science contracts, tests, source fixtures, observations,
  public schemas, defaults, selectors, or assurance authority;
- scaling any climate field other than daily total precipitation;
- phase, melt, density, sublimation, longwave, interception, liquid-routing, or
  canopy-physics changes;
- independent validation, transferability, regionalization, production
  activation, parameter covariance estimation, or promotion;
- another precipitation grid extension after `2.0`.

## Intended Write Set

- this package tree;
- `docs/ROADMAP.md`, `docs/planning/snow-surface-energy-balance-roadmap.md`,
  and `docs/work-packages/README.md`;
- package-local execution/analysis tooling;
- ignored runtime outputs under
  `target/snow_surface_eb04w2_precipitation_scaling/`.

All production, contract, test, source-data, assurance, and historical-package
paths are protected.

## Phase Plan

### Phase A — Scaffold and freeze

Hash-bind EB-04W1, the exact retained/new grids, data roles, protected fields,
operators, candidate rule, compensation flag, and `2.0` stop-loss before new
model output.

### Phase B — Implement and self-test

Create a package-local runner that reuses EB-04W1's audited transformation and
real-run mechanisms, executes only extension cells, verifies every transformed
token, and combines retained/new cells without rewriting predecessor evidence.

### Phase C — Execute the extension

Run exactly 20 new cells with sanitized `OPENWEPP_*` state and the frozen
release binary. Any partial inventory is `INVALID` until completed.

### Phase D — Analyze and adjudicate

Reconstruct all 44 cells, apply the frozen ranking and compensation rules,
render figures/sidecars, and decide whether precipitation scaling is bracketed
or stopped at the experiment budget for each lane.

### Phase E — Validate, review, and close

Run scoped package/tool, identity, provenance, conservation, figure,
documentation, security, and exact-diff checks. Complete dual independent
reviews, finding disposition, dual verification, prompt archival, and roadmap
handoff.

## Acceptance Criteria

1. Freeze predates every `1.6-2.0` run and binds the tool, binary, predecessor,
   grids, operators, roles, ranking, flags, and stop-loss.
2. Exactly 20 unique extension runs complete with return code zero.
3. All 24 retained EB-04W1 anchor lane/multiplier cells pass committed evidence
   and raw-output identity checks.
4. Only daily precipitation differs in extension fixtures, within rendering
   roundoff; every protected token and file remains identical.
5. The combined analysis contains exactly 44 unique cells and applies the
   frozen rule without result-aware changes.
6. All inherited mass, phase, accumulation, melt-component, and trace/WAT
   closures remain within `1e-12 m`.
7. Magnitude-best, chronology-best, parity bracket, compensation warning, and
   experiment-budget outcomes are explicit for every lane.
8. Every figure parses, is visually inspected, and has an accessible Markdown
   sidecar explaining population, units, processing, uncertainty, and limits.
9. No protected production, contract, fixture, observation, selector, default,
   assurance, or historical evidence path changes.
10. All selected validation, review, disposition, verification, prompt, and
    terminal exact-diff gates pass.

Any unmet current-scope criterion forces `HOLD`; it cannot be reclassified as
future work after execution starts.

## Validation Selection

Risk is `Moderate / analysis-only forcing calibration`. Required gates are the
transformer self-test, source/scaled token audit, exact retained/new inventory
and identities, combined-cell uniqueness, closure reconstruction, result-rule
reconstruction, SVG parse/visual inspection, Markdown lint, Python compilation,
security impact, exact diff, dual review/disposition, and dual verification.
Workspace Rust suites are not selected because the intended diff cannot touch
Rust, contracts, manifests, tests, or production inputs; terminal reconciliation
must escalate if that assumption changes.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent science/QA reviewers and two terminal
verifiers. Expected outputs are compact package-local review/verification
Markdown artifacts; each role has write access only to its named artifact. No
heavy workspace suite or comparator batch is selected.

## Progress

- [x] (2026-08-02) User directed EB-04W2 scaffold and end-to-end execution.
- [x] (2026-08-02) Prospectively specified retained/new grids, magnitude-first
  selection, compensation flag, and hard `2.0` experiment-budget stop.
- [x] (2026-08-02) Scaffolded package tree, roadmap/catalog entry, self-tested
  tool, 20-cell transformation preflight, and hash-bound machine freeze before
  new model output.
- [x] (2026-08-02) Executed 20/20 new cells, verified 24 retained anchors and
  144 retained output identities, and analyzed the exact 44-cell surface.
- [x] (2026-08-02) Produced four figures/sidecars plus scientific,
  calibration-readiness, operand-lineage, transformation, and handoff evidence.
- [x] (2026-08-02) Completed scoped validation, dual review/disposition, dual verification,
  prompt archival, and terminal handoff.

## Surprises & Discoveries

- Observation: EB-04W1's broad `[0.9, 1.1]` first tier selected Mica Creek
  `1.5` for one day of chronology improvement even though `1.4` was closer to
  peak parity.
  Evidence: EB-04W1 dual review and disposition.
- Observation: Paradise and Snowbird improved on both axes through `1.5`,
  while Niwot magnitude continued upward after chronology plateaued.
  Evidence: EB-04W1 response curves and scientific disposition.
- Observation: Paradise `1.8` simultaneously yields peak ratio `0.989` and
  zero-day chronology error.
  Evidence: combined results and seasonal/tradeoff figures.
- Observation: Snowbird `2.0` nearly resolves peak magnitude (`0.977`) but
  remains 23 days early.
  Evidence: combined results and scientific disposition.
- Observation: no selected cell enters the frozen compensation-warning
  quadrant, although Niwot `1.7` narrowly clears the storage threshold.
  Evidence: input/storage pathways and selection records.

## Decision Log

- Decision: reuse `1.0-1.5` anchors and execute only `1.6-2.0`.
  Rationale: immutable retained evidence avoids redundant result-bearing runs
  while preserving a contiguous response surface.
  Date/Author: 2026-08-02 / Codex.
- Decision: make peak-parity error the first ranking key and chronology a
  required improvement plus second key.
  Rationale: EB-04W1 revealed that treating the entire magnitude band as tied
  obscured Mica Creek's tradeoff. This new rule is frozen before W2 results and
  does not reinterpret W1's frozen selection.
  Date/Author: 2026-08-02 / Codex.
- Decision: stop at `2.0` regardless of boundary response.
  Rationale: W2 is the agreed bounded calibration extension; continuing again
  would become a forcing-compensation rabbit hole and delay EB-04X.
  Date/Author: 2026-08-02 / Codex.
- Decision: close the precipitation-forcing branch and return to EB-04X after
  review, without W3.
  Rationale: the exact experiment budget is exhausted; three lanes are
  bracketed and Snowbird's remaining 23-day timing deficit is not resolved by
  more forcing search inside this package.
  Date/Author: 2026-08-02 / Codex.

## Outcomes & Retrospective

EB-04W2 completed all 20 new release runs and integrated them with 24 immutable
anchors. Mica Creek, Niwot, and Paradise now have interior, site-specific
empirical calibration candidates under the frozen objective. Paradise is the
cleanest result because magnitude and chronology optima coincide. Mica and
Niwot retain genuine magnitude-versus-timing tradeoffs.

Snowbird reaches near-parity mass at `2.0`, but that is the final budget cell
and its chronology remains 23 days early. The forcing lever is therefore
scientifically useful but not a universal timing correction. The package
closes the precipitation branch without W3 and hands the campaign to EB-04X.
