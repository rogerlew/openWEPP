# Gate Results

| Gate | Status | Evidence |
| --- | --- | --- |
| scaffold Markdown lint | PASS | 23 files, zero errors or warnings |
| dual eligibility review | PASS | independent A/B at `51b1f63b` |
| exact pre-production baseline | PASS | 119 pass, one intentional ignore; one traversal |
| focused characterization | PASS | attestation, checkpoint, candidate, and READY-audit cases |
| affected resume tests | PASS | 13/13 after production refactor |
| clean-head crate library | PASS | 123/123 executed at reviewed production head; two intentional ignores |
| targeted Clippy | PASS | library/tests, warnings denied after test-only correction |
| dual implementation review | PASS | A/B plus exact correction confirmations |
| first post-change metric | FAIL-CORRECTED | `7faa45f9`; retained, not rerun/reused |
| authoritative corrected metric | PASS | 125 pass; lines 92.38%, regions 85.50%, floor 29/29, CRAP max 25.3961 |
| dual terminal verification | PENDING-FINAL-RECORD | technical evidence passed; durable lineage blocker corrected in this increment |
| campaign-global TESTGATE | DEFERRED-MASTER | master owns it after all seven packages close |
