# WBVAL06 Kickoff Prompt

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
- `/workdir/openWEPP/docs/work-packages/20260606-wbval06-single-ofe-wat-conservation-residual-defect-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `/workdir/openWEPP/docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `/workdir/openWEPP/docs/backlog/20260605-snow-code-deferred-science-review.md`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval04-rocky-mountain-daymet-wbval01-redo-001/`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval05-j95-percolation-defect-closure-001/` if WBVAL05 has executed.

Files: use the intended write set declared in `package.md`.

Task: close defect `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL` end-to-end
for the current WBVAL04 WAT-emitter population.

Correction Authority Envelope: defect ID
`WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL`; observed failure is annual
complete-identity residual above `1.0 mm/year` for years `2..6` on the current
WBVAL04 WAT emitters, with max current residual `94.433 mm` on `p4`, year `5`;
in-scope authority is `SC-WATBAL-001` plus explicitly implicated
`SC-PERC-001`, `SC-SNOWFREEZE-001`, `SC-EVAP-001`, or
`SC-RUNOFFPART-001`; allowed edits are contract-first WAT publication,
storage/flux accounting, diagnostics, tests, and corrections in the files
declared by `package.md`; protected boundaries are WEPPpy, `/wc1` inputs,
WBVAL05 J-95 percolation closure, snow-magnitude/comparator tuning, silent
identity changes, and heuristic/proxy physics.

Constraints: contract-first sequencing; canonical `SC-*` authority; pinned
baseline provenance when legacy migration applies; typed guards; no silent
defaults; no guard loosening; no canonicalize-and-proceed for domain
violations; no heuristic/proxy process physics; no comparator-match acceptance;
do not relay intermediate diagnostic steps into a new package.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
