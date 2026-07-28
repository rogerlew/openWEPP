# Canonical Execution

Status: `PASS`

Evidence class: `Ran`

Comparator verdict: `PASS`

Subject: `10d2c2004d7c85a72a5cae7d73c2b571f245960f`

Authority base: `f8cba1c9f3e02d241a2bb7fccc3329a0a142ac57`

Boundary/campaign/attempt:
`INCREMENT` / `TESTGATE-BOUND-LEDGER-CONTRACT-01` / 1.

Artifact root:
`/home/workdir/gate-testgate-bound-ledger-contract-canonical-001`

Ledger:
`/home/workdir/gate-testgate-bound-ledger-contract-history.jsonl`

The fresh no-retry transaction passed package-chain admission, LIGHT, all ten
READY-audit checks, and HEAVY. Counts are 12 passed, 0 failed, 0 blocked, 0
retried, and 0 skipped. Planned and executed inventories both contain 2,387
items across 12 DAG nodes. The authenticated full-profile node passed
2,361/2,361 with 43 skipped under Nextest
`910d8172-8ff3-4008-8fba-15507f4cdd6b`.

Identifiers:

- plan ID:
  `2239dc7d4e5db6b463eadcc1e34f85f1069960c4310c1bd8ffe5b31621444a5a`;
- plan SHA-256:
  `2369d79397bcdaab349887574f258b28fbe32992b5ac79667e5e893717518b0f`;
- LIGHT receipt ID:
  `64bfb47d095f71c8da80ee38d1379f8cc81024131764b1315d4f7b4a36a02c00`;
- audit ID:
  `6c1776f8534ddc5884eab866e2c5604380372990dd78e7b17f2cbf08cf71223e`;
- HEAVY receipt ID:
  `940e599d3ff77e6ef96e5ccae1343915a4edd67d4d1b948b0d3027502b2e6904`.

File SHA-256 values:

- intent plan: `b1be33c393465bf360f2a1978b790f21667366a3f7a9eab002eafb4a4009f787`;
- terminal plan:
  `2369d79397bcdaab349887574f258b28fbe32992b5ac79667e5e893717518b0f`;
- LIGHT receipt:
  `64dc0b9b2dcae4dcced7cba147fdc73bb5abc80fa12b7d0ec8535fd0e4ad460e`;
- READY audit:
  `dbfad235f3099cd7cb08dd38ff0a4f4a0b53aecc3c339f2a5a21bd2157978aba`;
- receipt:
  `8da10fad47c78a61f9e8d1541308226a56bbba224662c77750b554487f6a6d71`;
- attempts:
  `bd936261487045185527a0261404c127504c24f4c86e27e63914ae117a8181e1`;
- external ledger:
  `bd936261487045185527a0261404c127504c24f4c86e27e63914ae117a8181e1`;
- attempt index:
  `4ae7a31ee80b7eb3b29d5dadac0de76386315135910e2cee6652904528b4a55b`.

The ledger chain is balanced: LIGHT `CLOSED`, HEAVY `STARTED`, HEAVY `CLOSED`
with the exact receipt ID, then transaction `CLOSED`, with every
`previous_entry_sha256` linked. `LOCAL_UNTRUSTED` is expected forest1/local
unsigned evidence and is not a hold. Coverage/CRAP is the canonical
`DEFERRED_TO_QUALITY_CI` observational disposition and is closure-eligible.
