# State Ownership Map

Status: `MILESTONE 1 PASS / V2 ownership structure implemented`

Vegetation solely owns one canopy-liquid and coupled warm-start lane per valid
`(stratum_id,tile_id)` occupancy, plus shared stratum phenology, geometry, and
six-tissue C/N pools. The diagnostic water state owns per-layer soil water; BGC
owns independent layer/species mineral N and typed litter/CWD receipts; the
energy owner independently reconstructs occupancy and weighted-stand operands;
the orchestrator owns transaction identity and the one atomic commit. Water
identity includes transaction, owner, occupancy, layer, resource, and amount
basis through request, authorization, finalized use, debit, and receipt.

The thirteen historical failure injections remain regression evidence. V2 must
extend byte-identical rollback to every occupancy lane, shared C/N state, all
owner candidates, pending transfers, and transaction identity.

Ran: `CoupledOwnedState` contains only typed shared-stratum and occupancy maps.
Every mutable canopy-liquid and numerical warm-start field exists solely in an
`OccupancyState` lane. Exact configuration-derived occupancy equality,
root-layer ordering, model/configuration/state digests, and initial/prior
transaction lineage pass focused tests. No V2 execution path can access the
historical shared liquid or hydraulic warm starts; those fields remain only in
the explicitly named V1 migration DTO.
