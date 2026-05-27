# 20260527-wshedimpl14-watershed-baseline-authoritative-end-to-end-comparator-lane-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL14 by implementing a baseline-authoritative
`openwepp-cli-watershed` comparator lane that validates topology dispatch,
branch execution, and watershed parquet publication against baseline fixture
authority, closing `GAP-SYSTEM-005`.

## Why This Package Exists
WSHED09/12/13 confirmed watershed conformance vectors and closed WS12 active-lane
parity, but program hold posture remained because system integration lacked a
baseline-authoritative end-to-end watershed comparator lane. This package closes
that comparator-lane gap in runner integration tests and canonical contract gap
posture.

## Scope
### Included
- Add a baseline-authoritative comparator lane in
  `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` that:
  - reads canonical baseline EBE authority fixture data from
    `/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/outputs/wepp_dcc52a6/ebe_pw0.txt`,
  - drives `openwepp-cli-watershed` end-to-end with a contract-valid HBP fixture
    seeded from baseline daily EBE signature values,
  - validates key continuity (`month`, `day_of_month`, `simulation_year`),
    topology dispatch (`sim_day_index`), branch publication continuity
    (`chan.out` peak vs `ebe` peak), and baseline-signature runoff/peak parity.
- Update canonical system contract gap posture for `GAP-SYSTEM-005`.
- Update contract registry note for `SC-SYSTEM-001`.
- Publish package evidence, reviews, verifications, gate results, disposition,
  and worker handoff.

### Explicitly Out of Scope
- Full watershed channel sediment process-parity migration (`GAP-SYSTEM-008`,
  `GAP-ROUTE-009`, `GAP-SED-006`).
- Replacement of current watershed sediment surrogate publication behavior with
  full `chnero/chnrt/detach` process execution.
- Non-`SC-SYSTEM-001` canonical contract redesign.

## Deliverables
1. `artifacts/wshedimpl14-watershed-validation-and-comparator-rerun-report.md`
2. `artifacts/wshedimpl14-hold-lift-decision-report.md`
3. `artifacts/wshedimpl14-contract-implementation-evidence.md`
4. `artifacts/wshedimpl14-contract-test-implementation-evidence.md`
5. `artifacts/wshedimpl14-preimplementation-contract-gate.md`
6. `artifacts/wshedimpl14-implementation-and-test-evidence.md`
7. `artifacts/wshedimpl14-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/wshedimpl14_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Confirm canonical comparator-lane authority and closure condition in
   `SC-SYSTEM-001` (`GAP-SYSTEM-005`).
2. Implement contract-derived comparator vectors in watershed CLI tests.
3. Record pre-implementation contract gate evidence.
4. Implement harness/test dependency edits and update canonical gap disposition.

## Autonomous Execution Intent (Required)
This package is execution-ready and executed end-to-end through disposition
without requesting additional user direction.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:` sections.

## Provenance and Authority Posture
- Canonical authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy provenance anchor defaults to
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Work-package artifacts are evidence, not replacement authority.
- No silent fallback masking for missing required comparator fixtures in
  production paths; comparator test lane remains fixture-aware and explicit.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl12-worker-handoff-immediate-next-actions-closure-001/artifacts/wshedimpl12-follow-on-package-specs.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl13-active-lane-15-function-parity-migration-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/outputs/wepp_dcc52a6/ebe_pw0.txt`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl14-watershed-baseline-authoritative-end-to-end-comparator-lane-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/Cargo.toml`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL14 authorization from WSHEDIMPL12 follow-on specs and
  WSHEDIMPL13 worker handoff.

### Phase B - Contract and comparator vector preparation
- Confirm and amend canonical `SC-SYSTEM-001` gap closure language for
  comparator-lane authority.
- Author contract-derived baseline comparator vector in watershed CLI tests.
- Record pre-implementation contract gate evidence.

### Phase C - Harness/test implementation
- Add required test dependency support in `openwepp-runner`.
- Implement baseline EBE row parsing + parquet row assertions in watershed CLI
  contract test suite.

### Phase D - Validation and governance evidence
- Run required gates:
  `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo deny check`.
- Update all package artifacts with truthful `Static`/`Ran` sections.

### Phase E - Disposition and handoff
- Publish final disposition and residual hold ownership.
- Hand off next immediate action (`WSHEDIMPL15`).

## Exit Criteria
- Baseline-authoritative comparator lane exists in
  `watershed_cli_behavior_contract` and passes.
- Lane validates baseline-seeded EBE signature (`peak_runoff`, `runoff_volume`)
  plus key/dispatch/publication continuity assertions.
- `SC-SYSTEM-001` updates `GAP-SYSTEM-005` to closed with executed evidence.
- Required workspace gates pass and are recorded.
- Worker handoff names residual blocker closure ownership.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: tests/docs/contract-gap updates only; no network or credential
  surface changes.

## Execution Outcome Summary
- Implemented WSHEDIMPL14 comparator lane by adding baseline-seeded
  end-to-end watershed CLI test coverage that parses baseline fixture authority
  and validates emitted parquet signatures and dispatch/publication continuity.
- Updated canonical `SC-SYSTEM-001` and registry notes to close
  `GAP-SYSTEM-005`.
- Program-level disposition remains `HOLD` because `GAP-SYSTEM-008`
  (channel sediment process parity migration) is still unresolved.
