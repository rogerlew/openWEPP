# Authority Provenance

Status: `RECORDED`

- Paradigm 2 admission and staged architecture: ADR-0029, ADR-0028, ADR-0026.
- Stage 3 thermal/liquid solver authority: Stage 0 surface-energy primitives,
  Marks 1999, libsnobal CC0 provenance recorded in prior Stage 0/Stage 3
  packages, Crocus, and the 10.3.8 holding-capacity authority.
- Decoupling decision: `docs/planning/snow-frost-fidelity-strategy.md` §10.3
  step 10 records that Stage 3's thermal/liquid capability is correct and
  affordable but must stop inheriting Stage 1's non-promoted density profile.

`SC-SNOWFREEZE-001` v111 records this as
`REF-SNOWFREEZE-PARADIGM2-STAGE3-DECOUPLE`, `INV-SNOWFREEZE-081`, and
`OBL-SNOWFREEZE-P-056`. The authority delta is a coupling/selector boundary:
Stage 3 thermal/liquid remains the prior Stage 3 solver, while the layer carrier
for the water-temperature arm is forced to be bulk-equivalent under the current
bulk density default.
