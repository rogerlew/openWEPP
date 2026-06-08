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

- local_required_bytes_total: 88738
- threshold_outcome: OK
- measurement_method: `wc -c` on Core required-reading paths and package scope files
- measured_at_utc: 2026-06-08T21:19:14Z

Thresholds:
- Use the canonical thresholds defined in
  `docs/standards/kernel-work-package-preparation.md`.

If threshold_outcome is REQUIRES-JUSTIFICATION, include explicit justification
for each heavy required pre-read and explain why it cannot be moved to
On-demand.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|------|------|--------------|-------------------------|-------------|--------------------|-------------------|-------|
| /workdir/openWEPP/AGENTS.md | Core | Root governance for all package work | Always | Pre-edit | Agent | 2026-06-08T21:19:14Z | Read |
| /workdir/openWEPP/docs/work-packages/AGENTS.md | Core | Work-package governance and artifact requirements | Always | Pre-edit | Agent | 2026-06-08T21:19:14Z | Read |
| /workdir/openWEPP/docs/codex_exec_plans.md | Core | Execution-plan contract for package autonomy | Always | Pre-edit | Agent | 2026-06-08T21:19:14Z | Read |
| /workdir/openWEPP/docs/work-packages/README.md | Core | Work-package process and closure conventions | Always | Pre-edit | Agent | 2026-06-08T21:19:14Z | Read |
| /workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md | Core | Mechanical refactor execution and gate requirements | Always | Pre-edit | Agent | 2026-06-08T21:19:14Z | Read |
| /workdir/openWEPP/docs/work-packages/20260608-refactor017-openwepp-runner-hillslope-tests03-publication-mechanical-modularization-001/package.md | Core | Package-local authority, write set, and gates | Always | Pre-edit | Agent | 2026-06-08T21:19:14Z | Read |
| crates/openwepp-runner/src/hillslope/tests03/publication.rs | Core | Target test file under modularization | Always | Pre-edit | Agent | 2026-06-08T21:19:14Z | Baseline package scope: 2079 lines, 49 tests |
| crates/openwepp-runner/src/hillslope/03_tests.rs | Core | Publication module wiring seam (`mod publication`) | Always | Pre-edit | Agent | 2026-06-08T21:19:14Z | Verified include path and module shape remained stable |
| /workdir/openWEPP/docs/defect_closure_execplans.md | Conditional | Defect-closure envelope and HOLD legitimacy rules | Defect-closure package only | Pre-edit when triggered | Agent | 2026-06-08T21:19:14Z | Read |
| docs/specifications/science-contract-authoring-procedure.md | Conditional | Contract-authoring procedure authority | Contract or kernel authority edits | Not triggered | Agent | 2026-06-08T21:19:14Z | Read for posture; no edits required |
| docs/specifications/science-contracts/kernel-process-contract-profile.md | Conditional | Kernel-profile compliance authority | Contract or kernel authority edits | Not triggered | Agent | 2026-06-08T21:19:14Z | Read for posture; no edits required |
| docs/specifications/science-contracts/index.md | Conditional | SC index and authority routing | Contract or kernel authority edits | Not triggered | Agent | 2026-06-08T21:19:14Z | Read for posture |
| docs/specifications/science-contracts/contracts/SC-PLANT-001.md | On-demand | Contract reference for PL activation assertions in publication tests | Only if contract edits occur | Not triggered | Agent | 2026-06-08T21:19:14Z | No contract edits in this package |

## Change Log

| UTC | Agent | Change |
|-----|-------|--------|
| 2026-06-08T21:19:14Z | Claude Opus | Closed map with measured byte budget, verified entries, and explicit non-triggered conditional/on-demand items. |
