# ARCH14 Hold Release Decision Record

Status: `complete`
Evidence mode: `Static + Ran`

## Decision
- outcome: `HOLD_ARCH14_PENDING`
- allowed values:
  - `GO_ARCH14_RELEASED`
  - `HOLD_ARCH14_PENDING`

## Preconditions
- [x] `CRF-001..010` evidence matrix complete.
- [x] Required workspace gates replayed and recorded.
- [x] Dual review artifacts complete.
- [x] Dual verification artifacts complete.

## Decision Rationale

1. High-severity hold criterion is not yet cleared.
   - `CRF-006` remains blocked because ARCH21 replay still fails `cargo fmt --check`.
   - Evidence: `artifacts/gate-results.md`, `artifacts/crf-closure-evidence-matrix.md`.
2. Mandatory architecture direction is preserved and not reversed.
   - `CRF-001` and `CRF-002` remain closed in ARCH15 with typed seam and unit-boundary wiring direction intact.
3. Medium-severity boundary closure (`CRF-007`) remains explicitly `HOLD` in ARCH19 with open `RUN-HOLD-*` and `PRQ-HOLD-*` items.

## Residual Risk Register

| risk_id | severity | statement | owner | follow_on |
|---|---|---|---|---|
| `R-ARCH21-001` | high | Workspace format gate is not green; full ratification gate set is not all-pass. | ARCH/KERNEL + HBP owners | Normalize formatting drift and rerun full gate set. |
| `R-ARCH21-002` | medium | `.run` and parquet boundaries remain governance-only with explicit open hold rows. | CONTRACT + runtime/output owners | Close ARCH19 `RUN-HOLD-001..003` and `PRQ-HOLD-001..003`. |
| `R-ARCH21-003` | medium | Parser/runtime seam ownership closure is representative, not exhaustive across all parser families. | INPUT + ORCHESTRATOR owners | Execute follow-on seam-family coverage package. |
