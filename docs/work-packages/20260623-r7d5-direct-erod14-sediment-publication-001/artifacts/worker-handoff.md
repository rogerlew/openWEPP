# Worker Handoff

Status: executed-held.

## Handoff

- R7D5 final disposition:
  `HOLD-R7D5-DIRECT-EROD13-EROD14-EROD15-TYPED-PRODUCER-ABSENT`.
- First actionable item: implement a typed direct EROD13/EROD14/EROD15
  producer span, not a publication copy hook. The producer must own:
  - direct EROD13 inputs and state updates needed by EROD14,
  - direct EROD14 class state carry (`gend`, `frcflw`, `frac`, `fidel`,
    `tcf1`, `sedmax`, `sed_frac`, and related transport scalars),
  - direct EROD15 publication operands for `total_detachment_kg`,
    `total_deposition_kg`, `particle_class_count`, and
    `sediment_concentration_kg_m3[0..5]`,
  - downstream MOFE sediment carry sufficient for sediment-coupled
    `qin/qout` provenance where the contract authorizes it.
- Preserve the R7D5 guard until the typed producer populates
  `DirectPublicationErosionOperands` from direct-owned state.
- Do not copy `execution.runtime_surface.total_detachment_kg`,
  `execution.runtime_surface.total_deposition_kg`,
  `sediment_concentration_kg_m3_0001`, `execution.wb13_rows`, compatibility
  HBP bytes, or public-output builders as direct authority.
- Re-run focused H2637 after the producer lands. The next expected transition
  is from R7D5 fail-closed exit `1` to direct exit `0`; only then compare HBP,
  PASS, WAT, loss, plot, and manifest parity.
