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
| remaining-hotspot extraction | PASS | `cargo clippy -p openwepp-gate-planner --lib -- -D warnings`; focused inventory 20 passed, 90 skipped, 15.282 s; run ID `baf52c7c-5b93-497f-8b2a-34499f11150c` |
| final affected CRAP | PASS | exact head `f1774586`; 105/105 instrumented tests passed; zero target rows above 30; evidence `/tmp/cqr-pre-heavy-final-yzFYI4` |
| coverage non-regression | PASS | exact head `3d6e8817`; line 67.79%, region 71.08%, 109/109 instrumented tests passed |
| final affected CRAP after characterization | PASS | zero target rows above 30; maximum exactly 30; matching CRAP SHA `673e514f...` |
| line-count governance | PASS-WARN | 2,859 production-host lines; WARN accepted, below 3,000 blocker; direct closure tests moved to an authorized child module |
| affected-surface CRAP/coverage | PASS | exact source-bound evidence retained under `/tmp/cqr-pre-heavy-nonreg-rNRfRV` |
| terminal-plan heavy gates | NOT RUN | delegate after implementation |
| ADR-0021 prior affected measurement | HOLD | exact clean head `b1096a78`; aggregate production surface passes, but reviewers found three functions below the binding region floor |
| reviewer focused inventory | PASS | Review A 31/31, run ID `0cf42846-6326-4336-ab4f-8271e9e31176`; Review B 31/31, run ID `bbcaad4d-09ed-4ba3-a88d-eb3e8e44b5bc` |
| final scoped Clippy | PASS | `cargo clippy -p openwepp-gate-planner --lib --tests -- -D warnings` |
| exact public reconstruction characterization | PASS | `exact_planner_output_reconstructs_through_the_public_audit_path`; 1/1 PASS in 391.957 seconds; run ID `4c972c0f-405d-4475-82d3-23ef39f0205b` |
| corrected changed-head ADR-0021 measurement | PASS | exact clean `68e9b747`; 117/117 PASS; production line 96.08%; production region 89.64%; 111/111 functions at least 75% region, minimum 80%; CRAP maximum 17.0 |
| fixture ownership probe | PASS | exact temp-repository and durable-ledger namespaces empty before and after `ready_audit_validation_execution_and_resume_chains_are_directly_bound`; 1/1 PASS in 55.258 seconds; run ID `cf7e6313-eda8-418f-8c6c-b91eb97e0b31` |
| true pre-production baseline | PASS | isolated detached `5e0e92c5`; 100/100 PASS; production line 44.34%, region 41.59%; 50 functions, 24 below 75%; active worktree untouched |
| renewed implementation review A | PASS | exact `dcc86c39`; all prior semantic/evidence findings cleared |
| renewed implementation review B | PASS | exact `dcc86c39`; all prior semantic/evidence findings cleared; Markdown lint and diff-check PASS |
