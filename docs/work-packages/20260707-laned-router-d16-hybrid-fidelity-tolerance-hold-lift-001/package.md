# LANED Router D16 Hybrid Fidelity-Tolerance Hold Lift

Status: EXECUTED-HOLD-COHORT-AUTHORITY

## Objective

Attempt to lift the D16 hybrid default-promotion fidelity hold by defining the
production-facing tolerance surfaces needed for
`SC-OFEROUTE-002#INV-OFEHYB-008`, then running enough current-mesh active
plain-vs-hybrid evidence to ratify or reject those tolerances before any
selector flip.

If the required cohort or authority is unavailable, close as
`EXECUTED-HOLD-*` with no partial promotion.

Execution result: the package held before contract/code changes because no
broad active-runnable cohort exists in the repo/session. The owcmp cohorts are
inventory-only manifests, and available runfile fixtures/external run roots do
not carry source-authorized `routing_coefficients` for the active Lane-D
selector.

## Rationale

D16 established that the current hybrid is faster and Case-4 compliant, but
not default-promotable from H2637 alone: active plain-vs-hybrid H2637 output
movement is material (`-0.4396 %` routed outlet, `-6.474 %` pass sediment
sums), and no ratified production tolerance covers that movement.

This hold-lift package executes the first follow-on from D16:

- predeclare the tolerance surfaces that a default-promotion package would need,
- audit whether the repo/session has a broad active-runnable cohort,
- run feasible evidence,
- either ratify/implement promotion or record the exact hold boundary.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `tests/AGENTS.md`
- `tools/owcmp/AGENTS.md`
- `tools/owcmp/README.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/artifacts/hold-legitimacy-audit.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/artifacts/timing-and-fidelity.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-disposition.md`
- `tools/owcmp/suites/*.json`

## Scope

Included:

- Current-mesh active plain-vs-hybrid tolerance-surface design for default
  promotion.
- Audit of available active-runnable cohort inputs and owcmp suite posture.
- Feasible preflight or comparator execution using existing artifacts and
  exact binary provenance where evidence invokes the runner.
- Contract amendment and selector flip only if the tolerance evidence is
  broad enough and authority-backed.

Excluded:

- Tier-2 mesh-resolution changes.
- Generic non-bare implicit solve optimization.
- Creating surrogate routing coefficients or approximating missing active
  operands.
- Broader no-env Lane-D active-owner default activation.
- Post-hoc ratification of H2637-only deltas.

## Acceptance Criteria

Promotion can proceed only if all current-scope gates pass:

- Tolerance surfaces are declared before accepting any new deltas.
- A cohort exists that includes at minimum H2637 plus contrasting dry,
  low-runoff, high-runoff, steep, and multi-event hillslopes.
- Every cohort member can run both active plain and active explicit hybrid
  with source-authorized `routing_coefficients` and current active Lane-D
  preconditions.
- The cohort compares HBP/pass/water publication surfaces, routed outlet
  magnitude, tail fold, end-window storage, clamp residual classes, active
  closure residuals, and timing/counters.
- Any tolerance thresholds are ratified in `SC-OFEROUTE-002` before a selector
  default flip.
- No selector/default flip occurs unless all evidence passes.

If any item cannot be proven, close as `EXECUTED-HOLD-*`.

## Intended Write Set

- This package directory.
- `docs/work-packages/README.md`.
- `SC-OFEROUTE-002` and selector code only if promotion is fully ratified.

## Phase Plan

1. **S0 Scaffold and authority read.** Create package, register it, and map
   current D16 blocker authority.
2. **S1 Tolerance-surface predeclaration.** Record non-binding candidate
   surfaces before any new run acceptance.
3. **S2 Cohort availability audit.** Inspect repo fixtures, owcmp manifests,
   and external run roots for active-runnable inputs.
4. **S3 Feasible evidence.** Run current preflights and any safe comparator
   checks available in the session.
5. **S4 Decision.** If cohort/tolerance authority is sufficient, amend
   contract and implement promotion. Otherwise close with a hold audit.
6. **S5 Review, verification, and gates.** Complete dual review, dual
   verification, gate table, line-count governance, final disposition, and
   handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, `verification_runner`, and `explorer` subagents for cohort
inventory checks, comparator/timing verification, contract/code review, and
bounded codebase questions. Expected outputs are package-local
`artifacts/review-*.md`, `artifacts/verification-*.md`, and comparator/timing
artifacts. Write access is read-only unless a worker is explicitly assigned a
bounded implementation fix inside the intended write set.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/tolerance-surface-design.md`
- `artifacts/cohort-availability-audit.md`
- `artifacts/feasible-evidence.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- `tools/owcmp/owcmp env --manifest` for each referenced owcmp cohort.
- Active-runnable cohort preflight proving routing-coefficient authority.
- H2637 D16 evidence reuse with exact binary provenance.
- New H2637 or cohort reruns if contract/code changes.
- Contract/profile/BEI checks if `SC-*` contracts are touched.
- Protected-output byte identity if selector/default surfaces are touched.
- Focused Lane-D / `ofe_routing` tests if code/contracts are touched.
- `cargo fmt --check`
- Full Rust closure gates only if Rust/contracts/fixtures are changed.
- `.rs` line-count governance.

## Closure Outcomes

- `EXECUTED-COMPLETE-PROMOTION-RATIFIED`: tolerances are ratified, selector
  promotion lands, and all gates pass.
- `EXECUTED-HOLD-COHORT-AUTHORITY`: the active-runnable cohort/comparator
  authority needed to ratify tolerances is unavailable.
- `EXECUTED-HOLD-FIDELITY-TOLERANCE`: a runnable cohort exists but rejects the
  proposed tolerance envelope.
