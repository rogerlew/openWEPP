# CANOPY-CAL-04A Best-Available Evidence and Daymet Forcing

Package ID: `20260726-canopy-cal-04a-best-available-evidence-daymet-001`

Status: `COMPLETE / DESIGN_AUTHORITY_ADMITTED`

Date opened: `2026-07-26`

Execution mode: `package-end-to-end`

Package type: prospective calibration authority, external forcing acquisition,
derived meteorology, and data-limited identifiability analysis.

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Purpose

Lift the actionable authority boundary recorded by CANOPY-CAL-04 without
pretending that data-limited canopy operands are uniquely identifiable.
Acquire checksum-bound Daymet forcing for the admitted Hubbard Brook phenology
plots, calculate VPD using the exact native-runner equation, quantify
temperature, VPD, photoperiod, elevation, and covariance support around the
observed P3 intervals, compare Daymet with the protected fixture forcing, and
freeze a best-available-evidence calibration design with explicit caveats.

This package establishes prospective design authority. It does not fit canopy
parameters, open Harvard modeled holdout results, select from downstream
hydrology or erosion, or alter production physics.

## Progress

- [x] (2026-07-26) Confirmed user direction to adopt a truthful
  best-available-evidence strategy for inherently data-limited calibration.
- [x] (2026-07-26) Identified nine admitted Hubbard phenology plot coordinates
  and elevations and the protected fixture/plot elevation mismatch.
- [x] (2026-07-27) Authenticated required reading, inputs, exact tools, and
  terminal plan.
- [x] (2026-07-27) Acquired and checksummed Daymet daily forcing and source
  metadata.
- [x] (2026-07-27) Reproduced native VPD and photoperiod equations.
- [x] (2026-07-27) Joined forcing and elevation to 932 admitted
  interval-censored P3 observations.
- [x] (2026-07-27) Quantified support, correlation, anomaly leverage, and
  parameter
  confounding.
- [x] (2026-07-27) Froze finite search domains, saturation classes, evidence
  tiers,
  assumptions, deterministic design, and no-refinement stopping rule.
- [x] (2026-07-27) Completed dual review, finding disposition, gates, dual
  verification, prompt archival, and final disposition.

## Scientific Posture

The objective is not to prove unique identification. Every operand receives
one evidence class:

- `DATA_CONSTRAINED`;
- `COMBINATION_CONSTRAINED`;
- `LITERATURE_BOUNDED`;
- `FORCING_SUPPORT_BOUNDED`;
- `PHYSICALLY_BOUNDED`;
- `ASSUMED_FOR_EXECUTION`;
- `NONIDENTIFIABLE`.

Assumptions are permitted only when explicit, prospectively frozen, sensitivity
tested in the later calibration package, and never mislabeled as observations
or probability priors.

## Included Scope

- Acquire public Daymet Version 4 daily single-pixel data and metadata for the
  nine admitted Hubbard Brook phenology plots for the observation period.
- Retain source-native bytes, request identities, retrieval timestamps, terms,
  checksums, and Daymet-returned coordinates/elevation.
- Derive:
  `VPD = 0.5 * (es(Tmax) + es(Tmin)) - VP`,
  where `es(T)=0.6108*exp(17.27*T/(T+237.3))` kPa and Daymet `VP` is converted
  from Pa to kPa.
- Fail rather than clamp a negative or non-finite derived VPD.
- Calculate native FAO-56 photoperiod with the exact production kernel formula.
- Parse protected `.cli` `Tmax`, `Tmin`, and dewpoint and derive native VPD for
  comparison only.
- Join Daymet forcing to Hubbard P3 observation intervals by exact plot/year.
- Analyze elevation gradients, forcing support, correlations, rolling 21-day
  features, between-year anomaly leverage, and threshold-family confounding.
- Propose finite deterministic search domains and explicit outside-support
  saturation classes without looking at model calibration scores.
- Retain analysis tools and result tables under this package.

## Excluded Scope

- No parameter candidate or production-runner calibration execution.
- No Harvard phenology data join, modeled trace, score, or domain influence.
- No downstream snow, ET, interception, runoff, frost, or erosion influence.
- No production Rust, science-contract, protected fixture, admitted source, or
  prior-package artifact edit.
- No use of current native or Bill Elliot parameter values as bounds, centers,
  probability priors, or acceptance targets.
- No claim that Daymet resolves microclimate, species effects, all six GSI
  thresholds, or individual magnitude operands uniquely.

## Declared Write Set

- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260726-canopy-cal-04a-best-available-evidence-daymet-001/**`
- `references/canopy_phenology/daymet_calibration/**`

All other paths are read-only.

## Inputs and Authority

- CANOPY-CAL-04 final hold disposition and exact terminal evidence;
- CANOPY-CAL-04/05 admitted timing windows and frozen operator;
- CAL-03 frozen calibration/holdout ledger and protocol;
- Hubbard EDI 51.16 source CSV and EML plot geometry;
- Daymet Version 4 daily data documentation and returned source metadata;
- `SC-PLANT-001` CP-GSI02;
- exact native runner VPD and photoperiod implementations;
- protected Hubbard native member and climate, comparison-only.

## Required Deliverables

- `artifacts/required-reading-map.md`
- `artifacts/intent-plan.md`
- `artifacts/source-and-request-manifest.csv`
- `artifacts/hubbard-plot-geometry.csv`
- `artifacts/daymet-daily-derived.csv`
- `artifacts/fixture-daily-derived.csv`
- `artifacts/phenology-forcing-join.csv`
- `artifacts/forcing-support-summary.csv`
- `artifacts/correlation-and-confounding.md`
- `artifacts/elevation-analysis.md`
- `artifacts/vpd-and-photoperiod-method.md`
- `artifacts/best-available-evidence-ledger.csv`
- `artifacts/proposed-calibration-design.md`
- `artifacts/limitations-and-caveats.md`
- exact command and execution inventories
- two independent scientific reviews and finding disposition
- gate evidence and two independent terminal verifications
- `artifacts/final-disposition.md`

## Plan

### Phase 1: prospective intake

Freeze plot mapping, Daymet product/version, request construction, years,
variables, calendars, native-equation derivations, comparison roles, joining
rules, missing/error handling, analysis methods, and Harvard embargo before
affected analysis.

### Phase 2: acquisition and deterministic derivation

Retrieve daily `tmin`, `tmax`, `vp`, and `dayl` for all nine plots. Preserve
source bytes and metadata, checksum every object, and rebuild one normalized
daily table. Derive VPD and native photoperiod without clamping.

Parse the protected Hubbard `.cli` separately. The fixture comparison may
diagnose forcing mismatch but cannot select calibration parameters or rewrite
the protected member.

### Phase 3: observation-aligned analysis

Join only Hubbard calibration intervals. Quantify forcing immediately before
and through each observed P3 bracket, 21-day histories, year anomalies,
plot/elevation gradients, cross-variable correlation, and practical
confounding. Preserve weekly interval censoring; do not invent observation
midpoints as truth.

### Phase 4: calibration-design authority

Define finite forcing-supported domains, outside-support saturation classes,
exact evidence tiers, assumptions, candidate enumeration, refinement,
stopping, and boundary-hit rules. Domains must be model-result-blind. Where
data cannot separate parameters, retain an ensemble axis or classify an
assumption rather than selecting a convenient value.

### Phase 5: closure

Run deterministic rebuild and join validators, checksum verification,
equation cross-checks, role/embargo guards, documentation lint, diff hygiene,
write-set reconciliation, dual review, finding disposition, dual verification,
and final disposition.

## Acceptance

The package may close `COMPLETE / DESIGN_AUTHORITY_ADMITTED` only if:

- every Daymet source and derived result is reproducible and checksum-bound;
- plot coordinates/elevations and observation joins are exact;
- VPD and photoperiod reproduce native equations within declared tolerance;
- Daymet and protected-fixture forcing roles remain distinct;
- finite domains follow prospectively declared evidence rules;
- correlations, confounding, scale mismatch, gridded-data limitations, and
  data gaps are explicit;
- every operand has a truthful evidence class and later sensitivity obligation;
- Harvard and downstream evidence remain uncontaminated;
- dual review and dual verification pass.

A negative result may still complete if it truthfully proves that some
thresholds remain nonidentifiable while producing a reproducible bounded
ensemble design. Missing or irreconcilable source identity, invalid VPD,
broken plot mapping, or retrospective domain selection forces `HOLD`.

## Review and Delegation Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two prospective or terminal scientific reviewers and two
terminal verifiers for Daymet provenance, meteorological derivation,
observation alignment, calibration-domain legitimacy, caveats, and closure;
expected outputs are review and verification artifacts; write access is
read-only.

## Minimum Gates

```text
sha256sum -c references/canopy_phenology/daymet_calibration/SHA256SUMS
.venv/bin/python <package>/tools/validate.py
markdown-doc lint --path <package>
git diff --check
```

No Rust correctness campaign is selected because production Rust is read-only.
