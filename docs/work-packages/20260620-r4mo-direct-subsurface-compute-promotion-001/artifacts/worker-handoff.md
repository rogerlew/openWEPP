# Worker Handoff

Status: complete.

Evidence class: Static/Ran.

## Retained Edits

- `direct_runtime.rs`: R4M/R4O span constants, day-frame fields, executor
  order, public re-exports, and typed direct-domain error.
- `direct_runtime/subsurface.rs`: direct WB18 percolation and WB19
  drainage/lateral compute from typed layer vectors.
- `direct_runtime/storage.rs`: R4B upstream completeness gate now requires
  R4M/R4O shadows for direct `D` and direct `Qd`.
- `direct_runtime` tests: R4M/O parity, branch, guard, missing-upstream, and
  anti-alias coverage.
- `openwepp-runner` tests: updated direct-runtime counter expectations for
  R4M/R4O.

## Gates

Focused tests, workspace Rust gates, `cargo deny check`, H2637
default-disabled median, and H2637 PASS row identity passed.

## Known Residuals

R4M/O remains shadow/direct-runtime only. It does not claim WB17 ET/root uptake
promotion, public subsurface publication cutover, scheduler activation, or
default direct-runtime activation.

## Next Package

Proceed with R4N direct WB17 evapotranspiration/root-uptake compute promotion.
