# Acceptance Matrix

Status: `SCAFFOLD REVIEW REQUIRED`

Evidence class: `Static + Ran`

| ID | Obligation | Evidence |
|---|---|---|
| LB-01 | Original first-attempt failure retained | `/home/workdir/gate-auth11-test-provider-canonical-001` |
| LB-02 | Wrong-campaign root retained and rejected | `/home/workdir/gate-auth11-test-provider-canonical-001-retry-2` |
| LB-03 | Fresh secure bootstrap | focused fresh-ledger test |
| LB-04 | Existing history preserved and verified | focused existing/malformed tests |
| LB-05 | Symlink/non-regular fail closed | focused adversarial tests |
| LB-06 | Rust preflight unchanged | exact terminal diff/security review |
| LB-07 | Full regression | planner 227/227 and canonical exact-head receipt |
| LB-08 | Independent acceptance | dual review, terminal verification, and receipt verification |
