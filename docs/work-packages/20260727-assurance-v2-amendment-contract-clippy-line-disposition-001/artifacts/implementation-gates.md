# Implementation Gate Evidence

Exact implementation commit:
`21fb046474699387deebc0c9916600cce8987594`.

Evidence class: `Ran`

| Gate | Result |
|---|---|
| focused assurance amendment target | PASS, 16/16; 2 skipped; run `bd7fd066-e6c2-47cb-847b-d2ca4466ce4d` |
| workspace Clippy with warnings denied | PASS |
| workspace doc tests, locked/offline | PASS |
| authority anti-evasion | PASS |
| AUTH11 obligation guards | PASS, 3/3; run `63b20375-c744-489b-b22c-04f22a1aba61` |
| formatting, docs lint, diff hygiene | PASS |
| full workspace Nextest profile | HOLD, 2,360/2,361 passed; 43 skipped; run `dd04b429-27d3-494d-96be-1d3a7a80423f` |

The retained full-profile failure was a stale source-string assertion in
`tests/integration/testgate_ci_executor_contract.rs`. It expects
`load_candidate_after_ready_audit(...)`, while the reviewed ledger correction
intentionally uses `load_candidate_after_ready_audit_text(...)` with bytes read
from the retained bound handle.

Successor
`20260727-testgate-bound-ledger-source-contract-alignment-001` replaced that
exact assertion and passed focused 11/11, strict Clippy, full 2,361/2,361,
dual implementation/terminal review, and fresh canonical receipt
`940e599d3ff77e6ef96e5ccae1343915a4edd67d4d1b948b0d3027502b2e6904`
with 12/12 nodes and 2,387/2,387 inventory items. The stale-assertion HOLD in
this artifact is therefore `LIFTED`; this package may proceed through its own
terminal/receipt disposition.

The assurance-specific canonical transaction at exact head
`ffe1dd71eec578a621f66fc2939304971653e92b` then passed all 12 nodes,
2,387/2,387 inventory items, and full 2,361/2,361 under campaign
`ASSURANCE-V2-CLIPPY-LINE-01`. The gate table's retained historical HOLD is
superseded by this exact closure evidence.

Harvard remained sealed and CAL population remained prohibited during these
gates.
