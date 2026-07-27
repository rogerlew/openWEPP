# Blocked Plan Evidence

Evidence class: `Ran`

Canonical terminal planning at exact commit `55ebd99b` produced plan
`9bf443db254878882cf53931a405c88ce5d4d934860f03c3d9391e5d53da88f5`
with 12 nodes and inventory cardinality 2,376.

Exact retained JSON:
`/home/workdir/gate-external-dag-dc01-attempt-001/terminal-plan.json`

SHA-256:
`8c13c672e39e9f50701d7650415b7958f2c76a0864842736e529a2c32cd8f55e`

Source identities:

- base `1f93921d7ecc2e2b05b528a10b5b11d4d5c1b624`;
- head `55ebd99bcca61896e79eacd68b3d905eaa06d0ef`;
- package-chain
  `b77a8c1163dbf85d703f48616e07b7a2d64047fe918a1ae64978bc34c8b8b475`;
- intent plan
  `8fb099bf63e1c1320bf0d8028f22038c7f1b677cb631bdd14ce3dc2b363dc80c`.

Ordered nodes and inventory counts:

| Order | Definition | Count |
|---:|---|---:|
| 1 | `authority-admission-v1` | 1 |
| 2 | `authority-antievasion-v1` | 1 |
| 3 | `cargo-fmt-v1` | 1 |
| 4 | `documentation-lint-v1` | 1 |
| 5 | `gate-policy-schema-consistency-v1` | 11 |
| 6 | `hard-invariant-native-canopy-management-v1` | 71 |
| 7 | `hard-invariant-native-canopy-plant-v1` | 19 |
| 8 | `hard-invariant-native-canopy-runtime-v1` | 632 |
| 9 | `placeholder-scan-v1` | 1 |
| 10 | `workspace-clippy-v1` | 1 |
| 11 | `workspace-doctest-v1` | 1 |
| 12 | `workspace-full-nextest-v1` | 2,350 |

Present:

- `bash tools/release/check_authority_suite_antievasion.sh ...`;
- `cargo nextest run --workspace --profile full` with 2,350 tests.

Missing:

- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`.

Disposition: `BLOCKED BEFORE LIGHT`. No audit, ledger, receipt, heavy process,
CAL population, or Harvard access occurred.

The red plan has 2,376 globally unique inventory IDs and 3,090 summed per-node
entries. Its workspace-full node has 2,350 tests.

Corrected green expectation: preserve the 12-node set and add exactly one
AUTH11 LIGHT node with three independently enumerated tests. Those three test
IDs already occur in the workspace-full inventory, while the two new planner
tests are new globally unique IDs. The exact green cardinalities are therefore
13 nodes, 2,378 globally unique IDs, 3,095 summed per-node entries, and 2,352
workspace tests. The earlier 2,379 expectation double-counted the already
present AUTH11 tests and is rejected as an acceptance-model defect.
