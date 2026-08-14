# Stage 0 Question Disposition

Evidence class: `Static`

1. The real water owner is `DirectFrameExecutor::run_day_spans_hydrology`
   operating on a `DirectDayFrame` and committing into `DirectLaneFrame`.
2. Vegetation and ground can share an immutable clone after day input
   projection and before the hydrology span.
3. Infiltration and runoff have not occurred at that point.
4. Persistent water is layer liquid/frozen storage; canopy release, runon,
   depression and residue interception are interval operands. No persistent
   snow-free litter/pond store exists.
5. Both legacy R4N surface ET and root uptake must be bypassed inside the
   shadow clone.
6. Day/lane/run frames clone without semantic loss in memory; builder state and
   complete restart state require a new versioned snapshot.
7. Hydrology is local OFE depth; each lane area is `fwidth * slplen`. V7
   tile-to-stand conversion remains internal to its topology and must join the
   OFE basis exactly once.
8. Multi-OFE routing carries local/upstream depths using area ratios. No
   temperature/enthalpy lineage currently accompanies runon.
9. No soil/frost owner accepts `-G`; frost instead computes its own flux from
   `surtmp` and resistance.
10. V7 supplies terminal direct/diffuse VIS/NIR radiation. Other required
    ground forcing is absent or non-authoritative: atmospheric/canopy longwave
    separation, runon enthalpy, thermal substrate identity and ground
    roughness/surface class. Harder--Pomeroy hydrometeor temperature is an
    existing candidate for precipitation temperature, not yet LSE authority.
11. V7 consumes upward ground longwave as prescribed forcing.
12. V7 canopy-air residuals omit ground sensible and vapor exchange.
13. A physically coupled solve cannot be added without changing accepted V7
    residual equations/unknown topology.
14. The required real coupled consumer therefore needs a successor immutable
    vegetation model identity; unchanged receipt-only consumption would not.

Child 1 owns the successor identity, complete thermal owner and constitutive
authority. No Rust constitutive work is permitted before its authority gate.
