# ARCH-01 Kickoff Agent Prompt

You are executing `20260520-arch01-subsystem-map-and-contract-spine`.

## Mission
Produce architecture-discovery artifacts that let openWEPP start subsystem
implementation under ADR-0011 without waiting for full legacy re-kernelization.

## Required Outputs
- `artifacts/subsystem-inventory.md`
- `artifacts/subsystem-dependency-map.md`
- `artifacts/state-surface-catalog.csv`
- `artifacts/invariant-catalog.md`
- `artifacts/reference-citation-matrix.md`
- `artifacts/comparator-confidence-tier-policy.md`
- `artifacts/architecture-decision-summary.md`
- `artifacts/legacy-run-sidecar-compatibility-bridge.md`

## Evidence Rules
- Tag claims as `[DIRECT]` or `[INFERENCE]`.
- Distinguish `Static` vs `Ran` evidence mode.
- Treat legacy static analysis as secondary authority.

## Comparator Policy
- Tier A: single OFE + daily water balance = higher-confidence acceptance signal.
- Tier B: hourly/watershed = investigation signal.

## Out of Scope
- Broad kernel implementation.
- Large-scale cohort reruns.
