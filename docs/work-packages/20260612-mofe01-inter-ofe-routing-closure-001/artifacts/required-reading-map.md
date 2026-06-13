# Required Reading Map — MOFE01

Status: updated through M-E0 executed-hold

Agents executing or authoring the package maintain this as a living control
artifact (tiers, rationale, read timing); a stale map is a governance defect.

## Reading Budget

- local_required_bytes_total: ~115000 (Core + Conditional, local-repo files)
- threshold_outcome: OK
- measurement_method: estimated from the FDHP01 measurement plus the two
  package-local artifacts; re-measure at M-A kickoff
- measured_at_utc: 2026-06-12

## Map

| Path | Tier | Why required | Trigger | Read timing |
|------|------|--------------|---------|-------------|
| /workdir/openWEPP/AGENTS.md | Core | Root governance | Always | Pre-edit |
| /workdir/openWEPP/docs/codex_exec_plans.md | Core | Execution-plan contract | Always | Pre-edit |
| /workdir/openWEPP/docs/work-packages/README.md | Core | Package process | Always | Pre-edit |
| package.md (this package) | Core | Envelope, posture, gates | Always | Pre-edit |
| artifacts/mofe-staged-increment-plan.md | Core | Dispatch governor | Always | Pre-edit |
| docs/defect_closure_execplans.md | Conditional | DC rules | Defect closure within increments | Pre-edit |
| science-contract-authoring-procedure.md + kernel-process-contract-profile.md + index.md | Conditional | Contract edits (M-B) | Contract/kernel edits | Pre-edit when triggered |
| docs/decisions/0012-…baseline-anchor.md | Conditional | Legacy provenance | Legacy map (M-A) | Pre-edit when triggered |
| SC-RUNOFFPART-001 / SC-WATBAL-001 / SC-SYSTEM-001 | On-demand | Routing/closure authority | Touched mechanism | Phase-local, section-targeted |
| ADR-0011/0017/0018; docs/ROADMAP.md | On-demand | Strategy/comparator posture | Orientation | Phase-local |
| FDHP01 staged plan + scope artifact | On-demand | Execution-shape template + failure modes | All increments | Phase-local |
| wepppy 20260502_mofe_flagged_hillslope_triage | On-demand | Legacy defect taxonomy (calibration, not authority) | M-A item 2 | Phase-local |
| /workdir/wepp-forest_260430_baseline/src/ (per-plane loop lineage) | On-demand | Legacy routing map | M-A item 3 | Phase-local; cite lines read |
| /wc1/runs/ar/arboreal-dendrite/wepp/ | On-demand | Substrate | M-A onward | Phase-local |

## Change Log

| UTC | Agent | Change |
|-----|-------|--------|
| 2026-06-12 | Claude Code | Initialized at scaffold. |
| 2026-06-12 | Codex | Executed M-A; read package, staged increment plan, work-package/science/crate/test agent guides, current runner seams, and pinned legacy routing files. |
| 2026-06-12 | Codex | Executed M-B; reread package governance and touched SC-RUNOFFPART/SC-WATBAL authority, hydrology orchestrator seams, runner seed seam, and WB11/WB14/runner tests. |
| 2026-06-13 | Codex | Executed M-C; reread staged plan, routing scope, M-B evidence, WB13/MOFE publication authority, runner WAT publication path, summary accumulator row guards, and scheduler publication provenance. |
| 2026-06-13 | Codex | Executed M-C2; reread M-C evidence, work-package non-deferral governance, scheduler/writeback request seams, MOFE hourly carry producer/consumer seams, WB13/WAT publication path, and M-B carry tests. |
| 2026-06-13 | Codex | Executed M-D; reread M-C2 evidence, staged M-D scope, package/work-package/science/crate/test governance, current scheduler/writeback/publication seams, SC-RUNOFFPART/SC-WATBAL/SC-SYSTEM MOFE addenda, and pinned legacy `irs`/`rochek`/WATBAL routing lineage. |
| 2026-06-13 | Codex | Executed M-E0; reread M-D architecture, staged M-E0/M-E scope, work-package/science/crate/test governance, SC-RUNOFFPART/SC-WATBAL/SC-SYSTEM current MOFE authority, science-contract registry, scheduler/writeback/publication seams, and M-B contract authority tests. |
