# Real Consumer Reconstruction

Status: `PASS`

Evidence mode: `Ran: exact release CLI + independent streaming parser`

## Execution Identity

- Pre-v4 release binary: `8fb77e17b380c617a6f03faa27670ee45c29a4b2f0ada19b96a164248722c673`.
- Terminal v4 release binary:
  `464c87e16f24997753627d83399979b1f4bcc232629196c1d9847a7f9d0bb407`.
- Fixture: retained local `snotel_snowbird_ut` direct-production fixture.
- Both executions used Stage 3 `layered_thermal_liquid_v1`, multilayer density,
  Harder-Pomeroy hourly phase, CoE liquid holding, and disabled explicit
  longwave/sublimation.
- Final receipt:
  `target/snow_stage3_liquid_signed_hour_trace_closure/runs/post_v4_terminal/snotel_snowbird_ut/snotel_snowbird_ut-post_v4_terminal.receipt.json`.
- Canonical independent report:
  `target/snow_stage3_liquid_signed_hour_trace_closure/reconstruction-report.json`.

## Reconstruction Results

The package-independent streaming parser consumed `14245` real JSONL rows.
It reconstructed:

    incoming - routed - retained_delta - refrozen = residual

Maximum absolute error was `1.2271813339820303e-17 m` against the `1e-9 m`
contract tolerance. Stage 3 was enabled on `8615` rows and had nonzero incoming
liquid on `6009`; no disabled row carried a nonzero Stage-3 liquid operand.

Every rejected alias separated materially:

| Rejected formula | Distinguishing rows |
|---|---:|
| omit retained delta | 2088 |
| substitute top-level CoE routed melt | 2796 |
| substitute CoE retained store | 6811 |
| double-count refrozen liquid | 1764 |

There were `2047` mixed-signed-hour rows. Stage-3 incoming/routed/retained/
refrozen values were individually nonzero on `2004/1914/725/338` of them, and
`227` mixed rows had all four operands nonzero simultaneously.

Across the full trajectory, Stage 3 received/routed/retained/refroze
`40.3463/39.5692/0.7230/0.0541 m`. On mixed-signed days those totals were
`16.6773/16.1353/0.4975/0.0445 m`.

## Behavior Neutrality

- All `14245` v4 rows projected to the complete pre-v4 field surface with
  `0` value mismatches.
- WAT parquet was byte-identical at SHA-256 `e74b8df25485f6e1dd1430a9332c4aab3bafb8498a228f5455611d8081521b75`.
- HBP/PASS was byte-identical at SHA-256
  `d5d3468d361510df069475423f785e2be036e0b353c281d0c32d0f82b583c149`.
- The final v4 JSONL is `659499507` bytes versus `380969291` bytes for v3,
  a `1.7311x` diagnostic-trace size increase. The trace remains explicit
  environment-selected internal evidence and is not a normal public output.

Disposition: the predecessor's missing-operand `HOLD-EVIDENCE` is closed. This
proves observability and existing Stage-3 routing behavior; it does not by
itself authorize a signed-hour or export-physics correction.
