# Required Reading Map Template

Use this template as the canonical starting point for package-level
`artifacts/required-reading-map.md`.

## Authority and Maintenance Responsibility

Agents executing or authoring the package have explicit authority and
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

- local_required_bytes_total: <value>
- threshold_outcome: <OK|WARN|REQUIRES-JUSTIFICATION>
- measurement_method: <command or artifact reference>
- measured_at_utc: <YYYY-MM-DDTHH:MM:SSZ>

Thresholds:
- Use the canonical thresholds defined in
  `docs/standards/kernel-work-package-preparation.md`.

If threshold_outcome is REQUIRES-JUSTIFICATION, include explicit justification
for each heavy required pre-read and explain why it cannot be moved to
On-demand.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|------|------|--------------|-------------------------|-------------|--------------------|-------------------|-------|
| /workdir/openWEPP/AGENTS.md | Core | Root governance for all package work | Always | Pre-edit | Agent | <timestamp> | |
| /workdir/openWEPP/docs/codex_exec_plans.md | Core | Execution-plan contract for package autonomy | Always | Pre-edit | Agent | <timestamp> | |
| /workdir/openWEPP/docs/work-packages/AGENTS.md | Core | Work-package execution, gate, review, and conservation/publication acceptance rules | Always | Pre-edit | Agent | <timestamp> | |
| /workdir/openWEPP/docs/work-packages/README.md | Core | Work-package process and closure conventions | Always | Pre-edit | Agent | <timestamp> | |
| /workdir/openWEPP/docs/work-packages/<id>/package.md | Core | Package-local authority, write set, and gates | Always | Pre-edit | Agent | <timestamp> | |
| /workdir/openWEPP/docs/defect_closure_execplans.md | Conditional | Defect-closure envelope and HOLD legitimacy rules | Defect-closure package | Pre-edit when triggered | Agent | <timestamp> | |
| docs/specifications/science-contract-authoring-procedure.md | Conditional | Contract-authoring procedure authority | Contract or kernel authority edits | Pre-edit when triggered | Agent | <timestamp> | |
| docs/specifications/science-contracts/kernel-process-contract-profile.md | Conditional | Kernel-profile compliance authority | Contract or kernel authority edits | Pre-edit when triggered | Agent | <timestamp> | |
| docs/specifications/science-contracts/contracts/SC-<DOMAIN>-001.md | On-demand | Mechanism-specific canonical contract authority | Only for touched mechanism | Phase-local | Agent | <timestamp> | |
| docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md | Conditional | Baseline provenance requirement | Legacy migration/parity scope | Pre-edit when triggered | Agent | <timestamp> | |
| /workdir/wepp-forest_260430_baseline/src/<file>.for | On-demand | Baseline source provenance for touched mechanism | Legacy migration/parity scope | Phase-local | Agent | <timestamp> | |

## Change Log

| UTC | Agent | Change |
|-----|-------|--------|
| <timestamp> | <agent> | Initialized required-reading map from canonical template. |
