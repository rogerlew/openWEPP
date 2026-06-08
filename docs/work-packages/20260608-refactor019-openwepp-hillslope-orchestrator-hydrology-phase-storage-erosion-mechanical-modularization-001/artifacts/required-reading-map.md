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

- local_required_bytes_total: 76162
- threshold_outcome: pass
- measurement_method: `wc -c` on Core required-reading paths
- measured_at_utc: 2026-06-08T22:50:27Z

Thresholds:
- Use the canonical thresholds defined in
  `docs/standards/kernel-work-package-preparation.md`.

If threshold_outcome is REQUIRES-JUSTIFICATION, include explicit justification
for each heavy required pre-read and explain why it cannot be moved to
On-demand.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|------|------|--------------|-------------------------|-------------|--------------------|-------------------|-------|
| /workdir/openWEPP/AGENTS.md | Core | Root governance for all package work | Always | Pre-edit | Agent | 2026-06-08T22:50:27Z | |
| /workdir/openWEPP/docs/work-packages/AGENTS.md | Core | Work-package governance and artifact requirements | Always | Pre-edit | Agent | 2026-06-08T22:50:27Z | |
| /workdir/openWEPP/docs/codex_exec_plans.md | Core | Execution-plan contract for package autonomy | Always | Pre-edit | Agent | 2026-06-08T22:50:27Z | |
| /workdir/openWEPP/docs/work-packages/README.md | Core | Work-package process and closure conventions | Always | Pre-edit | Agent | 2026-06-08T22:50:27Z | |
| /workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md | Core | Mechanical refactor execution and gate requirements | Always | Pre-edit | Agent | 2026-06-08T22:50:27Z | |
| /workdir/openWEPP/docs/work-packages/20260608-refactor019-openwepp-hillslope-orchestrator-hydrology-phase-storage-erosion-mechanical-modularization-001/package.md | Core | Package-local authority, write set, and gates | Always | Pre-edit | Agent | 2026-06-08T22:50:27Z | |
| crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion.rs | Core | Target production file under modularization | Always | Pre-edit | Agent | 2026-06-08T22:50:27Z | 2110 lines, 4 pub fn entry points, no #[test] items (pre-refactor baseline) |
| crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/mod.rs | Core | Module wiring seam for the phase module; confirm `mod` exports remain stable | Always | Pre-edit | Agent | 2026-06-08T22:50:27Z | No export shape change observed |
| /workdir/openWEPP/docs/defect_closure_execplans.md | Conditional | Defect-closure envelope and HOLD legitimacy rules | Defect-closure package only | Pre-edit when triggered | N/A | n/a | Not applicable (mechanical refactor scope) |
| docs/specifications/science-contract-authoring-procedure.md | Conditional | Contract-authoring procedure authority | Contract or kernel authority edits | Pre-edit when triggered | N/A | n/a | Not applicable (no contract authorship in scope) |
| docs/specifications/science-contracts/kernel-process-contract-profile.md | Conditional | Kernel-profile compliance authority | Contract or kernel authority edits | Pre-edit when triggered | N/A | n/a | Not applicable (no contract amendments) |
| docs/specifications/science-contracts/index.md | Conditional | SC index and authority routing | Contract or kernel authority edits | Pre-edit when triggered | N/A | n/a | Not applicable (no contract edits) |
| docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md | Conditional | Baseline provenance requirement | Legacy migration/parity scope | Pre-edit when triggered | N/A | n/a | Not applicable (legacy migration not in scope) |
| docs/specifications/science-contracts/contracts/SC-<DOMAIN>-001.md | On-demand | Mechanism-specific canonical contract authority | Only for touched mechanism | Phase-local | N/A | n/a | Not applicable (no contract touched) |

## Change Log

| UTC | Agent | Change |
|-----|-------|--------|
| 2026-06-08T00:00:00Z | Claude Code (claude-sonnet-4-6) | Initialized required-reading map from canonical template during package scaffold. |
| 2026-06-08T22:50:27Z | Claude Code (claude-sonnet-4-6) | Captured budget via `wc -c` and marked threshold outcome PASS. |
