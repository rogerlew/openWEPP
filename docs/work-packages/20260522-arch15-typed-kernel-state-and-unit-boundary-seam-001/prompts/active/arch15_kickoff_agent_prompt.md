# ARCH15 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/package.md


You are executing `20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001`.

Objectives:
1. Remove stringly kernel seam state/flux maps (`BTreeMap<String, f64>`) from
   kernel contract and orchestrator writeback surfaces.
2. Implement typed symbol/value seam structures in
   `openwepp-kernel-contract`.
3. Wire `openwepp-unit-boundary` types into seam value modeling so unit-safe
   values are first-class boundary types.
4. Migrate hillslope/watershed orchestrators and integration tests to typed
   seam usage.
5. Produce full gate evidence and dual review/disposition/verification
   artifacts.

Constraints:
- This is a greenfield scientific hydrology simulation architecture.
- Do not add fallback wrappers that mask typed seam failures.
- Preserve deterministic ordering and existing writeback
  accept/reject/apply semantics.
- Preserve canonical WEPP/wepp-forest symbol continuity (legacy symbol labels
  remain valid identifiers).
- Correctness over completion: unresolved invariant/closure violations remain
  `HOLD`.
- Use truthfulness posture (`Static:` vs `Ran:`) in artifacts.

Required gates:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

Required outputs:
- `artifacts/typed-kernel-state-design.md`
- `artifacts/unit-boundary-seam-mapping.md`
- `artifacts/migration-plan-and-write-set.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/arch15_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
