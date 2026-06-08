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

- local_required_bytes_total: 150274
- threshold_outcome: pass (below policy threshold)
- measurement_method: `wc -c` on Core required-reading paths
- measured_at_utc: 2026-06-08T23:12:40Z

Thresholds:
- Use the canonical thresholds defined in
  `docs/standards/kernel-work-package-preparation.md`.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|------|------|--------------|------------------------|-------------|--------------------|-------------------|-------|
| /workdir/openWEPP/AGENTS.md | Core | Root governance for all package work | Always | Pre-edit | Codex | 2026-06-08T23:12:40Z | |
| /workdir/openWEPP/docs/work-packages/AGENTS.md | Core | Work-package process and artifact requirements | Always | Pre-edit | Codex | 2026-06-08T23:12:40Z | |
| /workdir/openWEPP/docs/codex_exec_plans.md | Core | Package execution contract | Always | Pre-edit | Codex | 2026-06-08T23:12:40Z | |
| /workdir/openWEPP/docs/work-packages/README.md | Core | Process and queue conventions | Always | Pre-edit | Codex | 2026-06-08T23:12:40Z | |
| /workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md | Core | Mechanical refactor execution requirements | Always | Pre-edit | Codex | 2026-06-08T23:12:40Z | |
| /workdir/openWEPP/docs/standards/kernel-work-package-preparation.md | Core | Scope, phasing, and evidentiary expectations | Always | Pre-edit | Codex | 2026-06-08T23:12:40Z | |
| /workdir/openWEPP/docs/prompt_templates/mechanical-refactor-kickoff-template.md | Core | Kickoff-format requirements | Always | Pre-edit | Codex | 2026-06-08T23:12:40Z | |
| /workdir/openWEPP/docs/prompt_templates/required-reading-map-template.md | Core | Reading-map control format | Always | Pre-edit | Codex | 2026-06-08T23:12:40Z | |
| /workdir/openWEPP/docs/work-packages/20260608-refactor020-openwepp-hillslope-orchestrator-runtime-inputs-08-tests-mechanical-modularization-001/package.md | Core | Package-local scope, gates, and write-set | Always | Pre-edit | Codex | 2026-06-08T23:12:40Z | |
| /workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs | Core | Target production file under mechanical modularization | Always | Pre-edit / in-progress | Codex | 2026-06-08T23:12:40Z | 2559 lines pre-scope |
| /workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/mod.rs | Core | Module wiring seam and visibility expectations | Always | Pre-edit | Codex | 2026-06-08T23:12:40Z | |
| /workdir/openWEPP/docs/defect_closure_execplans.md | Conditional | Required only if defect-closure posture activates | Defect-closure transition | Pre-edit when triggered | N/A | n/a | Not applicable to mechanical scope |

## Change Log

| UTC | Agent | Change |
|-----|-------|--------|
| 2026-06-08T23:12:40Z | Codex | Updated from scaffold to executed state: measured bytes, verified timestamps, and closure-ready checks |
