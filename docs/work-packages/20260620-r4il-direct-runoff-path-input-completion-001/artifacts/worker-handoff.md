# R4I-L Worker Handoff

Status: complete.

Evidence class: Static/Ran.

## Retained Edits

- `direct_runtime.rs`: R4I/R4J/R4K/R4L span constants, day-frame fields,
  executor order, and compact report-recording helper.
- `direct_runtime/runoff.rs`: R4I-L producers plus the R4A runoff partition
  consumer and completeness gate.
- `direct_runtime` tests: R4I-L producer identity, invalid inputs, missing
  upstreams, R4A consumption, and split test module.
- `openwepp-runner` tests: updated direct-runtime counter expectations.

## Gates

Focused tests, workspace Rust gates, `cargo deny check`, H2637
default-disabled median, and H2637 PASS row identity passed.

## Known Residuals

R4I-L remains handoff-only by design. It does not claim WB14 equation
migration or runoff publication cutover.

## Next Package

Proceed with R4M/O direct subsurface compute promotion from
`docs/work-packages/r4-burndown-execplan.md`.
