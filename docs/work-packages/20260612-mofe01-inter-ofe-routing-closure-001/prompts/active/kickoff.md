# MOFE01 Kickoff — inter-OFE routing closure (rung 3)

Execution mode: staged increments (one per dispatch) per
`artifacts/mofe-staged-increment-plan.md`. This kickoff governs increment
**M-A** (characterization + routing scope; no production edits); later
increments are dispatched by the plan's dispatch line.

Subagent authorization (REQUIRED, not optional): this prompt explicitly
authorizes subagent spawning/delegation to `comparator_suite_runner`
(gpt-5.3-codex-spark) for all heavy batch/closure/comparator runs
(37-hillslope cohort runs, legacy-output parsing batches, workspace test
loops) and to review/verification subagents for the dual artifacts.
**Do NOT run heavy batch work on the parent model** unless the subagent is
unavailable; record command-level evidence as justification
(`docs/standards/prompt-wording-guidance.md` §4a). Outputs: compact metrics
+ artifact paths; no source/contract edits by the runner.

Autonomy: execute the dispatched increment end-to-end per the staged plan
without asking for direction on intermediate steps. Hard stops: the plan's
conservation gates, single-OFE anchor, protected boundaries, and the
no-production-edits rule for M-A. Operator decisions route back per the
plan.

## The assignment (M-A)

Per `artifacts/mofe-staged-increment-plan.md` increment M-A:

1. Characterize openWEPP's current multi-OFE behavior on
   `/wc1/runs/ar/arboreal-dendrite/wepp` (37 hillslopes; graded 1–5-OFE
   ladder; 15-OFE observe-only).
2. Measure legacy's per-OFE-count closure defect from the on-disk
   `output/H*.wat.dat` (the comparator-trust calibration — see package.md
   posture: legacy is known-defective in this dimension).
3. Produce `artifacts/mofe-routing-port-scope.md` — legacy routing
   state-machine map (**read the cited lines; do not infer from symbol
   tables** — recorded FDHP01-Dh lesson), openWEPP seam mapping,
   state-shape proposal with alias table, red-test definitions, sizing.

## Required reading

Maintain `artifacts/required-reading-map.md` as a living artifact.

Reading budget (local-repo pre-edit reads, Core + Conditional): ~115,000
bytes → **OK** (`<=400000`, thresholds per
`docs/standards/kernel-work-package-preparation.md`). Large SC contracts
load phase-locally, section-targeted.

Core (always, before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001/package.md`
- `docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001/artifacts/mofe-staged-increment-plan.md`

Conditional (triggered — all apply to this package):
- `docs/defect_closure_execplans.md`
- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  `docs/specifications/science-contracts/index.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

On-demand (phase-local, touched mechanisms only):
- `SC-RUNOFFPART-001.md`, `SC-WATBAL-001.md`, `SC-SYSTEM-001.md`
- ADR-0011/0017/0018; `docs/ROADMAP.md` (item 1, substrate + comparator
  posture)
- FDHP01 staged plan + scope artifact (the execution-shape template and its
  recorded failure modes)
- wepppy `docs/work-packages/20260502_mofe_flagged_hillslope_triage/`
  (legacy defect-family taxonomy — calibration evidence, not authority)
- Legacy pinned baseline `/workdir/wepp-forest_260430_baseline/src/`
  (per-plane loop / run-on lineage — discovered in M-A, cited by line)
- Substrate `/wc1/runs/ar/arboreal-dendrite/wepp/`
