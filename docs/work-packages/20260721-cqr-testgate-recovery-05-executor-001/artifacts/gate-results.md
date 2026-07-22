# Gate Results

| Gate | Status | Evidence |
| --- | --- | --- |
| scaffold Markdown lint | PASS | package scaffold at `71cc2aa9` |
| dual eligibility review | PASS | independent A/B at exact scaffold |
| exact pre-production baseline | PASS | 125 pass, two intentional ignores; one traversal |
| focused stage/receipt characterization | PASS | direct, LIGHT/FINAL_LIGHT/reject, and READY-audited HEAVY cases |
| focused coverage-floor matrix | PASS | 6/6 plus split path test 1/1 |
| targeted Clippy | PASS | all targets, warnings denied |
| formatting and diff hygiene | PASS | exact final implementation head |
| dual implementation review | PASS | Review C/D at exact clean `2c0f1b12` after all findings corrected |
| first changed-head metric | FAIL-CORRECTED | `9fe678a7`; retained and not rerun |
| authoritative corrected metric | PASS | `2c0f1b12`; 135 pass, lines 92.96%, regions 85.01%, floor 79/79, zero CRAP above 30 |
| terminal verification A | PASS | independent hash/count/floor/CRAP/package audit at `98d6fc62`; no rerun |
| terminal verification B | HOLD-CORRECTED | implementation/metric PASS; three scaffold-doc findings corrected at changed docs head |
| terminal verification B re-audit | PENDING | exact corrected evidence commit required |
| campaign-global TESTGATE | DEFERRED-MASTER | master owns it after all seven packages close |
