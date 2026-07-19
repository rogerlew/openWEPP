# Critical Execution Failure

Evidence class: `Ran`

Artifact root: `/tmp/openwepp-testgate-digest-57a9Sq`.

The admitted local execution bound base
`cd5fe8e62fec6977e4ff5abb4e2279a60a931aa8` to head
`2981cdb817cc7f1ee46f5f043c8bf1e9e5264aec`, classified the increment
`CRITICAL`, and selected 12 nodes with 2,183 inventory items.

Receipt `1ad770581b147ba8bb8797e431d2a2d81e6395a61a231f03f807f44bd5ee1d6e`
is `LOCAL_UNTRUSTED` and `FAIL`: 8 nodes passed, workspace Clippy failed, 3
dependent nodes blocked, and no retry occurred. The failure is:

```text
tests/integration/testgate_ci_executor_contract.rs:58
assert_workflow_and_rollback_contract: 193/100 lines
clippy::too_many_lines
```

Cargo format, cargo-deny, documentation lint, placeholder scan, gate-policy
schema consistency, authority admission, authority anti-evasion, and required
authority passed. Workspace doctest, full Nextest, and global adjudicated CRAP
were correctly blocked by the failed Clippy prerequisite.

No GitHub workflow or forest1 runner was used. The named correction successor
is `20260719-testgate-adversarial-clippy-cleanup-001`.
