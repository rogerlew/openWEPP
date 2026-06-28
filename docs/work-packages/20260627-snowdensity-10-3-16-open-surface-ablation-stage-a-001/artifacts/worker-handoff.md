# Worker Handoff

Evidence mode: Static/Ran.

Status: no immediate implementation handoff; candidate non-promoted.

## Current State

- `SC-SNOWFREEZE-001` v102 authorizes
  `coe_open_sublimation_stage_a_v1` as an explicit opt-in diagnostic candidate
  only.
- The runtime implementation subtracts bounded sublimation from snowpack SWE as
  vapor, records it in internal trace/conservation ledgers, and keeps it out of
  routed melt/liquid.
- The diagnostic run over `sleepers_south_field` and `harvard_open` closed as
  `NON-PROMOTION-STAGE-A-GATE-NOT-MET`.

## First Actionable Follow-Up

Do not tune the Stage A vapor coefficient or increase sublimation to chase the
open mass tail. A follow-up package should start from the failed guardrail:
reduce cap-limited open over-persistence without worsening under-persistence.
The likely next physics route is a contract-first Stage B package for a
surface-layer cold-content / surface-temperature mechanism, or another
open-exposure ablation mechanism with independent authority and the same
bidirectional gate.

## Protected Boundaries

Default activation, density cap, public output schema, fixtures, parser/runfile
user controls, compatibility runtime, Qwet/frzftp posture, and frost attribution
must remain unchanged until a later package explicitly reauthorizes them.
