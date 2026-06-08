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
for each heavy required pre-read and explain why it cannot be moved to
On-demand.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|------|------|--------------|-------------------------|-------------|--------------------|-------------------|-------|
| /workdir/openWEPP/AGENTS.md | Core | Root governance for all package work | Always | Pre-edit | Agent | queued | |
| /workdir/openWEPP/docs/work-packages/AGENTS.md | Core | Work-package governance and artifact requirements | Always | Pre-edit | Agent | queued | |
| /workdir/openWEPP/docs/codex_exec_plans.md | Core | Execution-plan contract for package autonomy | Always | Pre-edit | Agent | queued | |
| /workdir/openWEPP/docs/work-packages/README.md | Core | Work-package process and closure conventions | Always | Pre-edit | Agent | queued | |
| /workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md | Core | Mechanical refactor execution and gate requirements | Always | Pre-edit | Agent | queued | |
| /workdir/openWEPP/docs/work-packages/20260608-refactor018-openwepp-input-contract-parsers-hbp-mechanical-modularization-001/package.md | Core | Package-local authority, write set, and gates | Always | Pre-edit | Agent | queued | |
| crates/openwepp-input-contract/src/parsers/hbp.rs | Core | Target production file under modularization | Always | Pre-edit | Agent | queued | 2095 lines, 4 pub fn entry points, no #[test] items |
| crates/openwepp-input-contract/src/parsers/mod.rs | Core | Module wiring seam: `pub mod hbp;` at line 5 — unchanged by refactor (Rust resolves both hbp.rs and hbp/mod.rs) | Always | Pre-edit | Agent | queued | Verify seam is stable before edits |
| /workdir/openWEPP/docs/defect_closure_execplans.md | Conditional | Defect-closure envelope and HOLD legitimacy rules | Defect-closure package only | Pre-edit when triggered | Agent | queued | |
| docs/specifications/science-contract-authoring-procedure.md | Conditional | Contract-authoring procedure authority | Contract or kernel authority edits | Pre-edit when triggered | Agent | queued | |
| docs/specifications/science-contracts/kernel-process-contract-profile.md | Conditional | Kernel-profile compliance authority | Contract or kernel authority edits | Pre-edit when triggered | Agent | queued | |
| docs/specifications/science-contracts/index.md | Conditional | SC index and authority routing | Contract or kernel authority edits | Pre-edit when triggered | Agent | queued | |
| docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md | Conditional | Baseline provenance requirement | Legacy migration/parity scope | Pre-edit when triggered | Agent | queued | |
| docs/specifications/science-contracts/contracts/SC-<DOMAIN>-001.md | On-demand | Mechanism-specific canonical contract authority | Only for touched mechanism | Phase-local | Agent | queued | |

## Change Log

| UTC | Agent | Change |
|-----|-------|--------|
| 2026-06-08T00:00:00Z | Claude Code (claude-sonnet-4-6) | Initialized required-reading map from canonical template during package scaffold. |
