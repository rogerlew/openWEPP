# Disposition

Evidence label: Static/Ran.

Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-ADR0021-COVERAGE-BLOCKER`

Disposition: local target hold.

Accepted findings:

- `review_agent_a` Finding 1: ADR-0021 science-tier coverage gate is not
  closable as complete.
- `review_agent_a` Finding 2: gate artifacts were stale/inconsistent.
- `review_agent_a` Finding 3: coverage/CRAP artifact provenance was not stable
  enough for completion.
- `review_agent_b` Finding 1: provisional characterization did not cover key
  refactored case34/case4 behavior.
- `review_agent_b` Finding 2: ADR-0021 coverage closure was not met.
- `review_agent_b` Finding 3: gate evidence was stale/inconsistent.

Actions taken:

- Interrupted the stale full nextest rerun; recorded exit `130`.
- Rolled back target Rust implementation/test edits to scaffold state.
- Added `artifacts/hold-legitimacy-audit.md`.
- Updated package artifacts to stop claiming completion-grade CRAP closure.

Rejected findings:

- None.

Deferred/follow-up findings:

- Dedicated WS20/WS21 channel sediment routing test-enhancement package needed
  before reattempting CQR on this target.
