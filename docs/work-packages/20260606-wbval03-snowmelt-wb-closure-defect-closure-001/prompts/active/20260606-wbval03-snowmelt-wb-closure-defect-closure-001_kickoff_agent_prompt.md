# WBVAL03 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval03-snowmelt-wb-closure-defect-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `/workdir/openWEPP/docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `/workdir/openWEPP/docs/backlog/20260605-snow-code-deferred-science-review.md`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/artifacts/run-manifest.md`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/artifacts/single-ofe-closure-ledger.md`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/artifacts/worker-handoff.md`

Files:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-summary-accumulator/src/lib.rs`
- `crates/openwepp-watershed-output/src/writers.rs`
- `tests/integration/**`
- `docs/work-packages/20260606-wbval03-snowmelt-wb-closure-defect-closure-001/**`

Task: close defects `WBVAL03-HKERNEL-WB11-PERC-E-003-J95` and
`WBVAL03-WAT-LEDGER-CONSERVATION-RESIDUAL` end-to-end. Diagnose internally until
each mechanism is owned or a branch condition is met. Complete the balance
identity before attributing the emitted-ledger residual. If a mechanism is owned
and contract-supported, amend contracts and tests, record the pre-implementation
gate, implement the correction, validate, and complete dual review and
disposition.

Constraints: contract-first sequencing; canonical `SC-*` authority; pinned
baseline provenance where migration applies; typed guards; no silent defaults;
no guard loosening; no heuristic snowmelt/percolation/storage math; no
canonicalize-and-proceed for domain violations; do not fix climate radiation in
this package; do not reopen the suspended snow/`RM` comparator route.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked. Do not request
a new package for intermediate diagnostic steps.

Outputs: update package artifacts and disposition for all completed phases.
