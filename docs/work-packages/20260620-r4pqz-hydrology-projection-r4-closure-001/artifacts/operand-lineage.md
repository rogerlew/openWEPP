# Operand Lineage

Status: complete.

Static lineage:

- `Q` / `QOFE`: R4A direct runoff partition shadow.
- `Dp`: R4M direct percolation/deep-seepage shadow.
- `latqcc`: R4O direct realized lateral flow shadow.
- `Qdd` / `Qd`: R4O direct tile drainage and final subsurface loss shadow.
- `Ep`, `Es`, `Er`, aggregate `ET`, and `Ws`: R4N direct WB17/root-uptake
  shadows.
- `Total-Soil` / `SoilWaterTotal`: aggregate recompute from the final R4N
  direct layer vector, not publication rows.
- snow/frost storage terms: R4G signed snow/frost coupling shadow for current
  R4 authority; detailed public snow/frost publication remains R6/outside R4.
- carry terms: R4J/R3C direct runon/carry and transfer shadows.
- profile-capacity operands: shadow placeholders are explicitly diagnostic
  until a later publication-cutover package promotes them.

All values are meters unless the eventual public projection layer explicitly
converts units.
