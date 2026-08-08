# Native-Stratum Boundary

Status: complete

Evidence mode: Static

Native management owns an immutable stand definition. Vegetation owns the
evolving state instantiated from it. They are never the same object.

Required configuration records are: stand/model ID; unique stratum ID;
vertical rank; lifeform/material class; explicit topology membership;
versioned and SHA-256-bound parameter set; separately versioned initial-state
set; separately versioned soil-layer rooting profile; and provenance, units,
validity domain, ecosystem applicability, and claim limits for each field.

Horizontal support is represented by non-overlapping topology tiles whose
fractions sum to one. Each tile contains at most one stratum at a vertical rank
and may contain strata at different ranks. Stratum cover is the sum of its tile
fractions. Cover therefore closes within each rank, while vertically overlapping
rank covers may sum above one. Aggregate compatibility cover is the union of
occupied tiles, not the sum of stratum cover and not an independence formula.

Vertical ranks are deterministic top-to-bottom traversal order. Equal-height
ties retain explicit rank and stable ID. Inconsistent height/rank, duplicate
same-rank tile occupancy, missing topology, or cover non-closure fails without
perturbing biomass or cover.

Rooting requires an explicit non-negative soil-layer participation vector tied
to the hydrology layer identity. Root depth alone cannot substitute for it.
Dynamic redistribution/remapping is `AUTHORITY_MISSING` and no hidden profile
may be inferred.

The current management schema remains unchanged. Current aggregate `cancov`,
GSI phenology, and community geometry are not silently upgraded to this surface;
conversion requires a later explicit migration tool with equation, assumptions,
source values, version, and digest.
