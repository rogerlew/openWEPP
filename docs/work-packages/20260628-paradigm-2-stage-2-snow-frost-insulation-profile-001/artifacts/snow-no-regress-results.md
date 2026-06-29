# Snow No-Regress Results

Status: `PASS-STATIC`

Stage 2 must not worsen the cross-SNOTEL snow rubric. Snow-side results are a
guardrail, not the promotion gate.

Static basis:

- `OPENWEPP_SNOWFROST_STAGE2_INSULATION_MODEL` is parsed only in the
  snow-to-frost thermal-input construction path.
- The selector is consumed after the prior-day `DirectSnowLaneState` is read for
  frost and does not write snow SWE, snow depth, snow density, snow layers,
  melt/liquid routing, phase partitioning, albedo, canopy, radiation, or WAT
  snow publication.
- Both frost-corpus model arms set
  `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_multilayer_density_v1`;
  the only changed runtime selector is
  `OPENWEPP_SNOWFROST_STAGE2_INSULATION_MODEL`.

Disposition: no cross-SNOTEL snow-rubric regression path is introduced by the
Stage 2 selector. Because the primary frost gate failed, no activation claim or
promotion-ranking snow rerun is made.
