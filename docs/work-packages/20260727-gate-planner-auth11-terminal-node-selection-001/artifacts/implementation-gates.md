# Implementation Gate Evidence

Reviewed implementation state: `0c71782ad447953ec6006549050bbc2806f356ae`
is `HOLD`; its retained terminal plan must not execute.

Review correction state: pending clean commit. Accepted production findings are
corrected within the declared write set:

- cross-target STATIC prerequisites bind generated prerequisite node IDs;
- AUTH11 is selected only by explicit impact entries;
- the unrelated CRITICAL negative case builds a plan and proves absence;
- the integration matcher and positive case use the exact real repository
  path;
- fixture impact selection and receipt node counts are reconciled.

## Static

- The gate definition is
  `authority-required-suite-obligation-guards-v1`.
- The node is LIGHT, blocking, hard-fail, non-reusable, repository-reviewed,
  and has the two frozen authority prerequisites.
- Its argv is exactly `cargo nextest run --test
  auth11_required_suite_obligation_guards_contract`.
- Its inventory source is `NEXTEST_TEST_TARGET`, with exact/minimum count
  three.
- Eight literal impact entries select their real external-authority surfaces.
  Review found the integration matcher used a fictitious directory path; its
  corrected exact real-file matcher is pending.
- The valid policy fixtures bind the new node, arguments, inventory, and
  receipt shape.
- `planner.rs` is 2,642 lines: above the 2,000-line warning threshold and below
  the 3,000-line closure blocker. The package makes a bounded test-only
  addition; decomposition remains follow-on maintenance rather than a current
  correctness blocker.

## Ran

All commands ran in `/home/workdir/openWEPP` against the uncommitted exact
implementation diff:

| Gate | Result |
|---|---|
| `cargo nextest run -p openwepp-gate-planner auth11_required_suite_node` | PASS, 2/2, run `3c8331c6-2953-4a43-a19f-6adfedd21277` |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | PASS, 3/3, run `f042396c-9347-40e3-bcdb-b78a96ddc062` |
| `cargo nextest run --test testgate_align_authority_contract` | PASS, 11/11, run `55b46c9b-3c4d-4563-8737-4f249a584996` |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS |
| `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings` | PASS |
| `cargo fmt --all -- --check` | PASS |
| JSON parsing for both policy files and all three valid fixtures | PASS |
| `git diff --check` | PASS |

Post-review reruns:

| Gate | Result |
|---|---|
| focused AUTH11 planner tests | PASS, 2/2, run `2b13663a-79d8-4e44-b408-8b747a0c4e67` |
| AUTH11 obligation guards | PASS, 3/3, run `1022ab5c-2d44-443b-bcb6-0f7c3e8dfee4` |
| gate-policy alignment | PASS, 11/11, run `9ee8cd94-4ace-4c05-9f2e-ebfd80b15215` |
| strict package Clippy | PASS |
| anti-evasion script | PASS |
| formatting, JSON, diff hygiene | PASS |
| full gate-planner Nextest | FAIL, 226/227, run `b47dca9b-8772-46fe-a596-c5efc464851f` |

The one full-suite failure was
`planner::tests::gate_policy_change_is_deterministic_and_critical`. Its
`FixedInventory` test provider lives in
`crates/openwepp-gate-planner/src/planner_coverage_tests.rs`, outside this
package's frozen write set, and returns one synthetic item for every selected
node. The now-correct `auth11-gate-policy` selection requires exactly three
AUTH11 items, so planning fails closed with `GATE-INVENTORY-EMPTY`. The
production inventory and focused AUTH11 tests are correct. Prospective
successor
`20260727-gate-planner-auth11-fixed-inventory-test-provider-001` now owns the
test-only correction. Its uncommitted exact diff passes the reproducer and full
planner suite 227/227; clean-commit review and terminal verification remain
pending.

Independent `cargo nextest list --test
auth11_required_suite_obligation_guards_contract` enumerated exactly:

1. `auth11_all_active_required_suite_targets_exist_and_are_registered`
2. `auth11_obligations_schema_and_anchor_bindings_are_enforced`
3. `auth11_registry_posture_and_protocol_guard_paths_exist`

## Pending

- exact clean implementation commit;
- prospective correction of the out-of-scope shared fixed-inventory provider;
- canonical terminal reconstruction proving 13 nodes, 2,378 globally unique
  IDs, 3,095 summed per-node entries, and 2,352 workspace tests;
- fresh dual implementation review;
- canonical package-chain and pre-heavy admission;
- admitted heavy execution and dual terminal verification.

Harvard remained sealed and CAL population did not run.
