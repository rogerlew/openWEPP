# Architecture And Provider Freeze

Evidence class: `Static` at
`546bf150ad179e7ed3175b575805e9f874c14a75`.

## Scheduler seam

The real consumer is `DirectFrameExecutor`'s publication-stream day loop. The
immutable same-day seam is after `seed_day_frame` and
`apply_publication_day_input`, before `run_day_spans_hydrology`. At that point
repository climate/frost/hydrology inputs are present and native infiltration,
ET, root uptake, runoff and state commit have not executed.

The shadow executes once for the complete OFE set at this seam, never once per
lane. It retains 48 consecutive `1800 s` candidates on isolated state. Native
hydrology then executes its unchanged daily path from the unchanged production
frame.

## Complete owner set

1. V9 vegetation configuration/state and canopy forcing receipt;
2. production hydrology lane/layer snapshot and persistent default-off surface
   liquid owner;
3. LSE configuration/state and per-tile forcing;
4. shared OFE soil-thermal snapshot;
5. biogeochemistry beginning state and nitrogen arbiter;
6. pending transaction envelopes and complete shadow replacement record.

## Provider posture

The attachment accepts typed repository-provider records, not caller-injected
solver trials, bindings, raw residuals or finalized candidates. Provider
projection validates exact day/OFE/tile/layer identity and maps daily repository
inputs into 48 explicit interval receipts. Missing higher-frequency inputs are
unsupported rather than defaulted.

## Shared soil-thermal continuation

Child 3 emits one candidate per tile from the same immutable OFE soil column.
For continuation, Child 4 must independently aggregate each layer's
tile-fraction-weighted storage and accepted infiltration credits exactly once,
then derive the one shared ending enthalpy/temperature snapshot and validate it
against the complete candidate set. Selecting a tile, copying a producer
residual, retaining multiple mutable OFE soil temperatures or omitting a tile
is forbidden.

## Exclusions

No runner selector, environment toggle, production default, production-state
commit, output row/manifest field, snow terminal payload, soil-BGC
transformation, deployment, calibration or empirical claim is authorized.
