# HB-09 Preimplementation Record

Evidence class: **Static**

## Classification

Both rows are actionable `E-PRODUCTION`. The fourteen-output writer is just
above CRAP 30; `float64_value` is fully covered but CC 69, proving that tests
alone cannot close it. Cover-first remains mandatory for same-source floor and
A–H closure before mechanical decomposition.

The 2,706-line source triggers WARN and has limited headroom below the
3,000-line blocker. Extraction should reduce or hold line count where practical;
new large parallel writer tables or duplicate test fixtures are prohibited.

## Data And Call Path

Public entry points accept either `WatershedInterchangeRowSeed` or
`WatershedPublicationFrame`, both through `WatershedOutputRecord`. The common
writer visits fourteen fixed path/schema pairs. `build_row_batch` dispatches by
Arrow type and `float64_value` maps schema names/aliases to typed operands,
derived sums/balances, or depth-to-volume conversions. `write_single_output`
creates, writes and closes each Parquet file with typed `OWSOUT-E-005` errors.

The runner watershed CLI constructs typed publication frames, calls this
writer, and downstream tests reopen `ebe_pw0` and `totalwatsed3`; that is the
real consumer chain.

## Refactor Hazards

- output file sequence and first-error precedence;
- exact schema builder paired with each configured path;
- match-arm alias precedence and unknown-field `None` behavior;
- Option propagation through sums/products/balance;
- depth millimetres × area square metres ÷ 1000 volume conversion grouping;
- kilograms versus tonnes and sediment class index order;
- Arrow nullability/type construction and metadata;
- exact `OWSOUT-E-001..005` code, path and detail behavior;
- no partial-output masking, fallback column, silent zero or null coercion.

## Planned Evidence

Fresh metrics will inventory eligible floors. Characterization will bind every
output path/schema pair, Float64 family, optional/null edge, unsupported type,
writer failure phase and independent water/sediment/channel closure. Final
proof includes same-source metrics, exact schema/value diff, full crate tests,
one real CLI readback consumer, line governance, two reviews and two
verifications.

No production or test edit has been made by this kickoff.
