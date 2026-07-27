# Gate Planner AUTH11 Terminal Node Selection

Package ID: `20260727-gate-planner-auth11-terminal-node-selection-001`

Queue ID: `GATE-AUTH11-NODE-01`

Status: `ACTIVE`

Authorization: the user's 2026-07-27 direction to complete CAL-04B and resolve
its assurance/gate-planner blockers through reviewed work-package corrections.

This defect-closure ExecPlan is maintained under
`docs/defect_closure_execplans.md`.

## Objective

Correct canonical terminal planning so a diff touching external-authority suite
posture selects the separately required
`auth11_required_suite_obligation_guards_contract` node in addition to the
source anti-evasion script, then resume the blocked exact-head heavy attempt
without manual gate injection.

## Reproducer

At exact commit `55ebd99bcca61896e79eacd68b3d905eaa06d0ef`,
canonical terminal plan
`9bf443db254878882cf53931a405c88ce5d4d934860f03c3d9391e5d53da88f5`
contains the full-workspace HEAVY node and authority anti-evasion LIGHT script
but omits
`cargo nextest run --test auth11_required_suite_obligation_guards_contract`.
The retained attempt is
`/home/workdir/gate-external-dag-dc01-attempt-001`; no LIGHT, audit, ledger, or
HEAVY execution occurred.

## Included Scope

- a canonical gate definition for the AUTH11 obligation guard;
- impact-map selection whenever external-authority suite posture is affected;
- planner/fixture tests proving stable order, exact arguments, inventory, and
  non-duplication with existing authority gates;
- focused validation, dual review, canonical chain/pre-heavy admission, and
  rerun of the required exact-head heavy closure plan.

## Excluded Scope

- manual execution outside the terminal plan;
- weakening/removing the authority anti-evasion script or existing alignment
  contract;
- CAL population, simulation science, Harvard access, deployment, or release;
- unrelated gate-policy selection changes.

## Declared Write Set

- `gate-policy/v1/gate-definitions.json`
- `gate-policy/v1/impact-map.json`
- `gate-policy/v1/fixtures/valid/gate-plan.json`
- `gate-policy/v1/fixtures/valid/gate-receipt.json`
- `gate-policy/v1/fixtures/valid/impact-map.json`
- `crates/openwepp-gate-planner/src/planner.rs`
- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260727-gate-planner-external-dag-closeout-correction-001/artifacts/implementation-gates.md`
- `docs/work-packages/20260727-gate-planner-auth11-terminal-node-selection-001/**`

No other path is writable. This write set must not widen.

## Execution Plan

1. Commit this prospective scaffold and obtain two independent read-only
   scaffold reviews before policy or Rust edits.
2. Before production edits, add this package to the catalog/roadmap and freeze
   the named red/green planner fixtures below.
3. Add the smallest gate-definition/impact-map correction and update canonical
   fixtures/tests.
4. Run gate-planner Nextest, warnings-denied Clippy, JSON/schema checks, exact
   terminal-plan reconstruction, formatting, diff, and line-count checks.
5. Obtain dual implementation review at a clean exact commit.
6. Run canonical package-chain and pre-heavy admission. The terminal plan must
   contain the anti-evasion script, AUTH11 guard, and fresh full workspace.
7. Delegate the one admitted heavy execution to the comparator runner, obtain
   dual terminal verification, archive prompts, and close.

## Acceptance

- Gate definition ID is `authority-required-suite-obligation-guards-v1`;
  target is the static
  `auth11_required_suite_obligation_guards_contract`; arguments are exactly
  `cargo nextest run --test
  auth11_required_suite_obligation_guards_contract`.
- The node is `LIGHT`, `BLOCKING`, `HARD_FAIL`, `NON_REUSABLE`,
  `REPOSITORY_REVIEWED`, timeout 300 seconds, maximum attempts 1, no retry
  reasons, `nextest-junit-v1`, exact inventory source
  `NEXTEST_TEST_TARGET`, minimum/exact count 3, and blocks `INCREMENT`.
- Prerequisites are `authority-admission-v1` and
  `authority-antievasion-v1`; stable ordering is after those two authority
  scripts and before `gate-policy-schema-consistency-v1`, HEAVY nodes, and
  workspace full regression.
- Positive impact entries use reason
  `AUTHORITY_REQUIRED_SUITE_OBLIGATION_CHANGED`, semantic surface
  `external-authority-required-suite`, and these literal IDs/matchers:
  `auth11-registry` exact
  `docs/specifications/external-authority/registry.yaml`;
  `auth11-obligations` exact
  `docs/specifications/external-authority/required-suite-obligations.json`;
  `auth11-suite-docs` prefix
  `docs/specifications/external-authority/suites/`;
  `auth11-promotion-protocol` exact
  `docs/specifications/external-authority/promotion-protocol.md`;
  `auth11-integration-contracts` prefix `tests/integration/auth`;
  `auth11-constitutive-cohorts` prefix `tests/fixtures/constitutive/`;
  `auth11-infile-cohorts` prefix `tests/fixtures/infile/`;
  `auth11-antievasion-tool` exact
  `tools/release/check_authority_suite_antievasion.sh`; and
  `auth11-gate-policy` prefix `gate-policy/`.
- `planner::tests::auth11_required_suite_node_is_selected_once_for_authority_surfaces`
  covers every positive path class and exact ordered node/inventory behavior.
- `planner::tests::auth11_required_suite_node_is_not_selected_for_unrelated_critical_diff`
  uses
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
  and proves an unrelated CRITICAL science/runtime diff does not select AUTH11.
- AUTH11 independently enumerated inventory must equal this exact sorted set:
  `auth11_all_active_required_suite_targets_exist_and_are_registered`,
  `auth11_obligations_schema_and_anchor_bindings_are_enforced`, and
  `auth11_registry_posture_and_protocol_guard_paths_exist`.
- Red plan remains exactly 12 nodes / 2,376 inventory. Green reconstruction
  must preserve those exact nodes/inventory and add one AUTH11 node with three
  independently enumerated tests: exactly 13 nodes / 2,379 inventory.
- Red and green plan IDs and the exact retained JSON path/hash are recorded in
  package evidence.
- Existing anti-evasion, alignment, and workspace nodes remain selected.
- The previously blocked external-DAG package receives a fresh canonical plan;
  the incomplete plan is retained but never executed.
- All focused and admitted heavy gates pass; dual review and verification pass.

## Security Invariants

- The canonical planner remains the sole test inventory authority.
- No caller may append or inject the missing node after planning.
- Required authority gates remain blocking and non-reusable.
- Harvard remains sealed and CAL population remains prohibited.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one bounded implementation worker, two independent
read-only reviewers, two independent terminal verifiers, and the
`comparator_suite_runner` for canonical heavy execution. Expected outputs are
bounded policy/Rust/evidence changes and retained plan/audit/receipt verdicts;
write access is limited to the declared write set.
