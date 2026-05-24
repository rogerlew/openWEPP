# SR03 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-sr03-soil-runtime-seam-expansion-001/package.md


You are executing `20260522-sr03-soil-runtime-seam-expansion-001`.

Objective:
Expand the soil parser-to-runtime seam for hillslope orchestration from the
minimal seed surface to contracted layer/profile runtime surfaces required by
soil and hydrology consumers.

Constraints:
- Preserve architecture-first + contract-first posture.
- Preserve canonical soil symbol continuity (`solthk`, `dg`, `thetdr`,
  `thetfc`, `nsl`, `ssc`, and required layer/profile surfaces) with explicit
  alias mapping where boundary names differ.
- No silent defaults or fallback wrappers for missing required soil inputs.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved high-severity seam ambiguity remains
  `HOLD`.

Required outputs:
- `artifacts/soil-runtime-seam-contract.md`
- `artifacts/soil-runtime-builder-implementation-evidence.md`
- `artifacts/soil-runtime-consumer-coverage-matrix.md`
- `artifacts/sr03_disposition.md`
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
