# PL16 Growth State-Trajectory Evidence

Status: `complete`
Evidence mode: `Ran`

## Annual Branch Trajectory Signal

From `pl16_contract_conformance_scheduler_emits_equation_updated_annual_growth_state_on_active_day`:
- Active annual day (`day=200`, non-reset branch) produced:
  - `state_after.sumgdd > state_before.sumgdd`
  - `state_after.vdmt > state_before.vdmt`
  - `state_after.cancov > state_before.cancov`

Interpretation:
- PL16 annual path is no longer pass-through; it emits equation-updated phenology, biomass, and canopy progression.

## Perennial Branch Trajectory Signal

From `pl16_contract_conformance_scheduler_emits_equation_updated_perennial_growth_state_on_active_day`:
- Active perennial day (`day=220`, non-reset branch) produced:
  - `state_after.sumgdd > state_before.sumgdd`
  - `state_after.vdmt > state_before.vdmt`
  - `state_after.rtd >= state_before.rtd`

Interpretation:
- PL16 perennial path emits equation-updated growth state with non-regressive root-depth trajectory behavior on active growth days.

## Command

```bash
cargo test --test parser_runtime_seam_integration pl16_contract_conformance_ -- --nocapture
```

Result: `3 passed`, confirming trajectory assertions for representative annual/perennial branches.
