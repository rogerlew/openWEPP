# Surface receiver topology

Status: `PARTIAL TOPOLOGY VALIDATION / CONSUMER BLOCKED`.

`Static:` `DirectSnowStage3V11StaticContext::validate` requires nonempty
receiver records and `validate_receiver_topology` requires each OFE's tile
fractions to sum to one. `terminal_parcels` binds the source lane through
`ofe_bindings`, filters records by that OFE, and maps mass to the configured
tile-ground basis. This removes the old one-record restriction.

`Static:` The mapping is not yet consumed by the real surface-liquid owner,
and no current scenario proves open mineral plus forest-litter tiles across
multiple OFEs/lanes. Receiver construction is therefore increment evidence,
not exact-once publication closure.
