# Architecture And Provider Freeze

Evidence class: `Static` at
`546bf150ad179e7ed3175b575805e9f874c14a75`.

## Scheduler seam

The real consumer is `DirectFrameExecutor`'s publication-stream day loop. The
immutable same-day seam is after `seed_day_frame` and
`apply_publication_day_input`, before `run_day_spans_hydrology`. At that point
repository climate/frost/hydrology inputs are present and native infiltration,
ET, root uptake, runoff and state commit have not executed.

Each actual lane input and its applied day frame are captured at this seam in
the original stateful lane-interleaved order. After the complete set exists,
the shadow executes 48 consecutive `1800 s` candidates only against its
isolated beginning hydrology. The higher-frequency provider cannot inspect the
post-native production frame. Production rows remain buffered until every
shadow day accepts.

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
inputs to 48 explicit interval receipts. The repository currently has no
authoritative provider for the complete snow-free interval receipt; daily
climate and rainfall breakpoints are insufficient to derive radiation,
humidity, canopy, thermal, and WB14 operands. Missing higher-frequency inputs
are therefore a live typed-unsupported closure blocker, not defaulted.

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
