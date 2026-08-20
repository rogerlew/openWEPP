# Contract implementation evidence

Status: complete / bounded default-off authority reviewed GO

Evidence mode: Static

The contract-first slice adds fresh `terminal_receiver_v1` authority without
altering the historical evaluation-only semantics of `INV-SNOWENERGY-034` or
`INV-SNOWFREEZE-101`:

- `SC-SNOWENERGY-001` v13 adds `INV-SNOWENERGY-035`, exact terminal-liquid
  composition, zero-sensible-enthalpy parcel custody, terminal-unallocated-
  energy rejection, and no-post-event-snow rules.
- `SC-SNOWFREEZE-001` v136 adds `INV-SNOWFREEZE-102`, absolute half-open wall
  support, total error precedence, in-progress adaptive/bisection restart, and
  atomic all-owner chronology.
- `SC-LANDSURFACEENERGY-001` v5 adds `INV-LANDSURFACEENERGY-114/115` for actual
  surface selection and full receiver flux rebuild on `[wall_t*,wall_end)`.
- `SC-SURFACELIQUID-001` v7 adds `INV-SURFACELIQUID-010/011`, exact-one typed
  ingress and a tagged variable-duration call to the existing shared Mein-
  Larsen/Green-Ampt WB14 transition. The ordinary 48 x 1800 s path stays exact.
- `SC-VEGETATIONTRANSACTION-001` v3 adds `INV-VEGTRANSACTION-008`, the coupled
  Stage 3 + V10/LSE/surface/soil/frost/routing candidate commit and rollback.

Every new invariant is present in the established invariant and guard/error
tables with unit/tolerance/test/gap bindings. Runon remains receiver-side;
snow-side runon is prohibited because it has no admitted snow authority.
Positive out-of-tolerance terminal-unallocated energy rejects the handoff and
retains no recipient.

Independent science review and hydrology/ownership review initially returned
HOLD. All accepted findings were remediated through three exact-byte review
cycles. Final science and ownership verdicts are GO for this bounded contract-
first gate while explicitly retaining overall snow-contract draft lifecycle
and every carrier/efficacy/qualification/production/cutover hold.
