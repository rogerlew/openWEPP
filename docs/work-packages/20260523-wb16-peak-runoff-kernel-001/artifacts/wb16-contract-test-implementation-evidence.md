# WB16 Contract-Test Implementation Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Implemented Test Target
- `tests/integration/wb16_peak_runoff_kernel_contract.rs`
- Registered in `Cargo.toml` as `wb16_peak_runoff_kernel_contract`

## Contract-Derived Tests
1. `wb16_contract_conformance_emits_peak_runoff_outputs_with_branch_authority`
- Verifies finite `peakro`/`watdur`, continuity `watdur = Q/peakro`, and
  deterministic method-branch selector across three vectors.

2. `wb16_contract_conformance_rejects_missing_peak_symbol`
- Verifies missing required symbol hard-fails at closure diagnostics with
  `HKERNEL-WB16-PEAK-E-001`.

3. `wb16_contract_conformance_rejects_non_finite_peak_symbol`
- Verifies non-finite required symbol hard-fails with
  `HKERNEL-WB16-PEAK-E-002`.

4. `wb16_contract_conformance_rejects_out_of_domain_peak_symbol`
- Verifies domain-invalid symbol hard-fails with
  `HKERNEL-WB16-PEAK-E-003`.

## Pre-Implementation Gate Execution
Command:
```bash
cargo test --test wb16_peak_runoff_kernel_contract
```

Observed pre-implementation result:
- `0 passed; 4 failed`
- Failure signatures confirmed missing WB16 production behavior (no
  `peakro`/`watdur`, no WB16 typed closure-diagnostics halts).
