# Final Disposition

Evidence mode: Static + Ran.

Status: `EXECUTED-COMPLETE`.

## Verdict

`TIMESTEP-POLICY-ARTIFACT-CLOSED`.

The `mn_corn_h4` day-792 lane-1 fixed-300 target-`dx` adequacy miss is a
timestep-policy artifact, not a routed-shape tolerance failure and not a
production-default ratification.

Key evidence:

- fixed-300 `dx1p25` vs `dx0p625` shape L1:
  `0.02094494047849004`
- threshold: `0.016666666666666666`
- shared-75 `dx1p25` vs `dx0p625` shape L1:
  `0.002982804005304084`
- source/upstream/clamp/limiter/bin controls: clean

## Implemented

- Added trace-gated diagnostic `OPENWEPP_LANED_ACTIVE_MAX_DT_S`.
- Bounded diagnostic max substep to finite positive `<= 300`.
- Kept production max substep at `LANED_ACTIVE_MAX_DT_S = 300`.
- Serialized `max_dt_s` into runtime active summary, manifest provenance,
  trace rows, and trace detail.
- Added compact committed analyzer inputs so replay does not require ignored
  raw run trees.
- Amended `SC-OFEROUTE-001` to rev 43, requiring coupled space-time evidence
  before renewed target-`dx` production promotion.

## Non-Changes

- No target-`dx` production default flip.
- No routed-shape tolerance widening.
- No source/coefficient tuning.
- No shadow mesh change.
- No hybrid revival.
- No default/off behavior change.

## Gates

All required local gates passed and are recorded in `artifacts/gate-results.md`:

- `git diff --check`
- Markdown lint for package, contract, README, and roadmap
- contract/profile/BEI checks
- focused selector and active-router tests
- six-rung `mn_corn_h4` timestep ladder
- analyzer replay from raw traces and compact committed inputs
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

## Reviews

Dual review found two substantive blockers:

- missing runtime-owned `max_dt_s` evidence metadata
- analyzer replay depending on ignored raw traces

Both were fixed and dispositioned. Dual verification found no remaining
Rust/contract blocker after closure artifacts and gate vocabulary were fixed.

## Handoff

Next package:

`20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001`

Purpose: renew fidelity-first `dx5` production mesh-policy ratification under
`SC-OFEROUTE-001` rev 43 using coupled same-`dt` spatial and same-`dx`
timestep-refinement evidence.
