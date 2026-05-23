# PL17 Residue Trajectory Evidence

Status: `complete`
Evidence mode: `Ran`

## Annual Branch Trajectory Signal

From `pl17_contract_conformance_scheduler_emits_equation_updated_annual_decomposition_state_on_active_day`:
- Active annual decomposition day (`day=200`) produced:
  - `payload.sumrtm_seed < before_sumrtm`
  - `payload.sumsrm_seed < before_sumsrm`

Interpretation:
- PL17 annual decomposition payload is no longer pass-through; tracked pools follow equation-driven decay on active dispatch days.

## Perennial Branch Trajectory Signal

From `pl17_contract_conformance_scheduler_emits_equation_updated_perennial_decomposition_state_on_active_day`:
- Active perennial grazing day (`day=180`) produced:
  - `payload.sumrtm_seed < before_sumrtm`
  - `payload.sumsrm_seed < before_sumsrm`

Interpretation:
- PL17 perennial decomposition payload emits equation-updated tracked pools under grazing control on active branch days.

## Command

```bash
cargo test --test parser_runtime_seam_integration pl17_contract_conformance_ -- --nocapture
```

Result: `4 passed`, confirming representative annual/perennial trajectory assertions.
