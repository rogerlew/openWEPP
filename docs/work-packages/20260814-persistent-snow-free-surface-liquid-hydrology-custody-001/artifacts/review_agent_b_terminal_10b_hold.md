# Hydrology And Ownership Review — `10b914da1`

Evidence: `Static` plus focused `Ran` evidence.

Verdict: `HOLD`.

The reviewer found one material public-error-context defect. Independent
receiver closure retained transaction, owner, OFE and tile identity but omitted
the uniquely configured `surface_id` and `source_id` from applicable E003,
E010 and E011 failures. `parcel_id` is correctly absent for aggregate receiver
equations.

The finding is accepted. Receiver operands now freeze the authoritative
configured `(OFE, tile, surface, source)` mapping, bind it into the canonical
digest and use it for closure, topology, join, arithmetic and atomic-envelope
errors without synthesizing identifiers.

Ran on the reviewed bytes:

- surface-liquid/receiver/real-hydrology library selection: 97/97 passed;
- unified LSE/real-hydrology integration: 39/39 passed; and
- custody authority contract: 10/10 passed.

All other reviewed mass, enthalpy, WB14, D/A/F, ingress, rollback and production
isolation surfaces passed. Fresh exact-byte review is required after the
correction is committed.
