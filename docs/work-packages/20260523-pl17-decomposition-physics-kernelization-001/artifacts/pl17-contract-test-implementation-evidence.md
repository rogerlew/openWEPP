# PL17 Contract-Test Implementation Evidence

Status: `complete`
Evidence mode: `Static + Ran`

## Implemented PL17 Contract-Derived Tests

Added in `tests/integration/parser_runtime_seam_integration.rs`:

1. `pl17_contract_conformance_requires_decomposition_rate_projection_symbols`
- Verifies runtime projection includes `pl_decomp_slot_*_oratea`, `pl_decomp_slot_*_orater`, and primary aliases.

2. `pl17_contract_conformance_scheduler_emits_equation_updated_annual_decomposition_state_on_active_day`
- Verifies annual active-day decomposition payload updates `sumrtm_seed` and `sumsrm_seed` downward on equation path.

3. `pl17_contract_conformance_scheduler_emits_equation_updated_perennial_decomposition_state_on_active_day`
- Verifies perennial grazing-day decomposition payload updates tracked pools on equation path.

4. `pl17_contract_conformance_rejects_missing_decomposition_equation_symbol`
- Verifies typed hard-fail posture when required slot/crop `oratea` symbol is missing.

Supporting updates:
- PL runtime projection coverage checks extended for PL17 decomposition parameter symbols and aliases.
- INT10 integration seed surface updated with PL17 decomposition equation inputs to preserve coupled replay coverage.

## Ran Evidence

```bash
cargo test --test parser_runtime_seam_integration pl17_contract_conformance_ -- --nocapture
```

Result: `ok` (`4 passed`, `0 failed`).
