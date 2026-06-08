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

- local_required_bytes_total: 91289
- threshold_outcome: OK (below 400000 bytes)
- measurement_method: `wc -c` on Core required-reading paths
- measured_at_utc: 2026-06-08T23:39:12Z

Thresholds:
- Use the canonical thresholds defined in
  `docs/standards/kernel-work-package-preparation.md`.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|------|------|--------------|------------------------|-------------|--------------------|-------------------|-------|
| /workdir/openWEPP/AGENTS.md | Core | Root governance for all package work | Always | Pre-edit | Codex | 2026-06-08T23:39:12Z | |
| /workdir/openWEPP/docs/work-packages/AGENTS.md | Core | Work-package process and artifact requirements | Always | Pre-edit | Codex | 2026-06-08T23:39:12Z | |
| /workdir/openWEPP/docs/codex_exec_plans.md | Core | Package execution contract | Always | Pre-edit | Codex | 2026-06-08T23:39:12Z | |
| /workdir/openWEPP/docs/work-packages/README.md | Core | Package catalog and process conventions | Always | Pre-edit | Codex | 2026-06-08T23:39:12Z | |
| /workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md | Core | Mechanical refactor execution requirements | Always | Pre-edit | Codex | 2026-06-08T23:39:12Z | |
| /workdir/openWEPP/docs/standards/kernel-work-package-preparation.md | Core | Scope, phasing, and evidentiary expectations | Always | Pre-edit | Codex | 2026-06-08T23:39:12Z | |
| /workdir/openWEPP/docs/prompt_templates/mechanical-refactor-kickoff-template.md | Core | Prompt structure and kickoff controls | Always | Pre-edit | Codex | 2026-06-08T23:39:12Z | |
| /workdir/openWEPP/docs/prompt_templates/required-reading-map-template.md | Core | Required-reading-map control format | Always | Pre-edit | Codex | 2026-06-08T23:39:12Z | |
| /workdir/openWEPP/docs/work-packages/20260608-refactor021-openwepp-tests-integration-parser-runtime-seam-integration-mechanical-modularization-001/package.md | Core | Package-local objective, scope, and gates | Always | Pre-edit | Codex | 2026-06-08T23:39:12Z | |
| /workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs | Core | Target monolith for mechanical modularization | Always / In-progress | Pre-edit / In-progress | Codex | 2026-06-08T23:39:12Z | 2,631 lines pre-scope |
