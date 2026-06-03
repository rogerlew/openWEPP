# HPHYS0266 Kickoff Agent Prompt

Scope: local repository science-contract/kernel diagnostic task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0266-layer-storage-lateral-snow-coupling-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/targeted-h1-h7-h39-first-ep-divergence-classification.md`

Files:

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0266-layer-storage-lateral-snow-coupling-closure-001/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs` only if evidence proves an in-scope production defect
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs` only if evidence proves an in-scope production defect
- `crates/openwepp-runner/src/hillslope/mod.rs` only if trace fields are contract-required
- `crates/openwepp-hillslope-orchestrator/src/tests.rs` only if production code changes

Task: execute package objective end-to-end for the declared scope. Diagnose
whether H1/H7/H39 first longer-season SWU stress residuals under closed
PMET/WB17 identities are owned by layer storage distribution, WB19 lateral
active-zone coupling, snow/runoff timing, or a proven baseline-authoritative
production defect.

Constraints: contract-first sequencing; canonical `SC-*` authority; pinned
baseline provenance at
`/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`;
typed guards; no silent defaults; no heuristic/proxy process-physics
substitutions; no production code edits before contract + diagnostic evidence
and pre-implementation contract gate completion.

Autonomy: execute package phases end-to-end and update required
artifacts/disposition without requesting additional user direction unless
hard-blocked.

Outputs: update package artifacts/disposition for all completed phases and
record full H1..H39 semantic metrics.
