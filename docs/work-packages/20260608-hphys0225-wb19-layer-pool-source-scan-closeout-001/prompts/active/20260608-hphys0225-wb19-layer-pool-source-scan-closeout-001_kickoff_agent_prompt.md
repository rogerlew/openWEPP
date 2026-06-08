Scope: local repository science-contract/kernel migration follow-on; flat-file reads/
 edits only; no external connectivity.

Execution mode: package-end-to-end.

Phase plan: follow the package.md phases through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260608-hphys0225-wb19-layer-pool-source-scan-closeout-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0225-wb19-available-pool-authority-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260608-refactor015-openwepp-hillslope-orchestrator-hydrology-kernel-phases-mechanical-modularization-001/package.md`
- `/workdir/openWEPP/tests/AGENTS.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260608-hphys0225-wb19-layer-pool-source-scan-closeout-001/**`
- `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs`

Task: execute the closeout package to make HPHYS0225 source scan resilient to the
refactored hydrology module layout.

Constraints:
- Preserve contract intent.
- Do not change production hydrology runtime behavior.
- Keep repository-only edits in-scope.

Autonomy: execute to disposition and update package artifacts without user intervention
unless hard-blocked.
