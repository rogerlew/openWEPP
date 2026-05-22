# ARCH14 Remediation Work-Package Queue

Static: dependency-ordered governance queue derived from ARCH14 dispositions.
Ran: none (queue design only).
Status: finalized.

## Dependency-Ordered Queue

| queue_id | title | primary_findings | dependency_order | dependencies | expected_output |
|---|---|---|---|---|---|
| `ARCH15` | Kernel Seam Typing + Unit-Boundary Wiring Ratification | `CRF-001`, `CRF-002`, `CRF-004` | 1 | none | Typed kernel seam contracts replacing stringly maps; unit-safe seam wiring proof; purity contract/docs alignment decision. |
| `ARCH16` | Scheduler Hot-Path Surface Optimization | `CRF-003` | 2 | `ARCH15` | Reduced clone/allocation scheduler interfaces with deterministic behavior preserved and benchmark evidence. |
| `ARCH17` | Parser-to-Simulation Seam Ownership + Integration Closure | `CRF-005`, `CRF-010` | 3 | `ARCH15` | End-to-end seam ownership contract, parser-to-runtime integration tests, and root/workspace integration acceptance checks. |
| `ARCH18` | HBP Authority and Convergence Closure | `CRF-006` | 4 | `ARCH15` | Explicit HBP authority split (parser vs bridge), convergence tests, and ADR-0012 provenance pin records with exact SHA evidence. |
| `ARCH19` | Top-Level `.run` + Parquet Boundary Contracts | `CRF-007` | 5 | `ARCH17` | Canonical `.run` and parquet boundary contracts plus cross-file closure map to parser/runtime surfaces, seeded by `/workdir/wepppyo3` parquet writer/schema references. |
| `ARCH20` | Governance Throughput + Build Hygiene Controls | `CRF-008`, `CRF-009` | 6 | `ARCH16`, `ARCH17`, `ARCH18` | Delivery throughput rubric, WIP/closure policy, and workspace build-discipline documentation. |
| `ARCH21` | Architecture Review Re-closeout | all open `CRF-*` | 7 | `ARCH15..ARCH20` | Re-verification package confirming closure evidence for all findings and releasing ARCH14 HOLD. |

## Blocker Classification

- Hard blockers (`must close before HOLD release`): `CRF-001`, `CRF-002`, `CRF-003`, `CRF-005`, `CRF-006`.
- Sequence-sensitive but non-blocking for immediate seam migration: `CRF-004`, `CRF-007`, `CRF-010`.
- Governance/hygiene follow-through: `CRF-008`, `CRF-009`.

## Queue Policy

- No package may claim closure for `CRF-001` or `CRF-002` without explicit typed seam and unit-boundary evidence.
- `ARCH21` is required to lift ARCH14 `HOLD` and must include dual review + dual verification artifacts.

## Provisioning Status

- `ARCH15` provisioned:
  - `docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/`
- `ARCH16` provisioned:
  - `docs/work-packages/20260522-arch16-scheduler-hot-path-surface-optimization-001/`
- `ARCH17` provisioned:
  - `docs/work-packages/20260522-arch17-parser-to-simulation-seam-ownership-and-integration-closure-001/`
- `ARCH18` provisioned:
  - `docs/work-packages/20260522-arch18-hbp-authority-and-convergence-closure-001/`
- `ARCH19` provisioned:
  - `docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/`
- `ARCH20` provisioned:
  - `docs/work-packages/20260522-arch20-governance-throughput-and-build-hygiene-controls-001/`

## Execution Status Snapshot (2026-05-22)

- `ARCH15`: complete (`GO-WITH-AMENDMENTS`).
- `ARCH16`: complete (`GO-WITH-AMENDMENTS`).
- `ARCH17`: complete (`GO-WITH-AMENDMENTS`).
- `ARCH18`: executed; currently `HOLD` pending hold-lift gate replay after
  ARCH17 stabilization.
- `ARCH19`: pending.
- `ARCH20`: complete (`GO`).
