# PL06 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-pl06-decomposition-resup-kernel-surface-scaffolding-001/package.md


You are executing
`20260522-pl06-decomposition-resup-kernel-surface-scaffolding-001`.

Objectives:
1. Add typed decomposition/resup kernel interfaces for scheduler-facing PL
   decomposition surfaces.
2. Add placeholder decomposition/residue partition scheduler phases with
   deterministic ordering semantics.
3. Integrate scheduler-to-kernel decomposition boundary wiring with typed
   failures only.
4. Add integration/test evidence for interface shape and phase-order guards.
5. Publish PL06 implementation evidence and disposition.

Constraints:
- Preserve architecture-first and science-contract governance posture.
- Do not introduce silent defaults for missing/non-finite required
  decomposition runtime inputs.
- Preserve baseline phase-order constraints and align with PL05 scheduler
  scaffold assumptions.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved ordering ambiguity remains `HOLD`.

Required outputs:
- `artifacts/pl06-decomposition-resup-kernel-surface-contract.md`
- `artifacts/pl06-residue-partition-scheduler-phase-scaffold.md`
- `artifacts/pl06-decomposition-resup-transition-state-map.md`
- `artifacts/pl06-decomposition-surface-test-evidence.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/pl06_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
- If code is changed, run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
