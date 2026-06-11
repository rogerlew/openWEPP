# Required Reading Map — FDHP01

Status: complete

Agents executing or authoring the package have explicit authority and
responsibility to maintain this file as a living control artifact: keep entries
accurate, re-tier as scope moves, record rationale/trigger changes. A stale or
incomplete map is a governance defect and keeps disposition in HOLD.

## Reading Budget

- local_required_bytes_total: ~111000 (Core + Conditional, local-repo files)
- threshold_outcome: OK
- measurement_method: `stat -c %s` over Core+Conditional paths, 2026-06-10
  (Claude Code, package congruence pass)
- measured_at_utc: 2026-06-10

Thresholds: canonical values in
`docs/standards/kernel-work-package-preparation.md` (`OK <= 400000`).
Note: on-demand SC contracts are large (`SC-SNOWFREEZE-001` ~150 KB,
`SC-WATBAL-001` ~326 KB) and must stay phase-local / section-targeted.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|------|------|--------------|-------------------------|-------------|--------------------|-------------------|-------|
| /workdir/openWEPP/AGENTS.md | Core | Root governance | Always | Pre-edit | Agent | 2026-06-10 | |
| /workdir/openWEPP/docs/codex_exec_plans.md | Core | Execution-plan contract | Always | Pre-edit | Agent | 2026-06-10 | |
| /workdir/openWEPP/docs/work-packages/README.md | Core | Package process + closure conventions | Always | Pre-edit | Agent | 2026-06-10 | |
| docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/package.md | Core | Package-local authority, envelope, gates | Always | Pre-edit | Agent | 2026-06-10 | |
| docs/defect_closure_execplans.md | Conditional | DC-ExecPlan envelope + HOLD legitimacy | Defect-closure package | Pre-edit | Agent | 2026-06-11 | applies |
| docs/specifications/science-contract-authoring-procedure.md | Conditional | Contract-authoring authority | Contract edits (SC-SNOWFREEZE-001 amendment) | Pre-edit | Agent | 2026-06-11 | applies |
| docs/specifications/science-contracts/kernel-process-contract-profile.md | Conditional | Kernel-profile compliance | Kernel decision-logic edits | Pre-edit | Agent | 2026-06-11 | applies |
| docs/specifications/science-contracts/index.md | Conditional | Contract registry | Contract edits | Pre-edit | Agent | 2026-06-11 | applies |
| docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md | Conditional | Baseline provenance | Legacy parity scope | Pre-edit | Agent | 2026-06-11 | applies |
| docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md | On-demand | Primary mechanism contract (INV-006/-012/-013, GAP-002) | Contract amendment + implementation phases | Phase-local | Agent | 2026-06-11 | section-targeted |
| docs/specifications/science-contracts/contracts/SC-WATBAL-001.md | On-demand | Downstream closure authority | Closure validation phase | Phase-local | Agent | 2026-06-10 | ~326 KB; section-targeted |
| docs/decisions/0011-…, 0017-…, 0018-… | On-demand | Strategy/comparator/DC policy | Orientation as needed | Phase-local | Agent | 2026-06-10 | |
| docs/ROADMAP.md | On-demand | Queue position (item 1) | Handoff phase | Phase-local | Agent | 2026-06-10 | |
| docs/backlog/20260607-frost-depth-model-heat-flow-parity.md | On-demand | Promoted backlog authority | M1 | Phase-local | Agent | 2026-06-10 | |
| docs/work-packages/20260608-fdmc01-frost-depth-comparator-characterization-001/ | On-demand | Sized gap + metrics to close | M1 + validation | Phase-local | Agent | 2026-06-10 | |
| docs/work-packages/20260608-fq4-ksflag-frost-activation-closure-001/ | On-demand | Activation non-regression baseline | Validation | Phase-local | Agent | 2026-06-10 | |
| /workdir/wepp-forest_260430_baseline/src/frostn.for (+ frzng/frznw/frsoil/mlttp/mltbtm/watdst) | On-demand | Legacy heat-flow reference (flag, not target) | Implementation phase | Phase-local | Agent | 2026-06-11 | external to repo budget; read for heat-flow/depth-bound localization |
| docs/prompt_templates/owcmp-comparator-runner-guidance.md | On-demand | Comparator execution procedure | Comparator runs | Phase-local | Agent | 2026-06-10 | |
| docs/work-packages/20260608-fdmc01-frost-depth-comparator-characterization-001/artifacts/frost_depth_characterization_summary.json | On-demand | FDMC01 quantified depth/duration gap | M1 + validation | Phase-local | Subagent + Agent | 2026-06-11 | proxy cap and legacy range confirmed |
| docs/work-packages/20260608-frostval01-ksflag-frost-single-ofe-closure-validation-001/artifacts/rerun-20260611-frostval01.md | On-demand | FROSTVAL01 activation/closure non-regression baseline | Validation | Phase-local | Agent | 2026-06-11 | referenced by package dependency note |

## Change Log

| UTC | Agent | Change |
|-----|-------|--------|
| 2026-06-10 | Claude Code | Initialized map during scaffold-congruence pass (post-REFACTOR/OWCMP/§4a guidance updates). |
| 2026-06-11 | Codex | Updated status to complete; recorded SC-SNOWFREEZE/legacy heat-flow reads, FDMC01 comparator artifact evidence, FROSTVAL01 non-regression baseline, and comparator subagent finding that no FDMC01 owcmp suite manifest exists. |
