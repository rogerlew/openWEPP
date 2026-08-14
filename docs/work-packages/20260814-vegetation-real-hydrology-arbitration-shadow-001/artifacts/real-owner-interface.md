# Real Owner Interface

Evidence class: `Static + Ran`

The real-owner adapter holds a full private clone of the production run/lane
envelope and the freshly seeded `DirectDayFrame` used by the scheduler. It
exposes only an immutable typed arbitration snapshot. The snapshot binds:

- run, hillslope, OFE/lane, day and transaction identities;
- exact ordered configured layer identities;
- every production layer field and the aggregate water and transfer state used
  by this arbitration boundary;
- OFE area and amount basis;
- a deterministic beginning fingerprint.

The implemented V8-precursor root request identity is
`(transaction, OFE, occupancy requester, soil layer, OFE-ground interval
basis)`. It does not claim the complete V8 `surface_class`, explicit resource
identity, or optional-layer form. Authorization groups equal-status requests
only by the exact
`(OFE, layer)` supply identity while preserving each request key. The owner
returns maximum authorizations and typed reasons, then constructs a full cloned
ending candidate by debiting finalized use only.

The production-owned `authorize_direct_layer_withdrawals()` endpoint reads the
seeded day-frame layers and delegates proportional same-source arithmetic to
the one dependency-neutral kernel primitive. The production and shadow paths
also share `apply_direct_finalized_layer_liquid_debit()`. No copied
authorization or debit algorithm remains.

The public V7 bridge additionally requires exact equality for interval,
candidate transaction, ordered layer identity, beginning liquid and frozen
facts. Root accessibility is derived from the same vegetation forcing rather
than accepted as a second caller assertion.

This child intentionally exposes a single-OFE executable bridge. Multi-lane
owner arbitration is tested at the source-key boundary, but routed multi-OFE
vegetation/ground coordination and complete V8 source identity are typed
unsupported until the Child-4 scheduler consumer owns the shared batch.
