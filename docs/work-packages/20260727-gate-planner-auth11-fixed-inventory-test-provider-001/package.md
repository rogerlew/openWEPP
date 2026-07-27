# Gate Planner AUTH11 Fixed-Inventory Test Provider Correction

Package ID:
`20260727-gate-planner-auth11-fixed-inventory-test-provider-001`

Queue ID: `GATE-AUTH11-TEST-PROVIDER-01`

Status: `ACTIVE`

Authorization: the user's 2026-07-27 direction to complete CAL-04B and resolve
its assurance/gate-planner blockers through reviewed work-package corrections.

This defect-closure ExecPlan is maintained under
`docs/defect_closure_execplans.md`.

## Objective

Correct the shared planner coverage-test inventory provider so the
gate-policy-change determinism test supplies the exact three AUTH11 inventory
items now required by prospective terminal-node selection, without weakening
production inventory enforcement or changing production planner behavior.

## Reproducer

At exact commit `964b449a51163811f5737bff98b6295df52364d3`,
`cargo nextest run -p openwepp-gate-planner` runs 227 tests and fails only
`planner::tests::gate_policy_change_is_deterministic_and_critical` with
`GATE-INVENTORY-EMPTY`. The test-local `FixedInventory` provider returns one
synthetic ID for every definition, while
`authority-required-suite-obligation-guards-v1` requires exactly three.
Focused production-path selection, AUTH11 3/3, alignment 11/11, Clippy,
formatting, JSON, and anti-evasion checks pass.

## Included Scope

- make `FixedInventory` return the exact frozen three AUTH11 test identities
  only for `authority-required-suite-obligation-guards-v1`;
- retain its existing one-item behavior for every other definition;
- assert the deterministic gate-policy plan contains the AUTH11 node with
  exact three-item inventory and both generated prerequisite edges;
- rerun focused and full gate-planner validation;
- return the predecessor AUTH11 package to fresh dual implementation review,
  exact terminal reconstruction, and canonical admission.

## Excluded Scope

- production planner, policy, fixture, schema, executor, or verifier changes;
- manual gate injection, heavy execution before admission, CAL population,
  Harvard access, deployment, or release.

## Declared Write Set

- `crates/openwepp-gate-planner/src/planner_coverage_tests.rs`
- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260727-gate-planner-auth11-terminal-node-selection-001/artifacts/implementation-gates.md`
- `docs/work-packages/20260727-gate-planner-auth11-fixed-inventory-test-provider-001/**`

No other path is writable. This write set must not widen.

## Execution Plan

1. Commit this prospective scaffold and obtain two independent read-only
   scaffold reviews before the test-provider edit.
2. Apply the smallest test-only provider correction and exact assertions.
3. Run the reproducer, focused AUTH11 tests, full gate-planner Nextest, strict
   package Clippy, formatting, JSON/alignment, diff, and line-count checks.
4. Obtain dual implementation review at one exact clean commit.
5. Reconstruct the predecessor AUTH11 terminal plan and require 13 nodes,
   2,378 globally unique inventory IDs, 3,095 summed per-node entries, 2,352
   workspace tests, the exact three AUTH11 IDs, and exact prerequisite edges.
6. Resume canonical admission only after every check is green.

## Acceptance

- Only the test-local `FixedInventory` implementation changes.
- Its AUTH11 branch returns exactly the three sorted identities frozen in the
  predecessor package; every other branch retains its previous single
  `definition:target` synthetic identity.
- The reproducer passes without lowering `minimum_count`, bypassing exact
  inventory, padding production inventory, or special-casing production code.
- The deterministic gate-policy test proves AUTH11 selection, exact inventory,
  and exact generated admission/anti-evasion prerequisite node IDs.
- Full gate-planner Nextest passes 227/227 and all predecessor focused gates
  remain green.
- Dual review and exact terminal reconstruction pass before canonical
  execution.

## Security Invariants

- Production inventory enforcement remains fail-closed.
- The canonical planner remains the sole real inventory authority.
- Harvard remains sealed and CAL population remains prohibited.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one bounded implementation worker and two independent
read-only reviewers; expected outputs are the exact test-provider correction,
focused/full validation, and review dispositions; write access is limited to
the declared write set.
