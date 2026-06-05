# HPHYS0305 Kickoff Prompt

Scope: local repository science-contract/kernel instrumentation task;
flat-file reads/edits and local comparator execution only; no external
connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/artifacts/snow-rm-window-reclassification.json`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/wepp-forest_260430_baseline/AGENTS.md`

Files:

- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/**`
- Canonical `SC-*` files if missing term/state authority must be added.
- Fixed comparator local instrumentation files under `/workdir/wepp-forest_260430_baseline` only when required for local evidence.
- openWEPP runtime instrumentation/tests only after contract-first gates pass.

Task: execute paired fixed-baseline/openWEPP melt-term and snow-state
instrumentation for H1/H7/H39 target windows. Identify first divergent source
per window before recommending any production correction.

Constraints: contract-first sequencing; canonical SC authority; fixed
comparator provenance; typed guards; no silent defaults; no
canonicalize-and-proceed; no heuristic process physics; no downstream
compensation in WB13/WB17/WB18/WB19/WB12 without source-owned evidence.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including contract amendments, contract-test evidence, pre-implementation gate,
paired term/state ledgers, full-suite context metrics, dual review, review
disposition, dual verification, and worker handoff.
