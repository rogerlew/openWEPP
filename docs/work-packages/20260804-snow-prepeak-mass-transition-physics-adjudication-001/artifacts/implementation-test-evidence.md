# Implementation And Test Evidence

Status: `pass / exact-current execution and independent reconstruction`

Evidence mode: `Ran`

## Direct Execution

- Release build command:
  `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`.
- Binary SHA-256:
  `4e0ebd96da7daa74c6a2c22dce200c87208997df9ac424a0e0b31de83b51da47`.
- Four real direct-production CLI runs completed with the exact v2 selector
  environment and copied fixture-manifest identity.
- Traces contain `61,364` schema-v4 daily rows. Every pre-v4 top-level and
  nested hourly operand is exact against the accepted v3 predecessor.
- WAT and HBP are bit-identical at all four sites. This direct hillslope surface
  emits no PASS file.

Across primary windows, raw signed melt, gross-positive hourly melt, top-level
CoE routed melt, and retained-store level differ from their prohibited target
aliases on `12,633`, `12,457`, `11,691`, and `23,075` days. Omitting retained
amount or doubling refreeze breaks Stage-3 closure on `8,671` and `6,841` days.
The legitimate top-level CoE handoff equals exact Stage-3 incoming on every
day; it is not substituted for Stage-3 routed liquid. Hourly-snowfall lineage
is forced by code and an explicit mismatched-alias unit test even though real
reported accumulation differs only by sub-tolerance serialization rounding.

## Independent Reconstruction

`tools/verify_adjudication.py` independently streamed the four exact traces,
summed hourly snowfall rather than trusting reported accumulation, reconstructed
all three daily mass identities, rebuilt 154 primary windows, and matched ten
annual result fields within `2.2204e-15 m`.

Maximum residuals are:

- authoritative snow storage: `9.9973e-13 m`;
- reported accumulation versus hourly snowfall: `3.4381e-13 m`;
- shared handoff: `1.3878e-17 m`;
- Stage-3 disposition: `2.5045e-17 m`; and
- reconstructed versus producer Stage-3 residual: `2.0922e-17 m`.

## Package And Protected Tests

- Python compile: three package tools and one focused test file pass.
- Package-local unit tests: `6/6` pass, including explicit rejection of the
  reported-accumulation and top-level-routed aliases.
- Focused existing snow/contract suite: `34/34` pass across six binaries.
- Rust format check and exact release rebuild pass.

The initial compatibility booleans were invalidated because the parser counted
expected v4 additions. Review then found two evidence-lineage defects in
reported snowfall and routed-liquid anti-alias checks. The corrected projection
and primitive reconstructions reanalyzed identity-checked retained outputs
without rerunning the model or changing a scientific operator.
