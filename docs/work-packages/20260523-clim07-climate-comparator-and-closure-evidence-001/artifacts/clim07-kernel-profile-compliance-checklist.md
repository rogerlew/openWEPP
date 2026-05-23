# CLIM07 Kernel Profile Compliance Checklist

Status: `completed`
Evidence mode: `Static + Ran`

## Checklist

1. Canonical `SC-*` file updated.
- status: complete
- evidence:
  - `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
  - `docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`

2. Required schema sections present for changed authority.
- status: complete
- evidence:
  - CLIM07 addendum includes vector families, deterministic requirements, and
    contract-test vectors.

3. Algorithm/branch and guard alignment updated for changed behavior.
- status: complete
- evidence:
  - CLIM07 vectors explicitly preserve existing climate seam typed guard posture
    (`CLIM-RUNTIME-E-009` duplicate-time failure vector).

4. Guard/error mapping aligned with executed vectors.
- status: complete
- evidence:
  - `clim07_breakpoint_domain_violation_remains_typed_hard_fail` validates
    `CLIM-RUNTIME-E-009` at hillslope and watershed seams.

5. Test-vector obligations reflected in tests and evidence artifacts.
- status: complete
- evidence:
  - `tests/integration/clim07_climate_comparator_and_closure_contract.rs`
  - `artifacts/clim07-comparator-vector-manifest.md`
  - targeted and workspace gate results recorded.

## Compliance Verdict
- `PASS` for CLIM07 scope.
