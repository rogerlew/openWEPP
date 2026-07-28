# Canonical Execution

Status: `PASS`

Evidence class: `Ran`

Comparator verdict: `PASS`

Subject/base:
`ffe1dd71eec578a621f66fc2939304971653e92b` /
`388432b8b8ee595c1f4433df49903ab34809f039`.

Boundary/campaign/attempt:
`INCREMENT` / `ASSURANCE-V2-CLIPPY-LINE-01` / 1.

Artifact root:
`/home/workdir/gate-assurance-v2-clippy-line-canonical-001`.

Ledger: `/home/workdir/gate-assurance-v2-clippy-line-history.jsonl`.

The fresh no-retry transaction passed package-chain admission, LIGHT, all ten
READY-audit checks, and HEAVY. Counts are 12 passed, 0 failed, 0 blocked, 0
retried, and 0 skipped. Planned and executed inventories both contain 2,387
items across 12 DAG nodes. The full-profile node passed 2,361/2,361 with 43
skipped under Nextest `6e141833-f10f-4c45-ba1d-8da196c4f198`.

Identifiers:

- plan ID:
  `10eda6ef262781d9c83e38f448444b30a7fff9f28fd083bb23ed0116514c87c9`;
- plan SHA-256:
  `14b7fabf9937807e33b8c27cef2fcb3b9ba8a4cb8ff3fca51aff3d1801bd897a`;
- LIGHT receipt ID:
  `7295de0245ab77096cc2fd8f5f774831d1b39dba0718dcb4199b369434e73015`;
- audit ID:
  `740e4371337d1bed63a062cb72512df9422bce3c95c52f9546d8e90837292975`;
- HEAVY receipt ID:
  `29d71a54d2cf38680190885abaf2d2967d547cdedefc0c31af5e00de669aa5d4`.

File SHA-256 values:

- intent plan: `4d06089164c43709a62282ea98a17057faec849cac22e38ab0227334d3af51c5`;
- terminal plan:
  `14b7fabf9937807e33b8c27cef2fcb3b9ba8a4cb8ff3fca51aff3d1801bd897a`;
- LIGHT receipt:
  `e5914025dbbe01ea8f56c11ae3e0ae154dac31491e50e2168803f3d32a5eceda`;
- READY audit:
  `eb3075cf5c75a28373ab01002b5b9f63f5cf925bd52b671013e232faa5663777`;
- receipt:
  `9c1633fd8ff53d47d33051ff3d401650328c3e1d80d6bdc5d3e32f0297af5a1b`;
- attempts and external ledger:
  `f159cb82b83223792a44947ec15f37ce61d74e7d682a67c951d9caca3844ea18`;
- attempt index:
  `1ad8b0af317529364b3db7b2ff258234da2b3d501263a0ce21b038961589883d`.

The ledger is balanced and hash-linked: LIGHT `CLOSED`, HEAVY `STARTED`,
HEAVY `CLOSED` with the exact receipt, and transaction `CLOSED`. Source
mutation before/after SHA-256 is unchanged. Coverage/CRAP is the canonical
closure-eligible `DEFERRED_TO_QUALITY_CI` observation. CAL and Harvard were not
accessed.
