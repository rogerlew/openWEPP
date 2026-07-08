# Kernel Profile Compliance

Evidence mode: Static + Ran.

Profile:
`docs/specifications/science-contracts/kernel-process-contract-profile.md`.

## Checklist

| Item | Status | Evidence |
|---|---|---|
| Canonical `SC-*` file updated | PASS | `SC-OFEROUTE-001` rev 43 |
| Required schema sections present | PASS | Existing active contract schema retained; frontmatter version bumped to `43` |
| Algorithm/branch table updated | PASS | Active mesh-policy state-surface row and branch/guard row now record trace-gated diagnostic max-`dt` and coupled space-time evidence |
| Guard/error mapping aligned with code | PASS | `OPENWEPP_LANED_ACTIVE_MAX_DT_S` rejects non-finite, non-positive, `>300`, inactive, or trace-off use through `RuntimeSurfaceFailure`; orchestrator revalidates `max_dt_s` before route execution |
| Unit-governance map completed | PASS | `max_dt_s` added to target-`dx`/mesh-policy unit-governance row with seconds, `<=300`, and active trace opt-in |
| Test-vector obligations reflected | PASS | Active mesh-policy obligation now includes diagnostic max-`dt` bounds/gating and coupled same-`dt`/same-`dx` evidence; focused tests and six-rung ladder recorded in gate results |

## Runtime Projection

The diagnostic selector is a runtime projection control for evidence only. It
does not change production default behavior:

- production default active mesh: fixed `10 cells/OFE`
- production max substep: `LANED_ACTIVE_MAX_DT_S = 300`
- diagnostic max substep: finite positive `max_dt_s <= 300`
- diagnostic max substep requires active trace evidence

## Failure Posture

Invalid diagnostic selector use fails before output setup through
`HillslopeCliError::RuntimeSurfaceFailure`. Invalid orchestrator config or
route-time `max_dt_s` fails through `DirectRuntimeError::DirectDomainViolation`.
No invalid value is normalized-and-proceeded.

## Evidence

Focused tests:

- `cargo test -p openwepp-runner laned_active --lib`
- `cargo test -p openwepp-hillslope-orchestrator laned_active --lib`

Run evidence:

- `artifacts/timestep-policy-summary.md`
- `artifacts/timestep-policy-adjudication.md`
- `artifacts/timestep-policy-analysis-inputs.json`
