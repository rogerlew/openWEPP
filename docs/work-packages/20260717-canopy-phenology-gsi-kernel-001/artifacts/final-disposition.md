# Final Disposition

Evidence class: `Static`, `Ran`, and independently verified retained `Ran`

Disposition: `PASS-PROCESS-KERNEL`

The package closes the bounded generalized Growing Season Index process law:

- Jolly–Nemani–Running temperature, vapor-pressure-deficit, and photoperiod
  indicators and their multiplicative daily product;
- FAO-56 daylight geometry with signed latitude and finite polar boundaries;
- exact available-sample warm-up and 21-sample FIFO arithmetic mean;
- year-aware consecutive-day admission, typed failures, and exact anchored
  restart; and
- a nonuniform public restart vector that continues bit-identically to an
  uninterrupted state.

Final evidence is 13/13 focused tests, 2,085/2,085 full-workspace tests, clean
formatting/Clippy/dependency policy, fresh adjudicated CRAP with zero actionable
rows, and two independent final `PASS` verifications. Earlier review and
verification HOLDs remain retained and were corrected without waiver.

## Claim Boundary

No production consumer reads this crate. This disposition does not claim an
openWEPP canopy-cover, LAI, biomass, litter, snow, ET, erosion, empirical-
validation, assurance, or release improvement. It establishes the process
signal and its state semantics only.

## Handoff

`CANOPY-PHENOLOGY-02` is next. Before production integration it must amend the
plant/residue authority for deciduous and mixed-forest seasonal canopy,
foliar/structural biomass and winter floor, leaf-on allocation, leaf-off litter
transfer, annual no-drift closure, native YAML operands, and real downstream
snow/ET/erosion consumption. It must not treat the diagnostic 0.5 GSI crossing
as an unratified production switch.
