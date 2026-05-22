# SR07 Kickoff Agent Prompt

You are executing `20260522-sr07-comparator-confidence-tier-delta-review-001`.

Objective:
Run legacy comparator review for Tier-A single-OFE daily water-balance
surfaces after SR06 and disposition resulting deltas to validate
semantic-parity direction.

Constraints:
- Preserve architecture-first + contract-first posture.
- Treat comparator outcomes using ADR-0011 confidence tiers and numerics policy
  (semantic parity, not bitwise parity).
- Use pinned baseline authority from ADR-0012 by default.
- No silent down-classification of Tier-A blocking deltas.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved high-severity Tier-A deltas remain
  `HOLD`.

Required outputs:
- `artifacts/single-ofe-daily-water-balance-comparator-delta-report.md`
- `artifacts/comparator-run-provenance-manifest.md`
- `artifacts/comparator-confidence-tier-disposition.md`
- `artifacts/semantic-parity-direction-assessment.md`
- `artifacts/sr07_disposition.md`
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
