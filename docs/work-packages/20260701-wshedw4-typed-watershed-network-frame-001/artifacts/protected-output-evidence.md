# Protected Output Evidence

Status: `EXECUTED-PARTIAL-HOLD`

Evidence class: `Ran:` plus `Static:`

W4 moved publication through `WatershedPublicationFrame` while preserving the
existing watershed output row seed schema.

## Focused Public Output Evidence

Ran:

```text
cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --nocapture
```

Result: `PASS`, `24` tests passed in `74.01s`.

Covered public-output surfaces:

- Existing watershed CLI output emission tests passed.
- W3 generated-mode `--jobs 1` versus `--jobs 3` decoded Parquet row
  equivalence passed across routed watershed outputs.
- W3 child-failure and missing-pass tests still prove routing/publication are
  skipped on supervisor/pass-inventory failures.
- W4 source guard proved the CLI now publishes from the typed
  `WatershedPublicationFrame` conversion rather than
  `WatershedKernelExecutionReport` / `WatershedWritebackSurface` directly.

## Conservation / Reconstruction Status

Static operand lineage is recorded in `operand-lineage.md`. Current W4 code
does not change protected output formulas; it harvests the compatibility kernel
result into typed routed-state fields and then builds
`WatershedPublicationFrame`.

The compatibility harvest still reads `WatershedKernelExecutionReport.writeback_surface`,
and missing compatibility symbols still default to zero in the partial
implementation. That matches the previous publication helper shape closely
enough for the held partial handoff, but it is not fail-closed typed
publication evidence for complete W4.

Independent reconstruction and committed-fixture conservation/magnitude audit
were **not** run as final acceptance gates because W4 cannot close complete
while routing still depends on `compatibility_writeback_surface`.

## Disposition

Protected output regression evidence is sufficient for the landed partial
implementation and hold disposition. It is not sufficient for
`EXECUTED-COMPLETE-WSHED-W4`; complete closure still requires committed-fixture
protected output identity or contract-governed deltas after typed routing no
longer projects through the old writeback surface, plus typed publication
fail-closed behavior for missing routed operands.
