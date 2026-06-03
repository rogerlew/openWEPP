# HPHYS0268 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/audits/20260525_water_erosion_kernel_audit.md`
- `/workdir/openWEPP/docs/audits/20260603_wepp_forest_nonag_frost_disable_audit.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0267-post-lateral-pre-swu-threshold-lineage-closure-001/artifacts/review_claude_code.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0267-post-lateral-pre-swu-threshold-lineage-closure-001/artifacts/worker-handoff.md`

Files:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/**`
- Production kernel files only if evidence proves an in-scope defect.

Task: execute package objective end-to-end for the declared scope. Re-anchor
H1/H7/H39 on the first material spring `Ep` divergence (`>1 mm`), diagnose
snowpack/SWE/RM lineage, and only patch production snowpack physics if the
evidence proves a pinned-baseline defect.

Constraints: contract-first sequencing; canonical `SC-*` authority; pinned
baseline provenance at
`/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`;
typed guards; no silent defaults; no heuristic/proxy process-physics
substitutions; keep non-ag frost disabled for HPHYS baseline parity unless a
separate correctness decision changes the target.

Autonomy: execute package phases end-to-end and update required
artifacts/disposition without requesting additional user direction unless
hard-blocked.

Outputs: update package artifacts/disposition for all completed phases and
record full H1..H39 semantic metrics.
