# WBVAL05 Kickoff Prompt

Scope: local repository science-contract/kernel defect-closure task;
flat-file reads/edits and local command execution only; no external
connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval05-j95-percolation-defect-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `/workdir/openWEPP/docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `/workdir/openWEPP/docs/backlog/20260605-snow-code-deferred-science-review.md`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval04-rocky-mountain-daymet-wbval01-redo-001/`

Files: use the intended write set declared in `package.md`.

Task: close defect `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` end-to-end for
`p7`, `p11`, `p18`, and `p20`.

Correction Authority Envelope: defect ID
`WBVAL05-J95-HKERNEL-WB11-PERC-E-003`; observed failure is
`HKERNEL-WB11-PERC-E-003` at `sim_day_index=95`, `calendar_year=1990`,
`julian_day=95` for `p7`, `p11`, `p18`, and `p20`; in-scope authority is
`SC-PERC-001` plus explicitly implicated `SC-WATBAL-001`,
`SC-SNOWFREEZE-001`, or `SC-RUNOFFPART-001`; allowed edits are
contract-first percolation/deep-seepage diagnostics, tests, and corrections in
the files declared by `package.md`; protected boundaries are WEPPpy,
`/wc1` inputs, annual WAT residual closure, snow-magnitude/comparator tuning,
guard loosening, and heuristic/proxy physics.

Constraints: contract-first sequencing; canonical `SC-*` authority; pinned
baseline provenance when legacy migration applies; typed guards; no silent
defaults; no guard loosening; no canonicalize-and-proceed for domain
violations; no heuristic/proxy process physics; no comparator-match acceptance;
do not relay intermediate diagnostic steps into a new package.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
