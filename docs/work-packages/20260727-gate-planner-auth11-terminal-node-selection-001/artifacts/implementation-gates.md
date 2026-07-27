# Implementation Gate Evidence

Exact implementation state: pending clean commit.

## Static

- The gate definition is
  `authority-required-suite-obligation-guards-v1`.
- The node is LIGHT, blocking, hard-fail, non-reusable, repository-reviewed,
  and has the two frozen authority prerequisites.
- Its argv is exactly `cargo nextest run --test
  auth11_required_suite_obligation_guards_contract`.
- Its inventory source is `NEXTEST_TEST_TARGET`, with exact/minimum count
  three.
- Nine literal impact entries select the node for the frozen external-authority
  surfaces.
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

Independent `cargo nextest list --test
auth11_required_suite_obligation_guards_contract` enumerated exactly:

1. `auth11_all_active_required_suite_targets_exist_and_are_registered`
2. `auth11_obligations_schema_and_anchor_bindings_are_enforced`
3. `auth11_registry_posture_and_protocol_guard_paths_exist`

## Pending

- exact clean implementation commit;
- canonical terminal reconstruction proving 13 nodes / 2,379 inventory;
- dual implementation review;
- canonical package-chain and pre-heavy admission;
- admitted heavy execution and dual terminal verification.

Harvard remained sealed and CAL population did not run.
