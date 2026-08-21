# Actual Consumer Path Proof

Status: `PARTIAL / TYPED OPT-IN ENDPOINT PROVED; ORDINARY RUNNER HOLD`

Static path: `DirectFrameExecutor::run_publication_stream_with_day_hook` in
`direct_runtime/03_executor.rs` seeds a `DirectDayFrame`, runs ordinary
hydrology/storage phases, constructs the publication row, publishes transfers,
and only then calls `DirectRunFrame::commit_day_frame`. The implementation adds
an opt-in wrapper on this same loop. Its hook stages the Child 2C request at
`ProjectedDay`, and its commit hook accepts the staged complete-owner candidate
only after the real row/transfer/day commit succeeds.

The follow-on adds a typed owner-executor path. The new
`DirectV11SnowStage3OwnerExecutor` uses `DirectV11RealConsumerStack`, invokes
`execute_v11_segment` over the exact accepted slab, receives the produced V11
owner envelopes (including LSE, BGC, and soil-thermal), and only exposes them to
the handoff runtime through `SnowStage3OwnerExecutionReceipt`. Its pending
parent/shadow is committed only after the scheduler commits the handoff runtime.
The focused endpoint test proves this concrete stage/commit path.

The complete owner envelope is the released seven-owner manifest:
`vegetation`, `snow`, `land_surface_energy`, `surface_liquid`, `hydrology`,
`bgc`, and `soil_thermal`. The persisted V3 seam serializes the same staged
cursor, complete receipt history, event ordinal, and owner digest through
canonical JSON. The terminal receipt now retains parent/segment identity,
candidate-set digest, and accepted tie rank; runtime admission rejects a
non-contiguous or body/digest-mismatched history.

The owner-aware scheduler clones the frame, runtime, and typed executor,
buffers rows until the candidate transaction succeeds, stages the typed owner
execution before the handoff runtime, and commits the typed owner after the
runtime commit. Static negative proof shows the production selector/default and
CoE owner remain unchanged.

The normal hillslope runner still calls the ordinary interleaved publication
stream and does not construct the typed V11 parent/slab/owner bundle; therefore
this is an opt-in typed endpoint proof, not ordinary production-runner closure.
The new carrier-to-event participant join and successor-participant guard are
proved by the package test, but terminal liquid is still not passed into the
real surface-liquid owner. Durable publication/outbox atomicity and ordinary
runner binding remain unresolved. The normal runner has no authoritative typed
V11/LSE/BGC/soil-thermal owner input or half-hour provider/GSI cursor from
which to construct the receiver; the package therefore ends with a precise
typed-input boundary rather than fabricated owner state. The package stays
`HOLD` until that authority and the remaining publication/contract gates are
closed.
