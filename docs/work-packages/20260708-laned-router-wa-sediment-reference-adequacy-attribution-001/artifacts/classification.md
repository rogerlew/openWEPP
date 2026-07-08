# Classification

Evidence mode: Ran.

## Verdict

Mechanism: `sediment response to sub-threshold routed-hydrograph shape perturbation`.

Promotion posture: `dx5` production mesh-policy promotion remains blocked.

## Evidence

The failing annual `tdep:4` value is not a broad daily water-magnitude or
active-router numerics miss:

- Year 4 has exactly one nonzero daily `tdep` delta day: sim day `1126`,
  julian `30`.
- The full annual `tdep` delta is `0.013223319145058476 kg` on that single day.
- Pass-parquet water magnitude operands on that day are identical:
  - `runvol` delta `0 m3`,
  - `sbrunv` delta `0 m3`,
  - `peakro` delta `0 m3/s`.
- Active trace source mass delta on the day is `0 m3`.
- Terminal routed outlet delta is `-0.003359397088388505 m3` on a
  `4594.312002957575 m3` reference terminal event.
- Terminal routed hourly-shape L1 is `0.0006352335679617539`.
- Max lane routed hourly-shape L1 is `0.0007414490157977821`.
- Tail fold and clamp are zero on the implicated day.
- Run-level uniform-shape rows and source-shape-degenerate rows do not increase
  (`10` and `1`, respectively).
- Run-level clamp totals are nonzero but roundoff-scale:
  `2.904645702250679e-13 m3` candidate and
  `6.413760918873332e-13 m3` reference.

## Rejected Mechanisms

`active-router numerics`: rejected. The active trace shows clean guard/counter
surfaces, zero day-1126 clamp, identical source mass, passed routed-water
surfaces, and no recurrence of the WA positivity class.

`daily water-magnitude sensitivity`: rejected as the primary mechanism.
Published pass-row water magnitudes are identical on the only implicated day,
and terminal routed outlet movement is sub-0.004 m3 on a 4594 m3 event.

`active-router numerics`: rejected. The implicated day has no clamp, no tail
fold, no source-mass delta, no source-shape fallback, and no uniform fallback.
The prior package recorded passed routed-water mesh-policy surfaces for the
same rung pair.

## Accepted Mechanism

The routed-hydrograph shape does move on the implicated day, and that movement
is a consumed water-timing input to the active erosion path. The accepted
classification is narrower: a low-denominator, single-event annual sediment
response to a routed-hydrograph timing/shape perturbation that is sub-threshold
on the current routed-water mesh-policy surfaces.

The annual reference denominator is only `0.5974836468326581 kg`; the one-third
absolute threshold implied by the current relative annual sediment rule is
`0.003983224312217721 kg`, and the observed single-day delta is
`0.013223319145058476 kg`.

This does not authorize an in-place tolerance widening. It does authorize a
narrow contract-first follow-on to adjudicate whether the annual pass-sediment
mesh-policy adequacy metric needs a predeclared low-mass/event-sensitive
condition, or whether the current strict annual relative rule remains binding
and `dx5` stays unpromoted.
