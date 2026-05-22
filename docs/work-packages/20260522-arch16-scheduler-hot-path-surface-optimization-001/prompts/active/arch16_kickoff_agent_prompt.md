# ARCH16 Kickoff Agent Prompt

You are executing `20260522-arch16-scheduler-hot-path-surface-optimization-001`.

Objectives:
1. Eliminate or materially reduce avoidable full-map clone operations in
   hillslope and watershed scheduler kernel execution paths.
2. Preserve deterministic ordering and typed writeback semantics.
3. Preserve closure/status behavior and failure-class routing.
4. Provide explicit before/after hot-path evidence.
5. Produce dual review/disposition/verification artifacts.

Constraints:
- Keep typed seam boundaries introduced in ARCH15.
- Do not reintroduce `BTreeMap<String, f64>` kernel seam maps.
- Do not weaken invariant guards or hide writeback errors.
- Correctness over completion: unresolved closure/invariant regressions remain
  `HOLD`.
- Use truthfulness posture (`Static:` vs `Ran:`) in artifacts.

Required gates:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

Required outputs:
- `artifacts/hot-path-benchmark-and-allocation-evidence.md`
- `artifacts/seam-impact-and-compatibility-notes.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/arch16_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
