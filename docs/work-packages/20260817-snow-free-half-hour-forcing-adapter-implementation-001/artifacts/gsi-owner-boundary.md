# Explicit GSI and forcing owner boundary

Status: `implementation PASS / broader restart dependency pending`

Implemented stable explicit owners:

- `DirectGsiOwnerConfigurationV1` binds schema, owner, parameters, repository
  latitude, and canonical configuration digest;
- `DirectGsiOwnerStateV1` binds oldest-first FIFO history, optional last date,
  and state digest;
- `DirectGsiDailyReceiptV1` now binds owner/run/day/source-climate,
  configuration, beginning/ending state, forcing, result, and receipt digests.

The closure-eligible `DirectV10RealConsumerShadow::try_new` now requires an
explicit GSI owner configuration, GSI beginning state, static forcing
configuration, and provider cursor. It no longer constructs generalized
parameters, an empty GSI owner, or a default cursor. Raw V10 projection and
day-execution methods are private; the public day operation is
`execute_prepared_gsi_day`.

`prepare_snow_free_gsi_day_from_repository` selects one repository day and
derives Tmin, canonical mean daily VPD, repository latitude, year/ordinal day,
and the source-climate digest from that same selection. It validates exact
run/day/source/configuration/latitude joins before returning staged GSI and
cursor transitions plus validated forcing receipts. Caller-completed
`GsiDailyForcing` no longer reaches this closure route.

Cursor restoration now accepts the static provider configuration and validates
its canonical digest. Daily GSI values and receipt digests remain outside that
static cursor identity.

Ran:

- affected orchestrator `cargo check`: PASS;
- forcing-adapter contract: PASS, 6/6, Nextest run
  `7252e2f0-610f-468e-958e-6d82ccab0b14`;
- zero-radiation repository-derived prepared day: PASS, 48/48;
- positive-radiation repository-derived prepared day: PASS, 48/48;
- repository-derived Child-4 forcing-type consumption: PASS, 48/48;
- downstream-failure all-owner rollback: PASS.

An earlier forcing-adapter run failed 2/6 after the receipt configuration
identity intentionally changed from a bare-parameter digest to the complete
owner-configuration digest. The tests were migrated to the repository-derived
owner route; the failure was semantic migration feedback, not discarded or
rerun unchanged.
