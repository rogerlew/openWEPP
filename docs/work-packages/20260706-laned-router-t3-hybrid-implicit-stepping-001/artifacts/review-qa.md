# Review - QA and Governance

Status: **EXECUTED** (2026-07-07).

Evidence mode: **Static**. Reviewed current `main` at `64d57f51`, execution
commit `bd64d2c8`, package artifacts, contract posture, and selected code
anchors. Subagent evidence: `rust_qa_reviewer`. No tests or gates were run by
this review.

Package verdict: **GO-WITH-AMENDMENTS** as experimental opt-in evidence, but
not closure-grade. The QA lane finds no High issues, but the package needs a
gate artifact, selector-hygiene repair, and direct Filippov regression before
rev-28 can be treated as settled.

## Findings

### Medium

- **T3-QA-M1 - Gate evidence is not closure-grade yet.**
  `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/package.md:64`
  requires `artifacts/gate-results.md`, but that artifact is absent.
  `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/worker-handoff.md:3`
  says "all closure gates green", while
  `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i2-hybrid-evidence.md:39`
  records focused tests, clippy, and full nextest only. I found no gate table
  classifying `cargo fmt --check`, `cargo deny check`, markdown/doc lint, or
  contract/profile checks as `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`.

- **T3-QA-M2 - Filippov closure needs a direct regression vector.**
  The high-risk path at
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:253`
  and
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:347`
  can commit a mass-exact jump. Existing unit vectors cover adjacent
  properties, but not the direct LOW -> HIGH -> Filippov path.

- **T3-QA-M3 - T3 selector hygiene is incomplete in the H2637 harness.**
  `tests/integration/laned_shadow_h2637.rs:1` says helpers neutralize both
  Lane D selector vars, and
  `tests/integration/laned_shadow_h2637.rs:243` clears only active/shadow for
  `run_h2637_native_active`. The new
  `OPENWEPP_LANED_ACTIVE_IMPLICIT` selector from
  `crates/openwepp-runner/src/hillslope/laned_active.rs:19` is not cleared.
  An inherited `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` can make a "plain active"
  rev-27 evidence leg run the hybrid path, weakening no-perturbation evidence.

## Checks

- T3 posture is honest in canonical authority:
  `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:480`
  labels `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` as EXPERIMENTAL and the fidelity
  tolerances as UNRATIFIED.
- The package evidence also records the strict-rule bottom line and follow-on
  limits in
  `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i2-hybrid-evidence.md:43`.
- D15A technical re-check passed from the QA lane. Its remaining governance
  amendment is recorded in the D15A Codex re-check section.
