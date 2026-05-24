# SR02 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-sr02-slope-runtime-seam-contract-and-builder-001/package.md


You are executing `20260522-sr02-slope-runtime-seam-contract-and-builder-001`.

Objective:
Implement slope parser-to-runtime seam ownership for hillslope orchestration
with typed projection and guard behavior aligned to SR01 boundary decisions.

Constraints:
- Preserve architecture-first + contract-first posture.
- Preserve canonical symbol continuity (`slplen`, `nslpts`, `xinput`,
  `slpinp`, `avgslp`) with explicit alias mapping where boundary names differ.
- No silent defaults or fallback wrappers for missing required slope inputs.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved high-severity seam ambiguity remains
  `HOLD`.

Required outputs:
- `artifacts/slope-runtime-seam-contract.md`
- `artifacts/slope-runtime-builder-implementation-evidence.md`
- `artifacts/sr02_disposition.md`
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
