# CLIM16 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-clim16-climate-governance-register-normalization-001/package.md


You are executing
`20260522-clim16-climate-governance-register-normalization-001`.

Objective:
Normalize CLIM governance vocabulary/register state and reconcile stale
HOLD/GO status drift across CLIM packages, including corrected governance
framing for legacy `ip*=0.70` and explicit `datver>=4.0` branch-policy
confirmation.

Constraints:
- Treat CLIM11..15 outcomes and CLIM01/CLIM04 dispositions as authoritative
  inputs.
- Treat corrected CLIM04 review framing (`0.70` is valid legacy behavior) as
  binding; focus closure on provenance and governance sync, not defect removal.
- Preserve truthfulness posture; do not assert gate/test evidence that was not
  run.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved governance drift remains `HOLD`.

Required outputs:
- `artifacts/climate-governance-normalization.md`
- `artifacts/climate-register-reconciliation.md`
- `artifacts/cligen-datver-branch-policy-confirmation.md`
- `artifacts/clim16_disposition.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
1. `cargo fmt --check` (if code files change)
2. `cargo clippy --workspace --all-targets -- -D warnings` (if code files change)
3. `cargo test --workspace` (if code files change)
4. `cargo deny check` (if code files change)
