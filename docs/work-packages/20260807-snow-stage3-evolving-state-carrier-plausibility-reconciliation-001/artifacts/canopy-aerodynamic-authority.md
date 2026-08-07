# Canopy Aerodynamic Authority Freeze

Static: `AUTHORITY_MISSING` before result execution.

The current Stage 3 turbulent operator consumes CLI wind, the virtual heights,
and exposed-snow roughness without a canopy operand. Sub-canopy longwave uses
effective canopy cover through sky view. Direct runtime also carries canopy
cover/height/LAI-like structural state, while frost `tmpadj` has a separate
legacy canopy/wind path. None of those paths is automatic authority for a
snow-surface within-canopy aerodynamic resistance.

No locally admitted source supplies, as one complete applicable operator, the
canopy aerodynamic equation, displacement/roughness semantics, required canopy
inputs, forcing exposure transformation, and snow-surface coupling. Therefore
the counterfactual budget admits no canopy arm. A scalar attenuation is
forbidden and frost equations must not be reused as snow-energy authority.
This `AUTHORITY_MISSING` finding does not prove that a separate canopy operator
is physically required: that classification first requires resolved wind
exposure and applicability evidence.
