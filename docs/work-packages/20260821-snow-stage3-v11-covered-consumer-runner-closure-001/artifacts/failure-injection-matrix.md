# Failure-injection matrix

Required poison cases include carrier failure, invalid forcing identity,
receiver-topology mismatch, `ERR-CT-021` retry, terminal parcel replay, and
partial owner installation. Each must leave beginning owners, cursors, and
receipt chain unchanged.

Status: `PASS for the bounded persistent physical-custody checkpoint; broader
terminal/restart matrix remains open`.

Ran: live parent failure injection after the physical outcome ledger, after an
accepted subslab, and after the final owner join preserves the exact parent,
consumer (including nested seven-owner state), clock, and Stage-3 beginnings.
The same live rainy transaction injects typed
`PrecipitationReceiptRejected(1)` and `SnowSoilHeatReceiptRejected(1)` failures
after candidate construction and proves the identical complete rollback.
Precipitation manifests reject coherently resealed missing/extra snowfall,
open-rain, and vegetation-route parcels. Snow--soil receipts reject wrong
typed node, stale/one-bit installed snow or soil identities, sign, support,
and candidate substitution before parent acceptance. The complete affected
suite exercises fixed-point nonconvergence and unchanged rollback.

Still open here: terminal-event parcel replay, downstream terminal-liquid
consumer failures, restart-wire poisons, and the production runner's complete
48-support installation matrix.
