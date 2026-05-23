# PL17 Kinetic-Validation Evidence

Status: `complete`
Evidence mode: `Static + Ran`

## Validation Focus

1. Decomposition equation inputs are required and domain-validated.
2. Tracked pool updates use explicit exponential decay factors.
3. Event modifiers apply to equation-updated pool values.
4. Missing required equation symbols hard-fail at decomposition dispatch.

## Evidence

Static:
- `compute_equation_decomposition_seed_surface` implements:
  - temperature factor (`tmpfac`) and water factors (`swatfc`, `fwatfc`, `envinx`),
  - decay factors (`exp(-envinx*oratea)`, `exp(-envinx*orater)`),
  - annual/perennial event modifiers with typed guards,
  - non-negative/finiteness postconditions for `sumrtm_seed` and `sumsrm_seed`.

Ran:
- `pl17_contract_conformance_rejects_missing_decomposition_equation_symbol` confirms hard-fail posture (`HS-DECOMP-E-001`) for missing required slot/crop decomposition symbol.
- PL17 annual/perennial equation-update tests confirm tracked pools are updated on active dispatch days.

Command:
```bash
cargo test --test parser_runtime_seam_integration pl17_contract_conformance_ -- --nocapture
```

Result: `4 passed`.
