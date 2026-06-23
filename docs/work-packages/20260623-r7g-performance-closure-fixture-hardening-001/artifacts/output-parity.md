# Output Parity Evidence

Evidence class: Ran.

Status: executed-held.

## H2637 Protected Outputs

| Output | Default vs rollback | Default vs direct default | Default vs explicit direct | Notes |
|---|---|---|---|---|
| HBP | `PASS` via identical checksum map | `BLOCKED` no direct output | `BLOCKED` no direct output | SHA `b84d598ef3dce350e658928016efb7f232d1c4a2ea9e20e5c5ac50c7992a50b2` |
| loss JSON | `PASS` via identical checksum map | `BLOCKED` no direct output | `BLOCKED` no direct output | SHA `9bdbabe532bfbc2f49d4a4ae5db24c6069e93384f306e71759c223a795a5be38` |
| PASS parquet | `PASS` via identical checksum map | `BLOCKED` no direct output | `BLOCKED` no direct output | SHA `b8e4928e3b3fd60f2f772332cf4812a744822de00a01cd98149bdbc7a6082520` |
| plot parquet | `PASS` via identical checksum map | `BLOCKED` no direct output | `BLOCKED` no direct output | SHA `4cdb19fecd36a3f074d5c900bc687eff7ce58f80a31c9cb7e5e0f5615ac5a783` |
| WAT parquet | `PASS` via identical checksum map | `BLOCKED` no direct output | `BLOCKED` no direct output | SHA `9b8e142b2fe4c0b717045ce7f03945134abc395d713aca29cb0e01a2aba9481a` |
| manifest JSON | `PASS` for protected `output_checksums`; metadata differs by design | `BLOCKED` no direct manifest | `BLOCKED` no direct manifest | default vs rollback output checksum JSON compare returned `0` |

## Metadata / Manifest Parity

Default-disabled manifest:

- `runtime_selection.requested = default-candidate`.
- `runtime_selection.selected = compatibility`.
- `selection_reason = default-candidate-disabled-compatibility-rollback`.
- `fallback_reason = direct-default-candidate-gate-disabled`.
- `execution_provenance.scheduler_kernel_executed = true`.
- `execution_provenance.publication_source = scheduler-kernel`.
- `wb13_publication.row_count = 235961`.

Rollback manifest:

- `runtime_selection.requested = compatibility`.
- `runtime_selection.selected = compatibility`.
- `selection_reason = explicit-compatibility-rollback`.
- `fallback_reason = null`.
- `execution_provenance.scheduler_kernel_executed = true`.
- `execution_provenance.publication_source = scheduler-kernel`.
- `wb13_publication.row_count = 235961`.

## Residuals

- Direct default candidate: blocked before output creation by active snow
  partition authority absence.
- Explicit direct production: blocked before output creation by the same guard.
- Direct protected-output identity cannot be claimed for full H2637 in R7G.
