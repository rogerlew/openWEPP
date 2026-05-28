Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab05-slope-residual-family-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab02-parser-failure-remediation-and-stability-rerun-001/artifacts/hillstab02-rerun-delta-report.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab02-parser-failure-remediation-and-stability-rerun-001/artifacts/hillstab02_disposition.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv`
- `/workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv`

Files:
- `docs/work-packages/20260528-hillstab05-slope-residual-family-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` (if required)
- `tests/integration/infile_slope_parser_contract.rs` (if required)
- `tests/integration/parser_runtime_seam_integration.rs` (if required)
- `crates/openwepp-input-contract/src/parsers/slope.rs` (if required)
- `crates/openwepp-hillslope-orchestrator/**` (if required)
- `crates/openwepp-runner/**` (if required)

Task: execute HILLSTAB05 end-to-end by closing the residual slope failure
families (token parse, endpoint constraint, cross-OFE boundary mismatch, and
`HS-RUNTIME-E-023`) with contract-first sequencing, then rerun the broad
hillslope cohorts and publish delta/disposition artifacts.

Constraints:
- Contract-first sequencing is mandatory:
  1. contracts, 2. contract-derived tests, 3. pre-implementation gate,
  4. production code edits.
- Do not modify production code before contract + test + gate completion.
- Canonical `SC-*` contracts are authority; package artifacts are evidence.
- Preserve baseline provenance to pinned baseline commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions.
- No silent defaults/clamping for domain violations; use typed guards/errors.
- Complete dual review and dual verification artifacts before disposition.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- updated contract/test/runtime evidence artifacts
- gate results (`fmt`, `clippy`, `test`, `deny`)
- rerun JSON + delta report
- final GO/HOLD disposition and worker handoff

