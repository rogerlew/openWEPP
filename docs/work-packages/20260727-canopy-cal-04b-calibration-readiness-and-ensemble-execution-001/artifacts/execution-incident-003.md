# Execution Incident 003

Status: `CLOSED BEFORE POPULATION`

Evidence class: `Ran`

The bounded third attempt passed preparation and both builds, then completed
the first production native-proof run. Its direct-kernel CSV formatted small
fractions with 17 digits after the decimal point. For the first nonzero GSI,
that representation did not round-trip to the original `f64` bits, so the
bit-exact comparison stopped at 1980-03-22.

The attempt is preserved at
`/home/workdir/cal04b-objects-native-proof-mismatch-003`. No population,
freeze, or Harvard command ran.

Correction: `expected-probe` now writes every compared float in scientific
notation with 18 significant digits, and a focused test requires the observed
small GSI value to round-trip bit-exactly. The correction changes proof
serialization only; it does not change the native kernel or production trace.

