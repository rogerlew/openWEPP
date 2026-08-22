# Runner-owned 48-support capability

Required shape:

```text
PreparedStage3V11DayV1 {
    day_index,
    accepted_gsi_receipt,
    beginning_provider_cursor,
    ending_provider_cursor,
    supports: [PreparedStage3V11SupportV1; 48],
}
```

Each support must bind the exact half-open time support, sealed atmospheric
receipt, precipitation parcels/phase, destination/OFE/tile identity, per-lane
Stage-3 and V11 forcing, exposure identity, WB14 and receiver topology, and
forcing digest. It must not contain event time, ending owners, live
conductance, terminal parcel, or owner candidates.

Status: `IN PROGRESS / capability sealing and destination-complete provider
bind landed; persistent covered adopter is proven for one support; runner
physical support builder and complete day installation remain open`.

`Static:` `PreparedStage3V11DayV1::bind_provider_day` now admits only 48
supports, returns `ValidatedPreparedStage3V11DayV1`, and binds every support
to the provider's GSI receipt, interval receipt digest, precipitation parcel
list, WB14 identity, complete destination set, and exact half-open nanosecond
bounds. The closure path rechecks the prepared beginning GSI state, beginning
cursor, sequential provider day, lane-to-OFE destination topology, and
committed provider destination topology before execution. The provider
GSI/cursor owners are installed only on the cloned candidate after all 48
coupled supports succeed.

`Static:` The current surface still requires the runner to assemble the
physical `DirectActiveSnowPartitionInputs`, per-lane Stage-3 forcing, and V11
interval from live owners. No completed daily result is accepted, but the
runner-owned physical support construction is not yet closed.

`Static:` Provider day-relative interval receipts are projected into
run-relative supports before binding: day 0 covers `[0, 86,400 s)` and day 1
starts at `86,400 s`. Each support's parent forcing identity is derived from
its ordered sealed provider/GSI, exposure, WB14, precipitation, and lane/OFE/
tile receipt set. That digest is the actual ParentAuthority, StepConstraint,
and CoupledClockState forcing identity; no future parent is preconstructed
with the static context's repeated digest.

`Static:` Covered support drafts additionally carry the distinct covered-V11
segment input, Stage-3 support/configuration projections, and sealed shared
carrier configuration. The runner has not yet been given authority to derive
those drafts internally from its staged owners and provider clock.
