# Restart schema and equivalence

Status: `NOT IMPLEMENTED / BLOCKED`.

`Static:` Existing persisted restart code was updated only to initialize the
new in-memory attachment field as `None`; the released V10 Restart V1,
coupled-time Restart V2, V11 Restart V3, and DirectHydrologyRestartV1 bytes
remain unchanged. This is compatibility evidence, not an outer restart.

`Static:` The required additive
`OPENWEPP_STAGE3_V11_COUPLED_RESTART_V1` envelope is absent. It does not yet
persist complete Stage-3 lanes, V11/real-consumer state, partial WB14 and
surface-liquid custody, provider cursor, active segment, event proposal and
accepted event, parcel consumed marker, owner receipts, or publication
reduction. Before/at/after-event equivalence and no-replay are therefore not
run and cannot be claimed.
