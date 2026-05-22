# ARCH21 Kickoff Agent Prompt

You are executing `20260522-arch21-architecture-review-re-closeout-001`.

Objectives:
1. Reconcile closure evidence for `CRF-001..010` from ARCH15..ARCH20.
2. Validate whether ARCH14 hold can be released.
3. Replay and record workspace gates for ratification evidence.
4. Produce explicit hold-release decision and blocker/follow-on mapping.
5. Complete dual review/disposition/verification artifacts.

Constraints:
- Correctness over completion: do not claim closure without direct evidence.
- Preserve mandatory architecture direction: typed kernel seam and
  unit-boundary wiring remain required.
- Use truthfulness posture (`Static:` vs `Ran:`) in all artifacts.
- If any high-severity blocker remains unresolved, keep disposition `HOLD`.

Required outputs:
- `artifacts/crf-closure-evidence-matrix.md`
- `artifacts/arch14-hold-release-decision-record.md`
- `artifacts/arch21-open-blockers-and-follow-ons.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/arch21_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
