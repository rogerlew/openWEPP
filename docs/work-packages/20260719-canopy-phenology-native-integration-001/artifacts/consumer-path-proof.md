# Real Consumer Path Proof

Evidence mode: `Static + Ran`

Status: `focused real-consumer proof passes`

The test
`native_forest_yaml_executes_through_the_direct_production_consumer` executes a
real native-YAML hillslope run through the direct production executor. Test-only
builder observations are joined to the completed `DirectDayFrame` for the same
day and lane. The assertions compare the producer realization with:

- the active typed growth state's foliar biomass, LAI, and canopy cover;
- the canopy argument supplied to snow partition;
- WB15 canopy, LAI, and foliar-biomass inputs plus an independent recomputation
  of interception;
- the ET compute inputs after the growth phase;
- same-day decomposition litter, surface residue, and residue depth;
- the residue depth recorded at the active frost thermal compute; and
- the canopy cover recorded at active erosion daily-state assembly.

The two-day controlled forcing makes day 1 leaf-on and day 2 leaf-off, so the
run exercises a nonzero litter handoff rather than proving only zero-value
wiring. The first realization independently asserts zero allocation and zero
litter. `cargo test -p openwepp-runner --lib` passed all 132 tests.
