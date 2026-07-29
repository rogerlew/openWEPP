# CANOPY-CAL-05 Litter-source and Decomposition Readiness

Package ID:
`20260728-canopy-cal-05-litter-source-decomposition-readiness-001`

Status: `COMPLETE / READINESS PASS / SOURCE AUTHORITY SUCCESSOR REQUIRED`

Date opened: `2026-07-28`

Execution mode: `package-end-to-end`

Package type: scientific source-sufficiency adjudication,
calibration-readiness execution, identifiability analysis, and missing-physics
handoff.

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Purpose

Execute canopy roadmap Order 5 under ADR-0042 after the operator reported that
the Forest Service has no additional tissue-separated litter data. Determine
what the implemented native leaf-source and decomposition paths can support,
prove that suitable observations can be consumed deterministically, quantify
source/decay confounding, and decide whether missing recurring needle and
fine-woody inputs require a contract-and-implementation package.

No unavailable source is zero. No execution axis is a prior, bound,
observation, fitted value, or production recommendation.

## Implementation Intent

- `implementation`: verify native leaf-off transfer, runner rate projection,
  direct-runtime decomposition, downstream publication, and a real
  residue/depth consumer as distinct evidence claims;
- `calibration-readiness`: prove deterministic source-to-stock operators,
  objective reconstruction, sensitivity, recovery, boundary/failure handling,
  and retained equifinality;
- `empirical-source-assessment`: describe only admitted foliar, pooled
  nonfoliar, total flux, and organic-stock relationships on their source
  boundaries;
- `empirical-calibration`: prohibited because source composition and
  stock/pool comparability are insufficient.

## Authority and Dependencies

- ADR-0042;
- `SC-PLANT-001` and `SC-RESIDUE-001`;
- CAL-03 daily native source/residue research outputs and frozen protocol;
- the CAL-04/05 authority-admission objects and exact Harvard plot joins;
- CAL-04B's frozen accepted upstream ensemble and later-stage uncertainty;
- the operator's 2026-07-28 report that the Forest Service has no additional
  data.

The production contracts and predecessor packages are read-only.

## Included Scope

- Authenticate all predecessor identities and evidence roles.
- Use `DirectDayFrame::run_r5c_decomposition_phase` and residue partition for
  daily direct-runtime traces; separately retain runner projection and
  real-consumer evidence.
- Verify native leaf-off source consumption from retained real-consumer
  CAL-03 evidence.
- Freeze finite synthetic surface-litter-input and decay axes as
  `ASSUMED_FOR_EXECUTION`.
- Reconstruct every producer state independently from retained operands.
- Demonstrate synthetic recovery from a complete daily stock series.
- Demonstrate source/decay equifinality from the frozen finite-horizon
  terminal-stock experiment alone.
- Exercise zero, interior, boundary, non-finite, and negative cases.
- Compute admitted Harvard foliar, pooled-nonfoliar, total-flux, and
  organic-stock diagnostics without changing units or material boundaries.
- Publish an ADR-0042 readiness matrix, three-axis stage statuses,
  additional-data inventory, and missing-physics recommendation.

## Excluded Scope

- No fitted or preferred source vector, decay constant, carbon fraction, or
  dry-mass conversion.
- No inference that pooled nonfoliar material is fine wood, needle turnover,
  bark, or reproductive litter in any chosen proportion.
- No comparison of model dry mass with observed carbon stock.
- No use of organic-horizon carbon as though it were exactly the modeled
  surface-residue pool.
- No production code, contract, fixture, management, default, or parameter
  change.
- No downstream snow, frost, hydrology, or erosion result may select litter or
  decay assumptions.

## Declared Write Set

- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001/package.md`
- `docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001/artifacts/operator-assistance-log.md`
- `docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001/artifacts/authority-gap-ledger.csv`
- `docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001/artifacts/final-disposition.md`
- this package subtree

All production code, canonical contracts, tests, protected fixtures, source
objects, and predecessor result artifacts are read-only.

## Frozen Prospective Design

The execution design is binding only after two independent prospective
scientific reviews pass.

- Producer: package-local Rust executable calling
  `DirectDayFrame::run_r5c_decomposition_phase`, then residue partition and
  downstream publication.
- Production projection proof: focused execution of the existing runner test
  `cqr_row7_growth_crop_and_surface_litter_projection_cover_schedule_paths`,
  plus exact source inspection of selected-crop rate projection. The
  package-local sweep is direct-runtime readiness, not a new end-to-end runner.
- Native source/consumer proof: exact hashes and rows from CAL-03's retained
  daily trace/rebuild evidence.
- Reconstruction: independent Python implementation reading producer inputs,
  not producer aggregate states or scoring code.
- Synthetic chronology: 20 years of 365 days; annual source deposited on day
  280; environmental inputs are fixed and labeled
  `ASSUMED_FOR_EXECUTION`.
- Synthetic annual surface-litter-input axis: `0.10`, `0.20`, `0.30`,
  `0.40 kg m^-2 yr^-1`. This hypothetical aggregate direct operand is not an
  observed composition or source-sufficiency result.
- Surface-rate axis: `0.0`, `0.5/365.25`, `1.0/365.25`,
  `2.0/365.25 d^-1`.
- Full-daily recovery truth: source `0.20 kg m^-2 yr^-1`, rate
  `0.5/365.25 d^-1`.
- Daily-series objective: unweighted SSE in `kg^2 m^-4`; it is synthetic and
  never applied to admitted observations.
- Terminal-stock objective: absolute distance in `kg m^-2` after day 365 in
  year 20, used only to demonstrate finite-horizon equifinality.
- Analytic terminal ridge: the five source/rate pairs in
  `artifacts/terminal-stock-ridge-design.csv` must reproduce the frozen truth
  endpoint within `1e-12 kg m^-2`; all pairs are retained.
- Initial surface stock: `0.20 kg m^-2`.
- Daily forcing: `tmax=20 C`, `tmin=10 C`, precipitation `0.004 m`,
  water-stress fraction `1`; no action; perennial context. Temperature limits
  surface decay. Precipitation saturates the separately reported
  standing-water factor but does not control the current surface-stock decay.
- Complete input: active slot/crop `1/1`, one-based runtime day `1..=365`;
  selector, root/ground seeds, cover factor, depth conversion, root rate, and
  all action fractions are zero. Surface and ground state carry across days
  and years. The pulse enters on day 280 before same-day decay. Terminal stock
  is emitted post-decay on year 20 day 365.
- Residue partition inputs are all `ASSUMED_FOR_EXECUTION`:
  `standing_residue_kg_m2=0`, `flat_residue_offset_kg_m2=0`,
  `buried_residue_kg_m2=0`, `cover_fraction=0`, and
  `rescov_interrill_weight=0`.
- Stopping rule: enumerate the complete Cartesian design once; no refinement,
  widening, or preferred-vector selection.
- Recovery tolerance: exact producer truth must be a daily-SSE minimum within
  `1e-20`; tied minima are retained.
- Reconstruction tolerance: each finite state within `1e-12 kg m^-2`.

`K000` is a `DIRECT_KERNEL_ZERO_RATE_ASSUMPTION`, not the native-forest
projection for configured `oratea=0`, where seasonal litter invokes the
contract fallback. These values test machinery; they are not ecological
bounds or recommendations.

The native-source lane inventories all 37 accepted GSI members and later-stage
membership identities without selecting a preferred member. Native dry-mass
outputs remain separate from Harvard carbon. If authenticated retained inputs
cannot reconstruct complete member-level source traces, leaf-only source
sufficiency is `NOT_ASSESSED`, not replaced by the synthetic pulse.

## Observation Operator and Claim Boundary

The Harvard plot join retains:

- mean foliar flux in `g C m^-2 yr^-1`;
- pooled nonfoliar flux in `g C m^-2 yr^-1`;
- total flux in `g C m^-2 yr^-1`; and
- organic-horizon stock in `kg C m^-2`.

CAL-05 may report the pooled nonfoliar share of observed total flux and the
source-native annual flux/stock ratio after the exact `g` to `kg` conversion.
That ratio is a descriptive derived diagnostic, not a fitted decay rate:
stock timing differs from flux timing, organic-horizon carbon is broader than
the model pool, and the nonfoliar material is pooled. Plot rows remain
separate; no invented uncertainty weight or aggregate fitting objective is
allowed. Each output preserves the 2000-2011 flux period, 2014 stock year,
plot, source units, `use.not=1`, positive replicate counts, finite nonnegative
values, positive stock, and `foliar + pooled_nonfoliar = total` within
`1e-7 g C m^-2 yr^-1`. This serialization tolerance was prospectively
amended after Incident 001 and remains over six times the maximum retained
`1.6167e-8` plot-mean reconstruction residual.

## Phase Plan

1. Freeze authority, assumptions, roles, units, design, objectives, failures,
   and stopping rules.
2. Obtain two prospective scientific reviews and disposition every finding.
3. Implement the producer, independent reconstructor, source/stock
   diagnostics, and validator.
4. Run native-path proof, synthetic design, dual reconstruction, recovery,
   sensitivity, boundary/failure, and Harvard descriptive diagnostics.
5. Determine the three ADR-0042 status fields for leaf source, recurring
   needle source, fine-woody source, total source, and decomposition.
6. Obtain two independent terminal reviews and two independent verifications,
   reconcile the exact diff, run applicable direct gates, and close Order 5.

## Required Deliverables

- `artifacts/required-reading-map.md`
- `artifacts/intent-plan.md`
- `artifacts/authority-and-assumption-ledger.csv`
- `artifacts/execution-control-contract.md`
- `artifacts/deterministic-design.csv`
- `artifacts/observation-operator-and-objective.md`
- two prospective scientific reviews
- `artifacts/prospective-finding-ledger.csv`
- package-local producer, reconstructor, diagnostics, and validator tools
- `artifacts/native-path-proof.csv`
- `artifacts/producer-results.csv`
- `artifacts/reconstruction-results.csv`
- `artifacts/synthetic-recovery.csv`
- `artifacts/equifinality-and-sensitivity.md`
- `artifacts/harvard-source-stock-diagnostics.csv`
- `artifacts/calibration-readiness-matrix.md`
- `artifacts/stage-status-ledger.csv`
- `artifacts/additional-data-inventory.csv`
- `artifacts/missing-physics-recommendation.md`
- two terminal scientific reviews
- `artifacts/finding-disposition.md`
- `artifacts/operator-governance-adjudication.md`
- two terminal verifications
- `artifacts/gate-evidence.md`
- `artifacts/exact-diff-reconciliation.md`
- `artifacts/final-disposition.md`

## Acceptance and Stop-loss

Readiness execution completes only if separately claimed runner projection,
native source/consumer, direct-runtime response, and downstream publication
evidence are truthful; reconstruction closes; daily synthetic truth is
recoverable; the analytic terminal ridge is retained; invalid cases fail
closed; and observation diagnostics preserve their boundaries. Package
completion does not preassign or upgrade stage statuses.

Stop result-bearing work for a broken parameter path, unexplained
reconstruction mismatch, unrecoverable synthetic truth, role leakage,
post-result design mutation, or a production/contract change. Missing
empirical data alone is not a stop condition under ADR-0042.

Status rules are frozen prospectively:

- native leaf transfer may be `IMPLEMENTED`; its other statuses depend on the
  retained native-ensemble analysis;
- recurring needle and fine-woody stages begin with the exact orthogonal
  triple `science_implementation_status=AUTHORITY_MISSING`,
  `calibration_evidence_status=NOT_CALIBRATION_READY`, and
  `identifiability_status=NOT_ASSESSED`;
- source composition and empirical decomposition remain
  `NOT_CALIBRATION_READY` or `NOT_ASSESSED` while authority gaps survive;
- `CALIBRATION_READY_DATA_LIMITED` is allowed only for a named implemented
  parameter/operator whose complete ADR-0042 readiness rows pass.

CAL-05 must recommend a bounded contract-first successor if an applicable
configuration needs recurring needle or fine-woody material, no authoritative
law exists, or an authoritative law is not implemented. The recommendation
separates observed class, configuration need, science authority, and
implementation. This package cannot authorize that successor.

## Review and Delegation Authorization

This package explicitly authorizes delegation to two independent prospective
scientific reviewers, two independent terminal scientific reviewers, and two
independent terminal verifiers. Each may write only its assigned
package-local review or verification artifact. Reviewers must challenge
material/stock comparability, execution-assumption labeling, real-path
evidence, recovery, equifinality, and the missing-physics recommendation.

## Progress

- [x] (2026-07-28) Recorded the operator response and opened this ADR-0042
  readiness package.
- [x] (2026-07-28) Froze the prospective execution design and claim boundary.
- [x] (2026-07-28) Completed dual prospective review, including independently
  reviewed Incident 001 tolerance amendment.
- [x] (2026-07-28) Executed the designs, reconstruction, recovery, sensitivity,
  covariance, saturation, boundary, failure, and Harvard diagnostics.
- [x] (2026-07-28) Completed dual terminal review and verification, direct
  gates, diff reconciliation, prompt archival, and final `HOLD` disposition;
  corrected runtime/validator findings while retaining the Incident 002
  governance finding.
- [x] (2026-07-28) Recorded explicit operator authorization for the
  retrospective analysis, lifted the sole Incident 002 governance hold, and
  refreshed lifecycle evidence without changing any scientific input,
  objective, result, or source-authority boundary.

## Decision Log

- Decision: Proceed without further Forest Service data with ADR-0042
  readiness execution, without preassigning a scientific result.
  Rationale: ADR-0042 separates implementation and readiness from calibration
  evidence sufficiency.
  Date/Author: 2026-07-28 / Codex.
- Decision: Treat recurring needle and fine-woody inputs as separate missing
  production-source questions.
  Rationale: pooled nonfoliar observations cannot identify either component,
  and current CAL-03 research output explicitly reports both as null.
  Date/Author: 2026-07-28 / Codex.
- Decision: Amend the Harvard plot-mean arithmetic closure tolerance from
  `1e-9` to `1e-7 g C m^-2 yr^-1` before rerunning the aborted diagnostic.
  Rationale: the retained mean table has a maximum floating serialization
  residual of `1.6167e-8`; the original guard rejected 24/28 rows before
  publishing output. No value, class, unit, objective, or science criterion
  changes.
  Date/Author: 2026-07-28 / Codex.
- Decision: Accept the retrospectively selected Incident 002 sensitivity,
  covariance, and saturation summaries as readiness evidence for the narrowly
  named direct-runtime surface source/rate operator.
  Rationale: the operator explicitly authorized the retrospective analysis;
  it uses only the frozen exhaustive grid and ridge and changes no axis,
  observation, objective, tolerance, result, stopping rule, or parameter
  selection. The analysis remains labeled retrospective and cannot support
  empirical calibration or missing-source claims.
  Date/Author: 2026-07-28 / Operator and Codex.

## Outcomes and Retrospective

CAL-05 raw execution and reconstruction pass for the narrowly named
direct-runtime surface source/rate operator. Complete daily stock recovers the
synthetic truth; one finite-horizon stock is nonidentifying across an exact
five-pair source/rate ridge.

Incident 002 selected the local-sensitivity stencil after results, contrary to
this package's stop-loss. The operator explicitly authorized that
retrospective analysis on 2026-07-28. Because it uses only the already frozen
exhaustive grid and ridge without changing inputs, objectives, results, or
selection, the sole governance hold is lifted. The named direct-runtime
surface source/rate operator is
`IMPLEMENTED / CALIBRATION_READY_DATA_LIMITED / PARTIALLY_IDENTIFIABLE`.

The source system does not pass empirical calibration. Native leaf source
sufficiency is `NOT_ASSESSED`; recurring needle and fine-woody source
authority is missing; and empirical decomposition fitting remains
`NOT_CALIBRATION_READY`. Harvard pooled nonfoliar material cannot identify a
component or map organic-horizon carbon to modeled dry mass.

Order 5 readiness execution is complete. At CAL-05 closure, bounded
contract-first `CANOPY-LITTER-SOURCE-AUTHORITY-01` remained required before
Orders 6-8; that successor has since implemented the authenticated external
boundary while retaining the predictive authority hold.

## Post-closure Visualization Supplement

On 2026-07-28 the operator requested graph artifacts for human
interpretation. The deterministic standard-library renderer
`tools/plot_results.py` now publishes three accessible SVG figures from the
frozen result tables:

- terminal-stock response across the source/rate grid;
- complete-daily-series recovery across all 16 candidates; and
- the five-pair source/decay ridge with its distinct annual trajectories.

The figures are indexed in `artifacts/figures/README.md`. They add no result,
axis, observation, parameter selection, or status change and retain every
`ASSUMED_FOR_EXECUTION` and predictive-authority limitation.
