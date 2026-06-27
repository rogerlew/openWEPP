# Closeout

Evidence mode: Static + Ran.

Disposition: `SPRING-DENSIFICATION-NON-PROMOTION`

## Result

The candidate was implemented and run through the real direct-production WAT
path, but it does not meet the no-worse gate.

- Prior SNOWDENSITY-10.3.8 holding-capacity-only failures: `761`.
- Holding-capacity + existing `physics_bulk_density_compaction_v1`: `498`
  failures.
- Holding-capacity + new `physics_bulk_spring_densification_v1`: `502`
  failures.
- Paired surfaces improved/marginal/worse: `1 / 0 / 3`.
- Candidate snow-control remains blocked: `502/1415` paired rows fail.

The new wet-time acceleration is therefore retired as a promotion candidate.
The useful finding is that the existing density-compaction arm, when paired with
the holding-capacity melt/liquid correction, carries the stronger coupled WAT
signal.

## Mechanism

The non-promotion cause is over-densification into under-persistence, not a
failure of the 10.3.10 compaction diagnosis. The existing
`physics_bulk_density_compaction_v1` arm already consumed most of the available
compaction headroom. The spring add-on then pushed rows that were already at or
below observed depth into the opposite failure class.

- `harvard_hardwood`: `153 -> 156` failures with `0` compaction-only headroom
  and `64` candidate under-persistence rows.
- `harvard_open`: `138 -> 140` failures with only `3` compaction-only headroom
  rows and `35` candidate under-persistence rows.
- `sleepers_w9_hardwood`: unchanged failure count `57 -> 57`, but classified
  worse with `10` candidate under-persistence rows.
- `sleepers_south_field`: only paired improvement, `150 -> 149`, where
  compaction-only headroom remains largest (`15` rows).

The lesson for the queue is that compaction remains a validated lever, but the
rate/realization lever is exhausted for this bundle. Do not pursue another
wet-snow compaction acceleration without new external authority and a different
failure class.

## Boundary Status

- Default activation changed: no.
- Parser/runfile/user CLI selector added: no.
- Fixture inputs changed: no.
- Public output schema changed: no.
- Density cap changed: no.
- Observed depth/density consumed by runtime: no.
- Frost attribution authorized: no.

## Follow-Up

Do not promote `physics_bulk_spring_densification_v1`. The next package should
adjudicate the combined opt-in bundle already shown to improve the gate:
`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`, then
target the remaining `498` failures with a separate lever. Open-surface mass
depletion is the leading physics follow-on after bundle adjudication because the
10.3.10 cap-limited mass tail is concentrated in `harvard_open` and
`sleepers_south_field`. Patchy meltout remains a structural/non-target
classification unless a separate observation correspondence package makes it
verdict-bearing. The `522 kg m^-3` cap also remains an authority check, not a
hidden tuning knob: if ripe-snow authority supports a different cap, handle it
as its own contract-first package.
