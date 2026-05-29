Scope: local repository release-gate CI automation task; flat-file reads/edits
and local command execution only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-relproc03-release-gate-ci-automation-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-relproc02-runner-sidecar-emission-cli-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/governance/openwepp-release-procedure-draft.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/artifacts/hillstab01_stability_cohort.py`

Files:
- `.github/workflows/release-gates.yml`
- `tools/release/run_release_candidate_gates.sh`
- `tools/release/run_hillstab_gate.sh`
- `tools/release/assert_hillstab_success.py`
- `docs/governance/openwepp-release-procedure-draft.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-relproc03-release-gate-ci-automation-001/**`

Task: execute RELPROC03 end-to-end by implementing release-gate CI automation
for workspace checks, release lint, and stability cohort gates, then
disposition evidence.

Constraints:
- Keep scripts deterministic and fail-fast.
- Do not silently pass stability cohort failures; assert suite pass/fail from
  harness JSON.
- Do not require external path discovery in scripts; require explicit args/env
  for cohort data roots.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
