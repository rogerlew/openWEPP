# Acceptance Matrix

Status: `SCAFFOLD REVIEW REQUIRED`

Evidence class: `Static + Ran`

| ID | Obligation | Evidence |
|---|---|---|
| LB-01 | Original first-attempt evidence retained byte-identically | `failed-root-baselines.md` post-comparison |
| LB-02 | Wrong-campaign root retained byte-identically and rejected | `failed-root-baselines.md` post-comparison |
| LB-03 | Lexical raw path; final and ancestor no-follow validation | focused final/ancestor symlink tests |
| LB-04 | Exclusive fresh creation and file/parent durability | focused fresh-ledger and collision tests |
| LB-05 | Existing history preserved and verified | focused malformed JSON/hash/predecessor tests |
| LB-06 | Directory/FIFO/non-regular fail closed | focused adversarial tests |
| LB-07 | Bootstrap failure never invokes transition | mocked `_invoke` regression |
| LB-08 | Substitution before append/finalization fails closed | path-swap tests; outside target unchanged |
| LB-09 | Rust preflight and record schema unchanged | exact terminal diff/security review |
| LB-10 | Full regression | planner 227/227 and canonical exact-head receipt |
| LB-11 | Independent acceptance | dual review, terminal verification, and receipt verification |
