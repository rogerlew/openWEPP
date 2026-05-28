Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv`
- `/workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`

Files:
- `docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/**` (only if defect remediation is required)

Task: execute HILLSTAB01 objective end-to-end by running broad hillslope
stability suites for release `openwepp-cli-hill` across the 1166-seed
`wepp-forest` cohort and the release-gate watchlist hillslopes, then publish
results and disposition.

Constraints:
- Contract-first sequencing is mandatory where authority changes are required:
  1. contracts/index, 2. contract-derived tests, 3. pre-implementation gate,
  4. production runtime edits.
- Do not modify kernel/runtime production code before contract+test+gate steps
  are complete when authority changes apply.
- Canonical `SC-*` contracts are implementation authority; package artifacts are
  evidence only.
- Preserve baseline provenance traceability to pinned baseline commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy physics substitutions in production runtime.
- No silent defaults/coercion/clamping for domain violations; use typed
  fail-closed guards.
- Complete dual review and dual verification artifacts before disposition.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including stability JSON/report outputs and gate summary.
