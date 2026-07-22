# Gate Results

| Gate | Status | Evidence |
| --- | --- | --- |
| scaffold Markdown lint | PASS | 23 files, zero errors or warnings |
| dual eligibility review | PASS | two independent reviews at exact `02745e8d` |
| exact pre-production library baseline | PASS | 117/117 instrumented; one traversal; retained evidence root |
| characterization | PASS | 2/2 focused direct tests after one fixture-only correction; run `3788975e-47c3-4545-9980-dc618ef504f9` |
| corrected focused characterization | PASS | 2/2; run `952e6507-01df-45fe-bdef-8cfb4f005346` |
| targeted Clippy | PASS | library/tests with warnings denied at corrected source |
| first changed-head metric | INVALIDATED-REVIEW | `424a1a5c`; numeric pass but two semantic review blockers; not reused |
| dual corrected implementation review | PASS | independent A/B at exact `d5af6207`; both blockers closed |
| authoritative affected library/coverage/CRAP | PASS | 119 pass, one intentional ignore; lines 96.95%, regions 89.08%, floor 15/15, CRAP max 5.024 |
| dual terminal verification | PASS | independent A/B at exact clean `0b3e989b`; no gate rerun |
| campaign-global TESTGATE | DEFERRED-MASTER | master seven-package ExecPlan owns it after all ranks close |
