# Required Reading Map

## Authority and Maintenance Responsibility

Agents executing or authoring this package have explicit authority and
responsibility to maintain this file as a living control artifact.

Required maintenance posture:
- Keep entries accurate, complete, and current with package scope.
- Add newly required readings immediately when scope expands.
- Re-tier readings when they move between Core, Conditional, and On-demand.
- Remove stale entries only after verifying they are no longer required.
- Record rationale and trigger updates when authority assumptions change.

A stale or incomplete required-reading map is a governance defect and must keep
package disposition in HOLD until corrected.

## Reading Budget

- local_required_bytes_total: queued
- threshold_outcome: queued
- measurement_method: `wc -c` on Core required-reading paths
- measured_at_utc: queued

Thresholds:
- Use the canonical thresholds defined in
  `docs/standards/kernel-work-package-preparation.md`.

If threshold_outcome is REQUIRES-JUSTIFICATION, include explicit justification
for each heavy pre-read and explain why it cannot be moved to On-demand.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|------|------|--------------|------------------------|-------------|--------------------|-------------------|-------|
| /workdir/openWEPP/AGENTS.md | Core | Root governance for all package work | Always | Pre-edit | Agent | queued | |
| /workdir/openWEPP/docs/work-packages/AGENTS.md | Core | Package governance and artifact requirements | Always | Pre-edit | Agent | queued | |
| /workdir/openWEPP/docs/codex_exec_plans.md | Core | Work-package execution contract | Always | Pre-edit | Agent | queued | |
| /workdir/openWEPP/docs/work-packages/README.md | Core | Package catalog and route-of-record conventions | Always | Pre-edit | Agent | queued | |
| /workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md | Core | Mechanical refactor gates and required closure | Always | Pre-edit | Agent | queued | |
| /workdir/openWEPP/docs/standards/kernel-work-package-preparation.md | Core | Scope, review, and disposition scaffolding requirements | Always | Pre-edit | Agent | queued | |
| /workdir/openWEPP/docs/work-packages/20260608-refactor021-openwepp-tests-integration-parser-runtime-seam-integration-mechanical-modularization-001/package.md | Core | Package-local objective and constraints | Always | Pre-edit | Agent | queued | |
| /workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs | Core | Target integration file for modularization | During Phase A | Pre-edit | Agent | queued | |

## Change Log

| UTC | Agent | Change |
|-----|-------|--------|
| 2026-06-08T00:00:00Z | Codex | Initial required-reading map scaffold created. |
