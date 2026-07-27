# Pre-Implementation Contract Gate

Status: `PASS / EXPECTED RED CONFIRMED`

Evidence class: `Ran`

Admitted contract base: commit `9142da64`.

No production behavior had changed when these gates ran.

| Gate | Result | Expected defect evidence |
|---|---|---|
| `cargo nextest run -p openwepp-runner canopy_phenology_02_real_consumers_share_the_typed_native_state` | expected red, exit `100` | explicit native `bbb/hmax` validation before GSI mutation was absent |
| `cargo test -p openwepp-hillslope-orchestrator native_canopy_height_zero_positive_and_structural_vectors --no-run` | expected red, exit `101` | seven compile errors: authoritative `direct_native_canopy_height_m` helper did not exist |

The source guard proves the production builder lacks the required pre-mutation
parameter validation and post-GSI height projection/publication. The
constitutive vectors prove the new zero, positive, structural-only, monotonic,
saturation, overflow, underflow, and invalid-operand law is not implemented
accidentally through an existing helper.

This expected-red result admits production implementation. It is not a passing
correctness claim; every vector and source guard must turn green afterward.
