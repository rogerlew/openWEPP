# Coverage Closure

Evidence label: Static.

Status: `QUEUED`

ADR-0021 tier: `science`, because the target owns WS12 impoundment
stage-discharge, stage-area continuity, and adaptive integration behavior bound
to `SC-IMPOUND-001`.

Closure rule:

- If characterization tests are added or materially changed, record line and
  region coverage status for the target, per-function 75% region-floor status or
  disposition, and obligation-to-test binding before completion.
- LCOV does not provide region coverage; if no region-capable report is
  available, record the gap explicitly and route to hold when the ADR gate
  cannot be satisfied.

Current scaffold status:

- Baseline LCOV: `LF:484`, `LH:262`, `54.13223140495868%`.
- Characterization not yet added.
- Coverage closure pending implementation.
