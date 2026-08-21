# Review Disposition Correction

Status: `EXECUTED HOLD`

Evidence mode: `Static source audit supplied in the terminal review`

The prior local API-shape reviews and affected-crate tests established the
ordinary scheduler hook, surface-liquid/WB14 bridge, restart wrapper,
replay/failure plumbing, immutable publication boundary, and line-count
compliance. They did not establish the constitutive Stage-3/V11 path. The
following findings are accepted as closure-blocking:

1. `openwepp-runner` does not construct or custody the persistent attachment
   from run configuration, sealed forcing, Stage-3 state, and V11 state.
2. `DirectSnowStage3ShadowConfiguration` still carries event and live carrier
   operands that the coupled owners must produce internally.
3. `derive_shared_carrier()` is a projection/fixture: it hardcodes air
   properties, treats projected wind as conductance, names only `stage3-snow`,
   and supplies zero physical ledgers instead of consuming the V11 carrier.
4. `derive_terminal_event()` accepts a configured tick with zero tolerances and
   zero rates; it does not run the persistent Stage-3 terminal solver or
   neighboring admissible candidates.
5. `owner_set_after_sequential_consumers()` and
   `owner_set_from_live_frame()` hash `Debug` representations into synthetic
   owner payloads. The actual `DirectV11RealConsumerStack` is not invoked by
   the attachment.
6. The chronology stages the shadow after the full ordinary day has executed,
   so it does not perform snow-covered slabs, terminal transition, and the
   snow-free remainder inside one coupled half-hour parent interval.
7. Restart persists the configured attachment/receipt wrapper rather than the
   complete Stage-3, V11, LSE, BGC, soil-thermal, carrier, and coupled-time
   owner cursor state.
8. The principal ordinary-path test is a one-lane configured plumbing case;
   repository-backed positive scenarios for the required physical ledgers are
   missing.

The following remain genuinely complete as bounded infrastructure: old
request/executor methods are test-only, the scheduler hook is internal once an
attachment exists, the surface-liquid/WB14 API is invoked, replay and late
failure plumbing are tested, publication rows are immutable and post-commit,
the V9 files are below the line ceiling, and protected selectors/defaults/CoE
ownership/outputs/deployment were unchanged.

Required next increment, before Child 3:

- construct and custody the attachment from real run-owned sealed/staged state;
- call the actual Stage-3 terminal solver and reviewed coalescing authority;
- derive carrier inputs from live V11, Stage-3, LSE, and sealed exposure owners;
- invoke the actual `DirectV11RealConsumerStack` for pre/post-event slabs;
- replace debug payloads with canonical typed owner states/candidates;
- move the transaction inside the coupled parent interval;
- persist the complete owner set and accepted coupled-time cursor;
- add multi-lane and repository-backed nonzero physical scenarios, including
  restart; and
- reconcile `identity.lock.json` and stale contract-version assertions, then
  obtain fresh science, ownership, and Rust reviews of the constitutive path.

Child 3 remains prohibited. No Rust implementation or science-contract change
is made by this correction commit.
