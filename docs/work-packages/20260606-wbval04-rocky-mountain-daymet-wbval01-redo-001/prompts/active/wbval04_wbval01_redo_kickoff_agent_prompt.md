# WBVAL04 WBVAL01 Redo Kickoff Prompt

Scope: local repository validation package; flat-file reads/edits and local
openWEPP command execution only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval04-rocky-mountain-daymet-wbval01-redo-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `/workdir/openWEPP/docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval02-simimpl28-radbound-defect-closure-001/`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval03-snowmelt-wb-closure-defect-closure-001/`
- `/workdir/wepppy/docs/adrs/ADR-0006-observed-daymet-radiation-toa-normalization.md`
- `/workdir/wepppy/docs/work-packages/20260606_indispensable_presenter_daymet_radiation_bounds/artifacts/execution_evidence.md`

Files:

- `docs/work-packages/20260606-wbval04-rocky-mountain-daymet-wbval01-redo-001/**`
- `docs/work-packages/README.md` only if lifecycle status changes are needed.

Task: redo WBVAL01 end-to-end for the current
`/wc1/runs/in/indispensable-presenter` run after verifying that rebuilt
WEPPpy climate artifacts are publication-safe for openWEPP.

Constraints:

- This is a validation/characterization package, not a production correction
  package.
- Do not edit Rust production code or canonical contracts from this package.
- Do not weaken `CLIM-RUNTIME-E-017`, `INV-CLIMATE-013`, or any source-bound
  radiation guard.
- Do not run openWEPP validation if the climate precondition audit finds any
  `rad > baseline sunmap.r3` in the current CLI artifact.
- Do not use comparator match or snow magnitude as acceptance.
- Preserve the snow/`RM` comparator route suspension behind
  `docs/backlog/20260605-snow-code-deferred-science-review.md`.
- Every artifact must label evidence truthfully as `Static:` or `Ran:`.
- Follow-ons must be defect-shaped; no handoff may name only a next diagnostic
  step.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:

- Update all queued artifacts through disposition.
- Record the climate-precondition audit before any openWEPP run.
- Produce a fresh run manifest, single-OFE closure ledger, WBVAL01 comparison,
  review disposition, verification artifacts, gate results, worker handoff, and
  final disposition.
