# ARCH18 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-arch18-hbp-authority-and-convergence-closure-001/package.md


You are executing `20260522-arch18-hbp-authority-and-convergence-closure-001`.

Objectives:
1. Define explicit HBP authority model and parser-vs-bridge responsibility
   boundaries.
2. Reconcile/constrain HBP implementation behavior to prevent divergence.
3. Add convergence test evidence and fixture-based closure checks.
4. Record provenance pin evidence aligned with ADR-0012 governance.
5. Produce dual review/disposition/verification artifacts.

Constraints:
- openWEPP owns implementation; no implicit external authority drift.
- Preserve strict typed failures for malformed/ambiguous HBP payloads.
- Do not rely on unpinned external references as sole authority.
- Do not absorb parquet boundary authoring into ARCH18; parquet boundary
  contracts are owned by ARCH19.
- Correctness over completion: unresolved authority ambiguity remains `HOLD`.
- Use truthfulness posture (`Static:` vs `Ran:`) in artifacts.

Required gates:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

Required outputs:
- `artifacts/hbp-authority-split-and-governance.md`
- `artifacts/hbp-convergence-test-evidence.md`
- `artifacts/hbp-provenance-pin-record.md`
- `artifacts/arch19-parquet-schema-handoff.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/arch18_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
