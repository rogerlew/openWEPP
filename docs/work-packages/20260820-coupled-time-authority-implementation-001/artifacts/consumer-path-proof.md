# Consumer Path Proof

Status: implementation candidate complete

Evidence mode: Static + Ran

Producer: `openwepp-coupled-time` staged clock/transaction APIs.

In-memory handoff: `CoupledClockStateV1`, owner candidates, event transition,
additive restart, diagnostic reduction, and publication outbox.

Real reference consumer:
`openwepp-hillslope-orchestrator::coupled_time_reference::run_reference_parent_v1`.
It uses A+B before the event, atomically transfers B-to-C, uses A+C afterward,
restores mid-parent, finalizes the complete A+B+C set once, and exposes output
only after commit. Focused 2/2 tests prove the chronology and rollback.

Negative proof: no vegetation, snow, LSE, hydrology, Lane D, Richards,
DirectV10 restart, selector, default, or production publication path imports the
new crate. This package proves a real orchestrator reference seam, not a physical
production adopter or cutover.
