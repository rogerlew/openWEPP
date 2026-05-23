# PL16 Regression Parity-Trace Evidence

Status: `complete`
Evidence mode: `Static + Ran`

## Legacy Equation Trace Mapping

PL16 implementation constants and branch equations were mapped to canonical
legacy growth authority (`REF-PLANT-LEGACY-GROW`, `grow.for` range cited in
`SC-PLANT-001`) for:
- PAR and dry-matter increment scale terms,
- annual/perennial LAI denominator forms,
- harvest-index heat/stress adjustment form,
- root-depth curve coefficients and depth capping behavior.

## Runtime Parity-Trace Signals

Ran tests demonstrate behavior shift from PL13 plumbing-only semantics to PL16
physics semantics for representative branches:
- annual active-day trajectory progression assertions pass,
- perennial active-day trajectory progression assertions pass,
- required-symbol guard fail-fast behavior passes.

## Command

```bash
cargo test --test parser_runtime_seam_integration pl16_contract_conformance_ -- --nocapture
```

Result: `3 passed`.

## Residual Risk Note

No direct numeric openWEPP-vs-legacy PL16 comparator harness row was added in
this package; parity trace evidence here is contract-authority mapping plus
branch-behavior conformance assertions.
