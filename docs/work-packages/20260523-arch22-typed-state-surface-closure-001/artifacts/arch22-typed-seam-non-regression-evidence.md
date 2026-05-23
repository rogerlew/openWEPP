# ARCH22 Typed-Seam Non-Regression Evidence

Status: `completed`
Evidence mode: `Ran`

## Regression Gate
Command:
```bash
cargo test --test parser_runtime_seam_integration
```

Result: pass (`45 passed; 0 failed`).

Log:
- `artifacts/test-logs/04-parser-runtime-seam-integration.log`

Interpretation:
- Parser-to-runtime typed seam behavior remains non-regressed after ARCH22
  production surface migration.

## Hydrology and Watershed Contract Continuity
Commands:
```bash
cargo test --test wb11_hydrology_kernel_contract
cargo test --test ws10_watershed_kernel_contract
```

Results:
- `wb11_hydrology_kernel_contract`: pass (`3 passed`).
- `ws10_watershed_kernel_contract`: pass (`4 passed`).

Logs:
- `artifacts/test-logs/02-wb11-hydrology-kernel-contract.log`
- `artifacts/test-logs/03-ws10-watershed-kernel-contract.log`

Interpretation:
- ARCH22 typed symbol migration preserves covered WB11/WS10 guard and runtime
  behavior expected by existing typed seam closure posture (ARCH15/ARCH21).
