# Bounded Runtime Correction

Status: `IMPLEMENTED / focused gates PASS / fresh science review PASS`

## Authority and correction

`SC-LANDSURFACEENERGY-001` requires forest-litter conductivity
`lambda_l = 0.1 + 0.03*W_l/(rho_w*dz_l)`. The strict V8 projection had supplied
the first mineral-soil node conductivity instead. The corrected projection
reads `W_l` from the immutable beginning `DirectSurfaceLiquidStateRecord`, uses
the configured litter thickness, and supplies the resulting litter
conductivity to the solver-ready surface operand.

The same review disposition selects each LSE tile's configured VIS/NIR ground
albedos as the sole E01--E03 column lower-boundary owner. The vegetation
`SnowFreeForcing` pair remains digest-bound as part of that owner receipt but is
not consumed or equality-joined as covered ground optics. Heterogeneous covered
tile optics are supported and exercised through the strict public endpoint.

## Evidence

- Ran: projection-level litter conductivity regression PASS. Its fixture makes
  litter and top-soil conductivities deliberately distinct and changes the
  beginning litter store independently.
- Ran: strict public endpoint target PASS, 10/10, including heterogeneous
  covered LSE optics with a deliberately different canopy-forcing albedo pair.
- Ran: orchestrator all-target Clippy PASS.
- Ran: formatting and diff hygiene PASS before package artifact updates.
- Ran: fresh independent science/ownership re-review PASS after its initial
  projection-evidence HOLD was remediated. No material runtime findings remain.

No V3/V5 frozen fixture or calculator was modified. The terminal authority HOLD
remains load-bearing and is assigned to
`20260817-c3-woody-v3-v5-oracle-reconciliation-001`.
