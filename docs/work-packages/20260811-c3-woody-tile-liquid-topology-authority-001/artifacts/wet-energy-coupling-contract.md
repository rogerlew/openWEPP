# Wet-Energy Coupling Contract

Status: `selected`

Evidence mode: `Static`

Radiation, wet/dry plant area, leaf/stem/wet-node energy, canopy-air state,
FvCB--Medlyn gas exchange, hydraulic state, evaporation, condensation, and
active store-cap re-solve are occupancy-local whenever forcing, radiation, or
wetness differs. PAR, wet fraction, temperature, and conductance are never
averaged before a nonlinear solve.

Energy operands carry transaction, stratum, tile, area basis, interval, and V2
digest. The energy owner independently reconstructs local closure and the
weighted stand closure from immutable component operands.
