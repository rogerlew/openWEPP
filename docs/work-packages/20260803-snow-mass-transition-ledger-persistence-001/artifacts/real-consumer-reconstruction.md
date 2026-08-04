# Real Consumer Reconstruction

Status: `PASS`

Evidence mode: `Static + Ran`

Real path:

`hourly snow solve -> SnowCouplingOutcome -> DirectSnowSolidToLiquidLedger -> exact liquid_handoff_m argument -> Stage-3 solve -> DirectSnowLiquidDispositionLedger + DirectSnowStage3Outcome -> optional DirectSnowVerboseDiagnostics -> runner schema-v4 formatter -> append-only JSONL writer -> ledger_persistence.py parser`.

The candidate release CLI wrote `14245` real Snowbird rows. The independent
parser reconstructed:

- upstream maximum error: `1.3877787807814457e-17 m` for
  `liquid_handoff - snowpack_swe_loss - rain_released`;
- downstream maximum error: `1.2271813339820303e-17 m` for
  `incoming - routed - retained_delta - refrozen - residual`;
- enabled Stage-3 rows: `8615`; linked incoming/handoff rows: `8615`;
- raw-melt-as-state-loss alias rejected on `3844` rows;
- positive-hourly-melt-as-state-loss alias rejected on `3779` rows;
- Stage-3-routed-as-top-handoff alias rejected on `2796` rows;
- retained-store-as-signed-delta alias rejected on `6811` rows;
- omitted-retained alias rejected on `2088` rows;
- doubled-refreeze alias rejected on `1764` rows; and
- `6009` Stage-3 rows independently reconstruct nontrivial disposition
  operands.

The compact consumer is the direct runtime: inputs, state, downstream operands,
and shadow projection all carry the same ledger types without recalculation.
The production water-temperature publication consumes only
`DirectSnowStage3Outcome`. The file consumer is the real release writer and
requires `verbose_diagnostics.as_deref()` after the runner has selected the
row. Producer-only evidence is not used for closure.

Evidence: `target/snow_mass_transition_ledger_persistence/reports/comparison.json`
and both immutable release binaries under the package target namespace.
