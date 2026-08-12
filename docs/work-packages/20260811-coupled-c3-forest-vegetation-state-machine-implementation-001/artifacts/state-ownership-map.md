# State Ownership Map

Status: `V2 ownership amendment active`

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
