Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillbench01-hillslope-cli-release-benchmark-optimization-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`

Files:
- `docs/work-packages/20260528-hillbench01-hillslope-cli-release-benchmark-optimization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` (if required)
- `crates/openwepp-runner/src/hillslope/mod.rs` (if required)
- `crates/openwepp-runner/tests/**` (if required)
- `tests/integration/**` (if required)

Task: execute HILLBENCH01 objective end-to-end by benchmarking release-build
`openwepp-cli-hill` for single-OFE and multi-OFE lanes, comparing to baseline
`wepp_260430_hill`, and landing scoped hot-path optimizations with before/after
evidence.

Constraints:
- Contract-first sequencing is mandatory where contract authority changes are
  required:
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

Outputs: update package artifacts/disposition for all completed phases and run
required gates (`cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`).
