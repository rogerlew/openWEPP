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
| affected terminal crate inventory | PASS | 122/122 (117 library + 5 binary), zero failures/errors, 678.176 s; run `86155219-9bf2-4268-bb54-4b47d14c466c`; JUnit SHA-256 `f8323a3b6a91811487e8fb3d20a84000be0e24c1fe12d820c35d0b6919c87011` |
| campaign-global TESTGATE | DEFERRED-MASTER | owned by the master ExecPlan after all packages close |
