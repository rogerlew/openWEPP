# WBVAL02 Kickoff Prompt

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
- `/workdir/openWEPP/docs/work-packages/20260606-wbval02-simimpl28-radbound-defect-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `/workdir/openWEPP/docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `/workdir/openWEPP/docs/backlog/20260605-snow-code-deferred-science-review.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0277-climate-radiation-physical-flux-guard-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0277-climate-radiation-physical-flux-guard-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/artifacts/run-manifest.md`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/artifacts/single-ofe-closure-ledger.md`

Files:

- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-climate-runtime-adapter/src/lib.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/**`
- `docs/work-packages/20260606-wbval02-simimpl28-radbound-defect-closure-001/**`

Task: close defect `WBVAL02-CLIM-RUNTIME-E-017-RADBOUND` end-to-end for
`p2`, `p4`, `p6`, `p9`, `p14`, and `p17`. Diagnose internally until the
mechanism is owned or a branch condition is met. If the mechanism is owned and
contract-supported, amend contracts and tests, record the pre-implementation
gate, implement the correction, validate, and complete dual review and
disposition.

Constraints: contract-first sequencing; canonical `SC-*` authority; pinned
baseline provenance where migration applies; typed guards; no silent defaults;
no radiation clipping; no guard loosening without contract proof; no
canonicalize-and-proceed for domain violations; do not edit snowmelt,
percolation, ET, runoff, or WAT residual surfaces for this package.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked. Do not request
a new package for intermediate diagnostic steps.

Outputs: update package artifacts and disposition for all completed phases.
