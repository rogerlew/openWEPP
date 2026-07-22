# Gate Results

| Gate | Status | Evidence |
| --- | --- | --- |
| scaffold Markdown lint | PASS | 23 files, zero errors or warnings |
| dual eligibility review | PASS | two independent reviews at exact `62cb1086` |
| exact pre-production binary baseline | PASS | 5/5 instrumented tests; one traversal; evidence in `/tmp/cqr-main-baseline-MVGA48` |
| characterization | PASS | existing 5/5 binary tests plus retained real-consumer Attempt 15 LIGHT/READY/HEAVY lifecycle evidence |
| changed-head binary unit inventory | PASS | 5/5; Nextest run `17adcfd1-6690-44e6-a86a-23077ba38ae5` |
| changed-head source-contract probe | PASS | 1/1; Nextest run `69cd411b-9ed3-4537-a05b-ebdc2fe46e9e` |
| targeted Clippy | PASS | binary tests, warnings denied |
| affected CRAP | PASS | 5/5 instrumented; zero rows above 30; maximum 30; one traversal at exact `dc935c7a` |
| dual implementation review | PASS | independent A/B reviews at exact `dc935c7a` |
| affected terminal crate inventory | NOT RUN | pending final focused run |
| campaign-global TESTGATE | DEFERRED-MASTER | owned by the master ExecPlan after all packages close |
