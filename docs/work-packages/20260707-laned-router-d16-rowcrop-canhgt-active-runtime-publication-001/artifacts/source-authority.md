# Source Authority

Status: COMPLETE

Static:

- Baseline `/workdir/wepp-forest_260430_baseline/src/grow.for` computes daily
  canopy height after canopy-cover update:
  `canhgt = (1 - exp(-bbb * vdmt)) * hmax`.
- Baseline `/workdir/wepp-forest_260430_baseline/src/initgr.for` initializes
  live canopy geometry from initial canopy cover using `bb`, `bbb`, and `hmax`.
- Baseline `/workdir/wepp-forest_260430_baseline/src/frcfac.for` consumes
  daily `canhgt/hmax` for live friction-cover context.
- `SC-PLANT-001` declares `Hc` as plant state and coupling surface, and
  requires daily plant state publication.
- `SC-OFEROUTE-001` requires active Lane D vegetation operands to consume
  post-growth LAI and positive finite `h_c` whenever LAI is positive.

Defect classification:

- Not a crop-growth absence. The failed row-crop fixture has positive LAI.
- Not an invalid-input authorization problem. Baseline authority gives a daily
  canopy-height equation using already-projected crop parameters.
- Implementation defect: daily growth state and Lane D publication omitted
  `canhgt`, leaving active routing with a stale/static initial value.

Ran:

- Confirmed the correction is a native daily growth-state publication fix, not
  a fallback, guard relaxation, or crop-growth scaffold addition.
- Amended `SC-PLANT-001` rev 19 to require daily `canhgt/Hc` computation from
  `bbb`, `vdmt`, and `hmax` on the PL16 growth surface.
- Amended `SC-OFEROUTE-001` rev 36 to bind Lane D dynamic `h_c` to the
  post-growth plant-state publication carried through
  `DirectDayFrame.evapotranspiration_compute_inputs.canopy_height_m`.
- Attempted broader Wave-1 erosion consumption of post-growth canopy height,
  but the `erosion_single_ofe_p61_sediment` fixture exposed a material
  fidelity change. That broader consumer move was reverted and is outside this
  blocker fix; the contract/code change here is limited to the active/shadow
  Lane D routing operand path.
