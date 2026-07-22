# Gate Results

| Gate | Status | Evidence |
| --- | --- | --- |
| scaffold documentation lint | NOT RUN | run before scaffold commit |
| broad gate-planner quick baseline | INVALIDATED | run began before the intake-evidence amendment was committed; its verifier fixture correctly rejected the changed worktree with `GATE-COMMITTED-CHECKOUT-NOT-EXACT` after 228.121 s. It is not evidence of a `pre_heavy.rs` regression and will not be repeated as a broad suite. |
| focused `pre_heavy` unit inventory | PASS | `cargo nextest run -p openwepp-gate-planner pre_heavy::tests --profile quick`; 15 passed, 90 skipped, 16.175 s; run ID `cd06fa34-9bfa-4550-b083-0cd5b914cb37` |
| characterization inventory | PASS | `cargo fmt --all`; then the same focused inventory, 19 passed, 90 skipped, 15.561 s; run ID `5bd3f726-7c16-49ba-8678-b2e7ed451dc0` |
| first extraction focused inventory | PASS | `cargo fmt --all -- --check`; `cargo nextest run -p openwepp-gate-planner pre_heavy::tests --profile quick`; 19 passed, 90 skipped, 15.710 s; run ID `bd87c218-85b7-4379-af70-177eec5d0f16` |
| first extraction scoped Clippy | NOT RUN/UNCONFIRMED | invoked as `cargo clippy -p openwepp-gate-planner --lib -- -D warnings`; client did not return a terminal status, so no result is asserted and it will be rerun only after the next source increment |
| delegated first affected CQR metric | PASS | `cargo llvm-cov -p openwepp-gate-planner --lib --ignore-run-fail --lcov` exited 0; 104/104 passed in 496.36 s. Matching `cargo crap` exited 0. Eight target rows remain; see `crap-after.md`. |
| second extraction focused inventory | PASS | 19 passed, 90 skipped, 16.876 s; run ID `8f3c1bd4-18f0-4a40-b9a4-3a97d99559fb` |
| durable-defect characterization | PASS | 20 passed, 90 skipped, 16.521 s; run ID `f3262ee4-9f1a-4567-9269-704c4637cf2a` |
| durable-defect extraction | PASS | 20 passed, 90 skipped, 16.658 s; run ID `31649409-1f30-4002-8c95-6442c4ad97c5` |
| affected-surface CRAP/coverage | NOT RUN | required before completion |
| terminal-plan heavy gates | NOT RUN | delegate after implementation |
