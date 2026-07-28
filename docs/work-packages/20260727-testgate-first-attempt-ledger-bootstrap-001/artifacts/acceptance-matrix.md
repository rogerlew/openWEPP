# Acceptance Matrix

Status: `TERMINAL VERIFICATION REQUIRED`

Evidence class: `Static + Ran`

| ID | Obligation | Evidence |
|---|---|---|
| LB-01 | Original first-attempt evidence retained byte-identically | `failed-root-baselines.md` post-comparison |
| LB-02 | Wrong-campaign root retained byte-identically and rejected | `failed-root-baselines.md` post-comparison |
| LB-03 | Lexical raw path; final/every-ancestor no-follow identity | symlink and transition/append/finalization swap tests |
| LB-04 | Exclusive fresh creation and file/parent durability | focused fresh-ledger and collision tests |
| LB-05 | Existing history preserved and verified | focused malformed JSON/hash/predecessor tests |
| LB-06 | Directory/FIFO/non-regular fail closed | focused adversarial tests |
| LB-07 | Bootstrap failure never invokes transition | mocked `_invoke` regression |
| LB-08 | Substitution before transition/append/finalization fails closed | final/ancestor path-swap tests; outside target unchanged |
| LB-09 | Inherited FD binds Rust transition to admitted inode | Python pass-fd and Rust bound-handle tests |
| LB-10 | Invalid/mismatched FD fails before LIGHT | missing/malformed/closed/directory/mismatch tests |
| LB-11 | Post-admission path swap cannot redirect Rust I/O | final/ancestor race tests; replacement untouched |
| LB-12 | Original path authority and schema remain exact | path-hash/recovery tests and security review |
| LB-13 | Full regression | planner 236/236 plus successor full 2,361/2,361 |
| LB-14 | Independent acceptance | dual review plus queued terminal/receipt verification |
| LB-15 | Rust line-count governance | `line-count-disposition.md`; 2,762/2,119-line WARNs with split intents |
