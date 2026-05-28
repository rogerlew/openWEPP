Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab02-parser-failure-remediation-and-stability-rerun-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/artifacts/hillstab01-stability-report.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv`
- `/workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv`

Files:
- `docs/work-packages/20260528-hillstab02-parser-failure-remediation-and-stability-rerun-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md` (if required)
- `crates/openwepp-input-contract/src/parsers/soil/**` (if required)
- `crates/openwepp-input-contract/src/parsers/management/**` (if required)
- `crates/openwepp-runner/**` (if required)
- `tests/**` (if required)

Task: execute HILLSTAB02 objective end-to-end by remediating parser
compatibility failures (`SOL-E-006`, `MAN-E-009`) and rerunning workspace
quality gates and broad hillslope stability cohorts with delta reporting.

Constraints:
- Contract-first sequencing is mandatory:
  1. contracts, 2. contract-derived tests, 3. pre-implementation gate,
  4. production code edits.
- Do not modify production parser/runtime code before contract + test + gate
  completion.
- Canonical `SC-*` contracts are authority; package artifacts are evidence.
- Preserve baseline provenance to pinned baseline commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions.
- No silent defaults/clamping for domain violations; use typed guards/errors.
- Complete dual review and dual verification artifacts before disposition.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- Updated parser contract/test/code evidence artifacts
- Required gate results (`fmt`, `clippy`, `test`, `deny`)
- stability rerun JSON and delta report
- final GO/HOLD disposition with handoff
