# Worker Handoff

Evidence label: Static/Ran.

Status: `LOCAL-HOLD-HANDOFF`

Current state:

- Target file:
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs`
  is restored to scaffold/baseline state.
- Package status:
  `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-ADR0021-COVERAGE-BLOCKER`.

First actionable follow-on:

- Create and execute a dedicated WS20/WS21 channel sediment routing
  test-enhancement package before retrying CQR on this module.

Follow-on acceptance needs:

- Branch-sensitive coverage evidence for case34/case4/case3 route paths.
- ADR-0021 science-tier line and region threshold evidence.
- Per-function region-floor disposition.
- Obligation-to-test binding against `SC-ROUTE-001` and `SC-SED-001`.
- Only after coverage closure, reattempt behavior-preserving CRAP
  decomposition for this target.

Do not:

- Reuse the provisional CQR implementation as completion evidence.
- Claim the interrupted full nextest rerun as a passing gate.
- Restart CQR on this module until the coverage blocker is planned or lifted.
