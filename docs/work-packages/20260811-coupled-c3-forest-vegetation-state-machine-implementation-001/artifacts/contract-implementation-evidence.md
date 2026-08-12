# Contract Implementation Evidence

Status: `EXECUTING / Milestone 1 implemented; public E04 fail-closed`

Evidence mode: `Static + Ran`

Static: the public state is V2-only: shared stratum C/N and phenology state is
separate from exact `(stratum,tile)` occupancy lanes. Strict validation binds
the released V2 model, configuration digest, complete state digest, exact
occupancy/root identity, every occupancy field, pending transfers, and
transaction lineage.

Ran: focused tests cover exact two-tile/two-stratum state, duplicate/missing/
extra/wrong occupancy, all 15 lane fields in the state digest, layer order and
cardinality, unit spelling, V1 parser rejection, initial/prior transactions,
and every admitted migration branch. Public execution remains explicitly
fail-closed before E04; no E01--E22 public-path or commit claim is made.
