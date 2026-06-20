# Pre-Implementation Contract Gate

Status: pre-implementation passed.

Static: `SC-EVAP-001` provides sufficient direct authority for WB17
surface/residue ET, soil layer mutation, optional stage/PMET behavior, SWU root
uptake, component ET, stress, uptake vectors, and `pltol` normalization.

Static: `SC-WATBAL-001` provides sufficient ordering authority for the split
R4N implementation:

- R4M WB18 percolation runs before R4N surface ET.
- R4N surface ET mutates layer state before R4O WB19 drainage/lateral.
- R4N root uptake runs after R4O and before R4B storage reconciliation.

Decision: no science-contract amendment is required before implementation.
