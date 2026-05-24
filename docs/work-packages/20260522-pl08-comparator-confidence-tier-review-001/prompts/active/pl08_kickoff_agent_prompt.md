# PL08 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/package.md


You are executing `20260522-pl08-comparator-confidence-tier-review-001`.

Objective:
Run comparator confidence-tier review for PL scope after PL05/PL06/PL07, with
Tier-A single-OFE daily water-balance and plant/residue parity investigation.

Constraints:
- Preserve architecture-first + contract-first posture.
- Apply ADR-0011 confidence-tier semantics and numerics policy
  (semantic parity, not bitwise parity).
- Use pinned baseline authority from ADR-0012 by default.
- Tier-A unresolved blocking deltas remain `HOLD`; Tier-B hourly/watershed
  divergences are investigation signals unless explicitly escalated.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; do not suppress unresolved parity blockers.

Required outputs:
- `artifacts/single-ofe-daily-water-balance-comparator-delta-report.md`
- `artifacts/plant-residue-parity-investigation.md`
- `artifacts/comparator-run-provenance-manifest.md`
- `artifacts/comparator-confidence-tier-disposition.md`
- `artifacts/semantic-parity-direction-assessment.md`
- `artifacts/pl08_disposition.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
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
