# LANED Router D16 Hybrid Cohort Authority Hold Lift

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY

## Objective

Execute the first follow-on from the D16 hybrid fidelity-tolerance hold lift:
construct an executable active plain-vs-hybrid promotion cohort with
source-authorized Lane-D routing coefficients, then return to D16 default
promotion only if the cohort exists and passes.

If the cohort cannot be constructed inside this package without inventing
route-coefficient authority, close as `EXECUTED-HOLD-*` with exact evidence and
no selector, contract, suite, or fixture posture change.

Execution result: held before implementation. The available owcmp manifests are
inventory-only declarations, the selected external cohort roots contain no
native `ow-lanuse-1` managements and no `routing_coefficients` extension, and
current authority forbids inferring Lane-D routing coefficients from legacy
cropland fields.

## Rationale

Package
`20260707-laned-router-d16-hybrid-fidelity-tolerance-hold-lift-001` proved the
hybrid cannot be default-promoted from H2637 alone. It then named the next
actionable blocker: build `D16-HYB-COHORT-AUTHORITY` by selecting cohort members
from the owcmp inventories plus H2637, producing source-authorized active
Lane-D run inputs, and adding an executable active plain-vs-hybrid comparator
suite.

This package executes that action until the first authority boundary. It is not
allowed to:

- synthesize `routing_coefficients` from row/ridge/random-roughness fields,
- treat compatibility cropland as native Lane-D operand authority,
- promote H2637-only timing/fidelity evidence into a cohort tolerance,
- or create an executable suite that silently relies on surrogate coefficients.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `tools/owcmp/AGENTS.md`
- `tools/owcmp/specification.md`
- `tools/owcmp/suites/*.json`
- `docs/work-packages/20260707-laned-router-d16-hybrid-fidelity-tolerance-hold-lift-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-fidelity-tolerance-hold-lift-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-fidelity-tolerance-hold-lift-001/artifacts/worker-handoff.md`

## Scope

Included:

- Select the minimum candidate cohort: H2637 plus the three current owcmp
  inventory roots.
- Audit whether those members can become active plain-vs-hybrid openWEPP
  runners under existing source authority.
- Run current owcmp environment and manifest-run preflights.
- Run focused active fail-closed evidence for missing routing coefficients.
- Add an executable comparator/timing suite only if source-authorized active
  inputs exist inside the current authority envelope.

Excluded:

- Default selector promotion.
- Contract ratification of fidelity tolerances.
- Tier-2 mesh-resolution policy.
- Generic hybrid solve optimization.
- Legacy-field to routing-coefficient bridge design.
- Operator-authored external management sidecars not present in this session.

## Acceptance Criteria

This package may lift the cohort-authority hold only if all items pass:

- At least H2637 plus contrasting external cohort members have active-runnable
  openWEPP inputs.
- Every active input carries complete, schedule-consistent native
  `routing_coefficients` for each scheduled Lane-D landuse.
- Coefficient authority is from native management input or a contract-approved
  bridge, not from package-local inference.
- `tools/owcmp/owcmp manifest run` has an executable active plain-vs-hybrid
  suite, not an inventory-only manifest.
- Active plain and active explicit hybrid run for every cohort member and
  produce the surfaces needed by the prior tolerance-surface design.

If any item cannot be proven, close as hold and name the first follow-on.

## Intended Write Set

- This package directory.
- `docs/work-packages/README.md`.
- `tools/owcmp/suites/` and comparator code only if an executable suite can be
  built from source-authorized active inputs.
- No Rust kernel/runtime code unless a current-scope defect is found while
  producing the cohort.

## Phase Plan

1. **S0 Scaffold and authority map.** Create the package and record the exact
   contract/process authority governing route-coefficient inputs.
2. **S1 Cohort selection.** Select H2637 plus existing owcmp inventory roots.
3. **S2 Source-authority audit.** Check selected roots for native datver,
   `routing_coefficients`, openWEPP runfiles, and executable suite posture.
4. **S3 Feasible evidence.** Run owcmp env/manifest preflights and focused
   active fail-closed evidence.
5. **S4 Implementation decision.** Build the suite only if source authority is
   present. Otherwise record the hold.
6. **S5 Review, verification, and gates.** Complete review, verification, gate
   table, line-count governance, final disposition, and handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to `rust_code_reviewer`, `rust_qa_reviewer`,
`comparator_suite_runner`, `verification_runner`, and `explorer` subagents for
route-coefficient authority review, owcmp/cohort verification, package gate
review, and bounded codebase questions. Expected outputs are package-local
`artifacts/review-*.md`, `artifacts/verification-*.md`, and compact comparator
or command evidence. Write access is read-only unless a worker is explicitly
assigned a bounded implementation fix inside the intended write set.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/cohort-member-selection.md`
- `artifacts/route-coefficient-authority-audit.md`
- `artifacts/owcmp-preflight.md`
- `artifacts/active-input-preflight.md`
- `artifacts/command-evidence.md`
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
- `tools/owcmp/owcmp manifest run --manifest` for each referenced owcmp cohort
  or explicit hold evidence if the manifest is inventory-only.
- Active-runnable input preflight proving native route-coefficient authority.
- Focused active fail-closed regression for missing `routing_coefficients`.
- Contract/profile/BEI checks if `SC-*` contracts are touched.
- Focused Lane-D / `ofe_routing` tests if code/contracts are touched.
- Anti-evasion guards if required-case bindings, cohort fixtures, or external
  authority suite posture are touched.
- `cargo fmt --check`
- `.rs` line-count governance.
- Full Rust closure loop only if Rust code, contracts, fixtures, or suite
  posture are changed.

## Closure Outcomes

- `EXECUTED-COMPLETE-COHORT-AUTHORITY`: source-authorized active cohort exists
  and executable suite runs.
- `EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY`: selected cohort inputs lack
  source-authorized `routing_coefficients` and no approved authoring/bridge
  path exists.
- `EXECUTED-HOLD-SUITE-IMPLEMENTATION`: source-authorized active inputs exist,
  but executable suite implementation cannot safely close in-envelope.
