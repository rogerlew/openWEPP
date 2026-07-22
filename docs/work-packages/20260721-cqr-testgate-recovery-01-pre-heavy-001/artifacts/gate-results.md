# Gate Results

| Gate | Status | Evidence |
| --- | --- | --- |
| scaffold documentation lint | NOT RUN | run before scaffold commit |
| broad gate-planner quick baseline | INVALIDATED | run began before the intake-evidence amendment was committed; its verifier fixture correctly rejected the changed worktree with `GATE-COMMITTED-CHECKOUT-NOT-EXACT` after 228.121 s. It is not evidence of a `pre_heavy.rs` regression and will not be repeated as a broad suite. |
| focused `pre_heavy` unit inventory | PASS | `cargo nextest run -p openwepp-gate-planner pre_heavy::tests --profile quick`; 15 passed, 90 skipped, 16.175 s; run ID `cd06fa34-9bfa-4550-b083-0cd5b914cb37` |
| affected-surface CRAP/coverage | NOT RUN | required before completion |
| terminal-plan heavy gates | NOT RUN | delegate after implementation |
