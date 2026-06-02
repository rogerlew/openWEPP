# HPHYS0241 Disposition

Status: hold
Evidence mode: static + ran

Decision: `HOLD_PENDING_HPHYS0242`

Static: HPHYS0241 implemented explicit MOFE hourly carry-array surfaces for
upstream saturation carry, upstream lateral carry, current lateral carry,
current saturation carry publication, copy-forward state, aggregate carryover
publication, runner manifest provenance, and watershed contributor intake
validation.

Static: Dispatch Group C no longer relies on implicit daily aggregate runon for
active multi-OFE carry-array execution. `wb12_runon_input` is compatibility-only
when `mofe_hourly_carry_arrays_enabled = 1`; present aggregate
`wb12_runoff_carryover` must match explicit arrays.

Static: positive top-layer saturation excess that would require cadence-accurate
hourly `ui_SCrunf(ii)` distribution is intentionally fail-closed rather than
approximated. This keeps the HPHYS stream in HOLD for
`20260601-hphys0242-wb14-wb12-cadence-ordering-closure-001`.

Ran: all required gates passed; see `gate-results.md`.

Disposition summary:

- MEASURE-HP241-001: satisfied for canonical contract authority.
- MEASURE-HP241-002: satisfied for explicit upstream/lateral carry and metadata
  handoff; material positive saturation carry remains fail-closed pending
  HPHYS0242 cadence closure.
- MEASURE-HP241-003: satisfied for array continuity and malformed upstream
  payload rejection vectors.
- MEASURE-HP241-004: satisfied for runner manifest and watershed contributor
  metadata validation.
- MEASURE-HP241-005: satisfied; required gates passed.

Next package: execute HPHYS0242 to close WB14/WB12 cadence/order dependencies
and decide final HPHYS HOLD/GO posture.
