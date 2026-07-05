# Required Reading Map

Status: executed
Evidence mode: Static

## Authority and Maintenance Responsibility

Agents executing or authoring the package have explicit authority and
responsibility to maintain this file as a living control artifact.

## Reading Budget

- local_required_bytes_total: 304425
- threshold_outcome: OK
- measurement_method: `wc -c` over Core + triggered Conditional pre-edit
  readings, plus package-local scaffold estimate
- measured_at_utc: 2026-07-05T22:24:39Z

Thresholds use `docs/standards/kernel-work-package-preparation.md`.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|---|---|---|---|---|---|---|---|
| `/home/workdir/openWEPP/AGENTS.md` | Core | Root governance for all package work | Always | Pre-edit | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/codex_exec_plans.md` | Core | Execution-plan and review/disposition contract | Always | Pre-edit | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/work-packages/AGENTS.md` | Core | Work-package gates, review, and evidence rules | Always | Pre-edit | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/work-packages/README.md` | Core | Package discovery and current active/held status | Always | Pre-edit | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-d9-dval-disposition-001/package.md` | Core | Package-local objective, scope, write set, and gates | Always | Pre-edit | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md` | Conditional | Canonical `SC-*` editing governance | Contract edit or kernel authority edit | Pre-edit when triggered | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md` | Conditional | Contract-authoring procedure | Contract edit | Pre-edit when triggered | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional | Kernel-profile compliance | Contract or kernel authority edit | Pre-edit when triggered | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md` | Conditional | Contract registry lifecycle | Registry/profile status edit | Pre-edit when triggered | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | On-demand | Canonical Lane D routing contract | Contract amendment or D-val status read | Phase-local | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/planning/mofe-fidelity-campaign-strategy.md` | On-demand | D9-D15 sequencing and campaign context | Scope/status check | Phase-local | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/work-packages/20260702-mofefid-d8-routing-fidelity-defect-closure-001/package.md` | On-demand | D8 corrections and current D-val dispositions | D9 baseline/disposition | Phase-local | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/work-packages/20260702-mofefid-d8-routing-fidelity-defect-closure-001/artifacts/baseline-diagnostics.md` | On-demand | D8 corrected metrics | D9 baseline/disposition | Phase-local | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/work-packages/20260702-mofefid-d8-routing-fidelity-defect-closure-001/artifacts/forcing-operand-audit.md` | On-demand | Case 1-3 operand verdict rationale | D9 case disposition | Phase-local | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/work-packages/20260702-mofefid-d8-routing-fidelity-defect-closure-001/artifacts/execution-report.md` | On-demand | D8 final verdicts and Case 4 boundary | D9 case/handoff disposition | Phase-local | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-laned-seam-implementation-001/package.md` | On-demand | Seam implementation status | Activation-boundary check | Phase-local | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-laned-activation-increment-001/package.md` | On-demand | Runtime shadow status and flip blockers | Activation-boundary check | Phase-local | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md` | On-demand | Erosion hourly-shape dependency boundary | D13 handoff references only | Phase-local | Agent | 2026-07-05 | |
| `references/copyrighted/Papanicolaou2018.md` | On-demand | Lane D paper authority | D-val source provenance | Phase-local | Agent | 2026-07-05 | Copyright governance applies. |
| `references/copyrighted/Papanicolaou2018-supplemental/` | On-demand | Validation input/series provenance | D-val source provenance | Phase-local | Agent | 2026-07-05 | Do not republish raw workbook rows. |
| `/home/workdir/openWEPP/docs/standards/local-ci-gate-selection.md` | On-demand | Local gate tier standard | Gate planning | Phase-local | Agent | 2026-07-05 | |
| `/home/workdir/openWEPP/tools/dval/compare_dval.py` | On-demand | Figure 4 D-val harness | Case 1-3 reruns | Phase-local | Agent + subagent | 2026-07-05 | |
| `/home/workdir/openWEPP/tools/dval/zone_taxonomy.py` | On-demand | Figure 9 taxonomy harness | Zone taxonomy execution | Phase-local | Agent + subagent | 2026-07-05 | Added by D9. |

## Change Log

| UTC | Agent | Change |
|---|---|---|
| 2026-07-05T22:24:39Z | Codex | Initialized required-reading map from canonical template. |
| 2026-07-05T22:38:46Z | Codex | Recorded triggered contract, D8 evidence, gate-selection, and D-val harness reads. |
