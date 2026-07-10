# Gate Results

Status: `EXECUTED-HOLD-MISSING-CHANNEL-HOURLY-SEDIMENT-SEQUENCING-AUTHORITY`

Evidence mode: `Static` plus `Ran` where stated.

| Gate | Result | Evidence |
|---|---|---|
| Execution baseline/worktree intake | PASS | Ran at HEAD `2be11f763c8f966a5ac3cab038d88af82650f637`; unrelated untracked artifacts preserved. |
| Instruction discovery | PASS | Ran `tools/agents/find-agents --for` on declared write set; recorded in `required-reading-map.md`. |
| Pinned baseline source map | PASS | Static/Ran searches; `baseline-source-map.md`. |
| Branch/topology support matrix | PASS | Static; `branch-topology-support-matrix.md`. |
| Operand lineage before production | PASS | Static; `operand-lineage.md`. |
| Canonical authority sufficient for production | BLOCKED | Missing interval WS18-WS26 sequencing/state authority; `hold-legitimacy-audit.md`. |
| Contract-derived tests | NOT RUN | Correctly withheld after blocked pre-implementation gate. |
| Rust/fixture/output implementation | NOT RUN | Correctly withheld; no partial path landed. |
| Protected dependency fail-closed test, initial filter | FAIL | Ran `cargo nextest run -p openwepp-watershed-orchestrator mt3_hourly_contributor_with_dependency_node_fails_closed`; no matching crate-local test because the case is a root integration test. This was a command-scope error, not a code failure. |
| Protected dependency fail-closed test, corrected | PASS | Ran `cargo nextest run --test wshedw5_typed_watershed_runtime_contract mt3_hourly_contributor_with_dependency_node_fails_closed`; 1 passed. |
| Existing M-T3 production CLI leaf consumer | PASS | Ran `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract`; 1 passed. |
| Real downstream consumer proof | BLOCKED | No typed routed hourly channel output; `consumer-path-evidence.md`. |
| Independent water/sediment reconstruction | BLOCKED | No new output/state exists; `conservation-reconstruction.md`. |
| Full Rust closure gates | NOT RUN | No Rust, contract, test, fixture, or output edit; pre-implementation hold. |
| Scoped Markdown lint | PASS | Final run: W11 28 files, W11A 18 files, roadmap 1, catalog 1; zero errors/warnings. |
| `git diff --check` | PASS | Final run; no whitespace errors. |
| Dual review/disposition | PASS | Independent Agent A/B reviews complete; all seven findings accepted and fixed in `disposition.md`. |
| Dual verification | PASS | Agent A `PASS-WITH-NOTES`; replacement independent Agent B `PASS`. The only A note was to await B and publish final status, now satisfied. |
