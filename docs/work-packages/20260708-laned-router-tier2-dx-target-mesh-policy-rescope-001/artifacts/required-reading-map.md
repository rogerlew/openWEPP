# Required Reading Map

Status: EXECUTED-HOLD-DX-REFERENCE-ADEQUACY

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

- local_required_bytes_total: 350747
- threshold_outcome: OK
- measurement_method: `wc -c` over Core paths listed below; executor must
  remeasure after any required-reading change.
- measured_at_utc: 2026-07-07T00:00:00Z

Thresholds from `docs/standards/kernel-work-package-preparation.md`:
- `OK`: `<=400000` bytes.
- `WARN`: `>400000` bytes.
- `REQUIRES-JUSTIFICATION`: `>800000` bytes.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|------|------|--------------|-------------------------|-------------|--------------------|-------------------|-------|
| `/home/workdir/openWEPP/AGENTS.md` | Core | Root repository governance | Always | Pre-edit | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/codex_exec_plans.md` | Core | Package autonomy and ExecPlan expectations | Always | Pre-edit | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/work-packages/AGENTS.md` | Core | Work-package execution, gates, review, verification, and hold rules | Always | Pre-edit | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/work-packages/README.md` | Core | Work-package catalog and active/held pointer | Always | Pre-edit | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/package.md` | Core | Package-local objective, scope, phases, gates, and exit criteria | Always | Pre-edit | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/required-reading-map.md` | Core | Living required-reading control artifact | Always | Pre-edit | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/decisions/0037-abandon-hybrid-implicit-stepping.md` | Core | ADR that abandons hybrid and demotes H2637 to synthetic stress | Always | Pre-edit | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-hybrid-abandonment-removal-001/artifacts/worker-handoff.md` | Core | Names this re-scope as the next package and defines surviving question | Always | Pre-edit | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-tier2-mesh-resolution-adjudication-001/package.md` | Core | Superseded package whose old hybrid-era framing must not be executed | Always | Pre-edit | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/selected-cohort-materialization.json` | Core | Selected-cohort member/run-file provenance | Always | Pre-edit | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-hybrid-abandonment-removal-001/artifacts/plain-identity-materialization.json` | Core | Plain active selected-cohort materialization after hybrid strip | Always | Pre-edit | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/backlog/20260706-laned-router-numerics-performance-tiers.md` | Core | Tiered numerics context and original Tier-2 rationale | Always | Pre-edit | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md` | Conditional | Canonical `SC-*` governance | Before contract, test, or kernel semantic edits | Pre-edit when triggered | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md` | Conditional | Contract authoring and amendment procedure | Before changing canonical `SC-*` text | Pre-edit when triggered | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional | Kernel contract profile and BEI expectations | Before changing canonical invariant, BEI, or profile text | Pre-edit when triggered | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md` | Conditional | Science-contract lifecycle registry | Before registry edits | Pre-edit when triggered | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/crates/AGENTS.md` | Conditional | Rust crate governance | Before Rust edits under `crates/` | Pre-edit when triggered | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/tests/AGENTS.md` | Conditional | Test governance | Before test edits under `tests/` | Pre-edit when triggered | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | On-demand | Surviving Lane D plain active routing authority | When proposing or implementing mesh-policy tolerance/guards | Phase-local | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs` | On-demand | Active plain mesh builder and runtime counters | When inventorying or editing active runtime | Phase-local | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/` | On-demand | Kinematic-wave mesh/router and Case-4 tests | When running/editing oracle ladders or router code | Phase-local | Agent | 2026-07-07T00:00:00Z | |
| `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/` | On-demand | Runner projection into active route config | When editing runner handoff or config projection | Phase-local | Agent | 2026-07-07T00:00:00Z | |

## Change Log

| UTC | Agent | Change |
|-----|-------|--------|
| 2026-07-07T00:00:00Z | Codex | Initialized required-reading map from canonical template. |
| 2026-07-07T00:00:00Z | Codex | Marked map current for executed-hold closure after final QA review. |
