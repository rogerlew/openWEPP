# SR06 Kickoff Agent Prompt

You are executing `20260522-sr06-consumer-ownership-wiring-hillslope-kernels-001`.

Objective:
Wire slope and soil runtime seam surfaces into hillslope consumer boundaries
(runoff/soil/watbal/perc adapters) with typed error propagation only.

Constraints:
- Preserve architecture-first + contract-first posture.
- Preserve SR05 parser-to-runtime integration closure assumptions.
- Preserve canonical symbol continuity established by SR04; avoid alias drift.
- No fallback wrappers or silent defaults for required consumer inputs.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved high-severity consumer-boundary
  ambiguity remains `HOLD`.

Required outputs:
- `artifacts/hillslope-consumer-ownership-wiring-contract.md`
- `artifacts/hillslope-consumer-wiring-implementation-evidence.md`
- `artifacts/hillslope-consumer-boundary-coverage-matrix.md`
- `artifacts/sr06_disposition.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
