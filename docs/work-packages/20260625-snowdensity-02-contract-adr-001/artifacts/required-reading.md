# Required Reading

Static:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/decisions/0026-stateful-winter-column-sub-solver.md`
- `docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001/package.md`
- `docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001/artifacts/snotel-density-delta-ledger.md`
- `docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001/artifacts/rubric-cell-classification.md`
- `docs/work-packages/20260625-snowdensity-01-evidence-reconciliation-001/artifacts/snowd-shen-archaeology.md`

Notes:

- SNOWDENSITY-01 establishes that current openWEPP and pinned legacy share the
  structural snow-density/depth issue; legacy bit-parity is not a remediation
  path.
- ADR-0017 keeps legacy and PySnobal as diagnostic flag profiles unless an
  independent correctness authority is proven.
- ADR-0026 provides the accepted runtime architecture boundary, but does not
  change snow physics.
